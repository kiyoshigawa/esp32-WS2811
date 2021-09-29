#![no_std]
#![no_main]
#[macro_use]
#[allow(unused_imports)]

pub mod colors;
pub mod pins;

use esp32_hal::target;
use esp32_hal::gpio::{OutputPin, PushPull, Output};
use hal::prelude::*;
use xtensa_lx::timer::{delay, get_cycle_count};
use panic_halt as _;
use esp32_hal as hal;
use crate::colors::colors as c;
use crate::pins::pins as p;

//macro to add Push trait to gpio pins:
//this wraps the pins' set_high() and set_low() functions in our_set_* wrappers.
macro_rules! push {
	($p:ty) => {
		impl p::Push for $p {
			fn our_set_low(&mut self) {
				self.set_low().unwrap();
			}
			fn our_set_high(&mut self) {
				self.set_high().unwrap();
			}
		}
	};
}

//make sure to add the pins you're using here and in pins.rs:
push!(esp32_hal::gpio::Gpio25<Output<PushPull>>);
push!(esp32_hal::gpio::Gpio13<Output<PushPull>>);
push!(esp32_hal::gpio::Gpio33<Output<PushPull>>);

//readability consts:
const ONE: bool = true;
const ZERO: bool = false;

// The default clock source is the onboard crystal
// In most cases 40mhz (but can be as low as 2mhz depending on the board)
// The ESP WROOM 32 I was testing with seems to run at 80MHz
// This is equivalent to 12.5ns per clock cycle.
const CORE_HZ: u32 = 80_000_000;
const CORE_PERIOD_NS:f32 = 12.5;

//Timing values for our 800kHz WS2811 Strips in nanoseconds:
const WS2811_0H_TIME_NS: u32 = 350;
const WS2811_1H_TIME_NS: u32 = 1200;
const WS2811_FULL_CYCLE_TIME_NS: u32 = 2500;

//Timing Values converted to equivalent clock cycle values:
const WS2811_0H_TIME_CLOCKS: u32 = (WS2811_0H_TIME_NS as f32 / CORE_PERIOD_NS) as u32;
const WS2811_1H_TIME_CLOCKS: u32 = (WS2811_1H_TIME_NS as f32 / CORE_PERIOD_NS) as u32;
const WS2811_FULL_CYCLE_CLOCKS: u32 = (WS2811_FULL_CYCLE_TIME_NS as f32 / CORE_PERIOD_NS) as u32;

//This is how much to offset from the clock cycle measurement before actually sending data to the strips
//the value was determined experimentally, tweak as needed for consistency
const SEND_START_OFFSET_DELAY_CLOCKS: u32 = 30000;

//the number of LEDs on each strip:
const NUM_LEDS_WINDOW_STRIP: usize = 74;
const NUM_LEDS_DOOR_STRIP: usize = 61;
const NUM_LEDS_CLOSET_STRIP: usize = 34;
const MAX_SINGLE_STRIP_BYTE_BUFFER_LENGTH: usize = get_single_strip_buffer_max_length(&ALL_STRIPS);
const MAX_SINGLE_STRIP_BIT_BUFFER_LENGTH: usize = MAX_SINGLE_STRIP_BYTE_BUFFER_LENGTH * 8;

//individual strips:
const CLOSET_STRIP: WS2811PhysicalStrip =
	WS2811PhysicalStrip {
		pin: p::CLOSET_STRIP_PIN,
		led_count: NUM_LEDS_CLOSET_STRIP,
		reversed: false,
		_color_order: ColorOrder::BRG,
	};
const WINDOW_STRIP: WS2811PhysicalStrip =
	WS2811PhysicalStrip {
		pin: p::WINDOW_STRIP_PIN,
		led_count: NUM_LEDS_WINDOW_STRIP,
		reversed: false,
		_color_order: ColorOrder::BRG,
	};
const DOOR_STRIP: WS2811PhysicalStrip =
	WS2811PhysicalStrip {
		pin: p::DOOR_STRIP_PIN,
		led_count: NUM_LEDS_DOOR_STRIP,
		reversed: true,
		_color_order: ColorOrder::BRG,
	};

//combined strip group:
const ALL_STRIPS: [WS2811PhysicalStrip; 3] = [
	CLOSET_STRIP,
	WINDOW_STRIP,
	DOOR_STRIP,
];

//calculate the total number of LEDs from the above values:
const NUM_LEDS: usize = get_total_num_leds(&ALL_STRIPS);

#[allow(dead_code)]
enum ColorOrder {
	RGB,
	RBG,
	GRB,
	GBR,
	BRG,
	BGR,
}

const fn get_total_num_leds(strips: &[WS2811PhysicalStrip]) -> usize {
	let mut index = 0;
	let mut total = 0;
	while index < strips.len() {
		total += strips[index].led_count;
		index += 1;
	}
	total
}

const fn get_single_strip_buffer_max_length(strips: &[WS2811PhysicalStrip]) -> usize {
	let mut max_len = 0;
	let mut index = 0;
	while index < strips.len() {
		if strips[index].led_count > max_len {
			max_len = strips[index].led_count;
		}
		index += 1;
	}
	// three bytes per led
	max_len * 3
}

struct WS2811PhysicalStrip {
	pin: u8,
	led_count: usize,
	reversed: bool,
	_color_order: ColorOrder,
}

impl WS2811PhysicalStrip {
	fn send_bits<P1, P2, P3> (&self, pins: &mut p::PinControl<P1, P2, P3>, timings: &[(u32, u32)] )
	where P1: OutputPin + p::Push,
		  P2: OutputPin + p::Push,
		  P3: OutputPin + p::Push,
	{
		for timing in timings {
			delay_until(timing.0);
			p::PinControl::push_high(self.pin, pins);
			delay_until(timing.1);
			p::PinControl::pull_low(self.pin, pins);
		}
	}
}

struct LogicalStrip<'a, const NUM_LEDS: usize> {
	color_buffer: [c::Color; NUM_LEDS],
	strips: &'a [WS2811PhysicalStrip],
}

impl<'a, const NUM_LEDS: usize> LogicalStrip<'a, NUM_LEDS> {
	fn new(strips: &'a [WS2811PhysicalStrip] ) -> Self {
		LogicalStrip::<NUM_LEDS> {
			color_buffer: [c::Color::default(); NUM_LEDS],
			strips,
		}
	}

	//this sets the color value in the color array at index:
	fn set_color_at_index(&mut self, index: usize, color: c::Color) {
		self.color_buffer[index].r = c::GAMMA8[color.r as usize];
		self.color_buffer[index].g = c::GAMMA8[color.g as usize];
		self.color_buffer[index].b = c::GAMMA8[color.b as usize];
	}

	//this fills the entire strip with a single color:
	fn set_strip_to_solid_color(&mut self, color: c::Color) {
		for i in 0..self.color_buffer.len() {
			self.set_color_at_index(i, color);
		}
	}

	//this will iterate over all the strips and send the led data in series:
	fn send_all_sequential<P1, P2, P3> ( &self, pins: &mut p::PinControl<P1, P2, P3>)
	where P1: OutputPin + p::Push,
		  P2: OutputPin + p::Push,
		  P3: OutputPin + p::Push,
	{
		let mut start_index = 0;

		for strip in self.strips {
			let end_index = start_index + strip.led_count;

			// generate byte array from color array (taking care of color order)
			let current_strip_colors = &self.color_buffer[start_index..end_index];
			let byte_count = strip.led_count * 3;
			let bit_count = byte_count * 8;
			let mut byte_buffer = [0_u8; MAX_SINGLE_STRIP_BYTE_BUFFER_LENGTH];
			if strip.reversed {
				for (i, color) in current_strip_colors.iter().rev().enumerate() {
					let base = i * 3;
					byte_buffer[base + 0] = color.b;
					byte_buffer[base + 1] = color.r;
					byte_buffer[base + 2] = color.g;
				}
			} else {
				for (i, color) in current_strip_colors.iter().enumerate() {
					let base = i * 3;
					byte_buffer[base + 0] = color.b;
					byte_buffer[base + 1] = color.r;
					byte_buffer[base + 2] = color.g;
				}
			}

			// from byte array to bit array
			let mut bit_buffer = [ZERO; MAX_SINGLE_STRIP_BIT_BUFFER_LENGTH];
			for (i, byte) in byte_buffer.iter().take(byte_count).enumerate() {
				let base = i * 8;
				for bit in 0..8_u8 {
					bit_buffer[base + bit as usize] = match (byte >> bit) & 0x01 {
						0x01 => ONE,
						0x00 => ZERO,
						_ => unreachable!(),
					};
				}
			}

			// from bit array to timing array
			let mut timings = [(0_u32,0_u32); MAX_SINGLE_STRIP_BIT_BUFFER_LENGTH];
			for (i, &bit) in bit_buffer.iter().take(bit_count).enumerate() {
				let bit_timing = match bit {
					ONE => WS2811_1H_TIME_CLOCKS,
					ZERO => WS2811_0H_TIME_CLOCKS,
				};
				let base_time = WS2811_FULL_CYCLE_CLOCKS * i as u32;
				timings[i] = (base_time, base_time + bit_timing);
			}

			// add clock + offset to timing array
			let offset_clocks = SEND_START_OFFSET_DELAY_CLOCKS;
			let clock_and_offset = get_cycle_count() + offset_clocks;
			for i in 0..timings.len() {
				timings[i].0 = timings[i].0 + clock_and_offset;
				timings[i].1 = timings[i].1 + clock_and_offset;
			}

			// call send bits and send the timing array
			strip.send_bits(pins, &timings);

			start_index = end_index;
		}
	}
}

//this is a delay function that will prevent progress to a specified number of
//clock cycles as measured by the get_cycle_count() function.
fn delay_until(clocks: u32) {
	loop {
		if get_cycle_count() > clocks {
			break;
		}
	}
}

#[entry]
fn main() -> ! {
	//make the logical strip:
	let mut office_strip = LogicalStrip::<NUM_LEDS>::new(&ALL_STRIPS);

	//get physical pins to a usable state:
	let device_peripherals = target::Peripherals::take().expect("Failed to obtain Peripherals");
	let peripheral_pins = device_peripherals.GPIO.split();
	//make sure the pin numbers here match the const pin numbers and macros above and in pins.rs:
	let closet_led_control_gpio = peripheral_pins.gpio33.into_push_pull_output();
	let window_led_control_gpio = peripheral_pins.gpio13.into_push_pull_output();
	let door_led_control_gpio = peripheral_pins.gpio25.into_push_pull_output();
	let mut pins = p::PinControl {
		p1: closet_led_control_gpio,
		p2: window_led_control_gpio,
		p3: door_led_control_gpio,
	};

	office_strip.set_strip_to_solid_color(c::C_WHITE);

	loop {
		office_strip.send_all_sequential(&mut pins);
		delay(CORE_HZ);
	}
}
