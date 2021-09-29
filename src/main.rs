#![no_std]
#![no_main]

use esp32_hal::target;
use esp32_hal::gpio::{OutputPin, PushPull, Output};
use hal::prelude::*;
use xtensa_lx::timer::{delay, get_cycle_count};
use panic_halt as _;
use esp32_hal as hal;
use crate::ColorOrder::RGB;

//macro to add Push trait to gpio pins:
//this wraps the pins' set_high() and set_low() functions in our_set_* wrappers.
macro_rules! push {
	($p:ty) => {
		impl Push for $p {
			fn our_set_low(&mut self) {
				self.set_low().unwrap();
			}
			fn our_set_high(&mut self) {
				self.set_high().unwrap();
			}
		}
	};
}

//struct to hold the actual pins and info about sending timing/position.
//all pins must be of type OutputPin with a Push trait. The push trait allows
//them to be used with set_low() and set_high() even though they are
//technically different types.
struct PinControl<
	P1: OutputPin + Push,
	P2: OutputPin + Push,
	P3: OutputPin + Push,
> {
	p1: P1,
	p2: P2,
	p3: P3,
	// send_all_start_cycle_count: u32,
	// send_all_current_bit: u32,
}

impl<P1, P2, P3> PinControl<P1, P2, P3>
where P1: OutputPin + Push,
	  P2: OutputPin + Push,
	  P3: OutputPin + Push,
{
	//this allows us to use the pin number in a match statement to call the set_low() function.
	fn pull_low(pin: u8, pins: &mut PinControl<P1, P2, P3>) {
		match pin {
			CLOSET_STRIP_PIN => pins.p1.our_set_low(),
			WINDOW_STRIP_PIN => pins.p2.our_set_low(),
			DOOR_STRIP_PIN => pins.p3.our_set_low(),
			_ => unreachable!(),
		}
	}
	//this allows us to use the pin number in a match statement to call the set_high() function.
	fn push_high(pin: u8, pins: &mut PinControl<P1, P2, P3>) {
		match pin {
			CLOSET_STRIP_PIN => pins.p1.our_set_high(),
			WINDOW_STRIP_PIN => pins.p2.our_set_high(),
			DOOR_STRIP_PIN => pins.p3.our_set_high(),
			_ => unreachable!(),
		}
	}
}

//the Push trait uses these wrapper functions to access the .set_low() and
// .set_high() functions on the pins
trait Push {
	fn our_set_low(&mut self);
	fn our_set_high(&mut self);
}

//readability consts:
const HIGH: bool = true;
const LOW: bool = false;

// The default clock source is the onboard crystal
// In most cases 40mhz (but can be as low as 2mhz depending on the board)
// The ESP WROOM 32 I was testing with seems to run at 80MHz
// This is equivalent to 12.5ns per clock cycle.
const CORE_HZ: u32 = 80_000_000;
const CORE_PERIOD_NS:f32 = 12.5;

//Timing values for our 800kHz WS2811 Strips in nanoseconds:
const WS2811_0H_TIME_NS: u32 = 500;
const WS2811_0L_TIME_NS: u32 = 2000;
const WS2811_1H_TIME_NS: u32 = 1200;
const WS2811_1L_TIME_NS: u32 = 1300;
const WS2811_FULL_CYCLE_TIME_NS: u32 = 2500;

//Timing Values converted to equivalent clock cycle values:
const WS2811_0H_TIME_CLOCKS: u32 = (WS2811_0H_TIME_NS as f32 / CORE_PERIOD_NS) as u32;
const WS2811_0L_TIME_CLOCKS: u32 = (WS2811_0L_TIME_NS as f32 / CORE_PERIOD_NS) as u32;
const WS2811_1H_TIME_CLOCKS: u32 = (WS2811_1H_TIME_NS as f32 / CORE_PERIOD_NS) as u32;
const WS2811_1L_TIME_CLOCKS: u32 = (WS2811_1L_TIME_NS as f32 / CORE_PERIOD_NS) as u32;
const WS2811_FULL_CYCLE_CLOCKS: u32 = (WS2811_FULL_CYCLE_TIME_NS as f32 / CORE_PERIOD_NS) as u32;

//these are to determine how many clocks to remove from the nominal timing values
//the values below were determined experimentally, tweak as needed for consistency
const DELAY_OVERHEAD_CLOCKS: u32 = 6;
const SINGLE_OUTPUT_SET_OVERHEAD_CLOCKS: u32 = 4;
const NUM_OUTPUTS: u32 = 3;

//a color correction table for LEDs to make them look like the color you expect:
//shamelessly stolen from Adafruit somewhere a long time ago.
const GAMMA8: [u8; 256] = [
	0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
	0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  1,  1,  1,  1,
	1,  1,  1,  1,  1,  1,  1,  1,  1,  2,  2,  2,  2,  2,  2,  2,
	2,  3,  3,  3,  3,  3,  3,  3,  4,  4,  4,  4,  4,  5,  5,  5,
	5,  6,  6,  6,  6,  7,  7,  7,  7,  8,  8,  8,  9,  9,  9, 10,
	10, 10, 11, 11, 11, 12, 12, 13, 13, 13, 14, 14, 15, 15, 16, 16,
	17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22, 23, 24, 24, 25,
	25, 26, 27, 27, 28, 29, 29, 30, 31, 32, 32, 33, 34, 35, 35, 36,
	37, 38, 39, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 50,
	51, 52, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 66, 67, 68,
	69, 70, 72, 73, 74, 75, 77, 78, 79, 81, 82, 83, 85, 86, 87, 89,
	90, 92, 93, 95, 96, 98, 99,101,102,104,105,107,109,110,112,114,
	115,117,119,120,122,124,126,127,129,131,133,135,137,138,140,142,
	144,146,148,150,152,154,156,158,160,162,164,167,169,171,173,175,
	177,180,182,184,186,189,191,193,196,198,200,203,205,208,210,213,
	215,218,220,223,225,228,231,233,236,239,241,244,247,249,252,255
];

//hardware specific config for tim's office:
const WINDOW_STRIP_PIN: u8 = 23;
const DOOR_STRIP_PIN: u8 = 25;
const CLOSET_STRIP_PIN: u8 = 33;

//make sure to add the pins you're using here:
push!(esp32_hal::gpio::Gpio23<Output<PushPull>>);
push!(esp32_hal::gpio::Gpio25<Output<PushPull>>);
push!(esp32_hal::gpio::Gpio33<Output<PushPull>>);

//the number of LEDs on each strip:
const NUM_LEDS_WINDOW_STRIP: usize = 74;
const NUM_LEDS_DOOR_STRIP: usize = 61;
const NUM_LEDS_CLOSET_STRIP: usize = 34;
const MAX_SINGLE_STRIP_BYTE_BUFFER_LENGTH: usize = get_single_strip_buffer_max_length(&ALL_STRIPS);
const MAX_SINGLE_STRIP_BIT_BUFFER_LENGTH: usize = MAX_SINGLE_STRIP_BYTE_BUFFER_LENGTH * 8;

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

#[derive(Default, Copy, Clone)]
struct Color {
	r: u8,
	g: u8,
	b: u8,
}

enum ColorOrder {
	RGB,
	RBG,
	GRB,
	GBR,
	BRG,
	BGR,
}

struct WS2811PhysicalStrip {
	pin: u8,
	led_count: usize,
	reversed: bool,
	color_order: ColorOrder,
}

impl WS2811PhysicalStrip {
	fn send_bits<P1, P2, P3> ( &self, pins: &mut PinControl<P1, P2, P3>, timings: &[(u32, u32)] )
	where P1: OutputPin + Push,
		  P2: OutputPin + Push,
		  P3: OutputPin + Push,
	{
		for timing in timings {
			delay_until(timing.0);
			PinControl::push_high(self.pin, pins);
			delay_until(timing.1);
			PinControl::pull_low(self.pin, pins);
		}
	}
}

struct LogicalStrip<'a, const NUM_LEDS: usize> {
	color_buffer: [Color; NUM_LEDS],
	strips: &'a [WS2811PhysicalStrip],
}

impl<'a, const NUM_LEDS: usize> LogicalStrip<'a, NUM_LEDS> {
	fn new(strips: &'a [WS2811PhysicalStrip] ) -> Self {
		LogicalStrip::<NUM_LEDS> {
			color_buffer: [Color::default(); NUM_LEDS],
			strips,
		}
	}
	//this will iterate over all the strips and send the led data in series:
	fn send_all_sequential<P1, P2, P3> ( &self, pins: &mut PinControl<P1, P2, P3>)
	where P1: OutputPin + Push,
		  P2: OutputPin + Push,
		  P3: OutputPin + Push,
	{
		let mut start_index = 0;

		for strip in self.strips {
			let end_index = start_index + strip.led_count;
			// generate byte array from color array (taking care of color order)
			let mut current_strip_colors = &self.color_buffer[start_index..end_index];
			let byte_count = strip.led_count * 3;
			let bit_count = byte_count * 8;

			let mut byte_buffer = [0_u8; MAX_SINGLE_STRIP_BYTE_BUFFER_LENGTH];
			if strip.reversed {
				for (i, color) in current_strip_colors.iter().rev().enumerate() {
					let base = i * 3;
					byte_buffer[base + 0] = color.r;
					byte_buffer[base + 1] = color.g;
					byte_buffer[base + 2] = color.b;
				}
			} else {
				for (i, color) in current_strip_colors.iter().enumerate() {
					let base = i * 3;
					byte_buffer[base + 0] = color.r;
					byte_buffer[base + 1] = color.g;
					byte_buffer[base + 2] = color.b;
				}
			}

			let mut bit_buffer = [LOW; MAX_SINGLE_STRIP_BIT_BUFFER_LENGTH];
			// from byte array to bit array
			for (i, byte) in byte_buffer.iter().take(byte_count).enumerate() {
				let base = i * 8;
				for bit in 0..8_u8 {
					bit_buffer[base + bit as usize] = match (byte >> bit) & 0x01 {
						0x01 => HIGH,
						0x00 => LOW,
						_ => unreachable!(),
					};
				}
			}
			// from bit array to timing array
			let mut timings = [(0_u32,0_u32); MAX_SINGLE_STRIP_BIT_BUFFER_LENGTH];
			for (i, &bit) in bit_buffer.iter().take(bit_count).enumerate() {
				let bit_timing = match bit {
					HIGH => WS2811_1H_TIME_CLOCKS,
					LOW => WS2811_0H_TIME_CLOCKS,
					_ => unreachable!(),
				};
				let base_time = WS2811_FULL_CYCLE_CLOCKS * i as u32;
				timings[i] = (base_time, base_time + bit_timing);
			}

			// add clock + offset to timing array
			let offset_clocks = 2000;
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

//individual strips:
const CLOSET_STRIP: WS2811PhysicalStrip =
	WS2811PhysicalStrip {
		pin: CLOSET_STRIP_PIN,
		led_count: NUM_LEDS_CLOSET_STRIP,
		reversed: false,
		color_order: RGB
	};
const WINDOW_STRIP: WS2811PhysicalStrip =
	WS2811PhysicalStrip {
		pin: WINDOW_STRIP_PIN,
		led_count: NUM_LEDS_WINDOW_STRIP,
		reversed: false,
		color_order: RGB
	};
const DOOR_STRIP: WS2811PhysicalStrip =
	WS2811PhysicalStrip {
		pin: DOOR_STRIP_PIN,
		led_count: NUM_LEDS_DOOR_STRIP,
		reversed: true,
		color_order: RGB
	};

//combined strip group:
const ALL_STRIPS: [WS2811PhysicalStrip; 3] = [
	CLOSET_STRIP,
	WINDOW_STRIP,
	DOOR_STRIP,
];

//calculate the total number of LEDs from the above values:
const NUM_LEDS: usize = get_total_num_leds(&ALL_STRIPS);

//this is a delay function that will prevent progress to a specified number of
//clock cycles from a specified start_clocks value.
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

	let device_peripherals = target::Peripherals::take().expect("Failed to obtain Peripherals");

	let peripheral_pins = device_peripherals.GPIO.split();

	//make sure the pin numbers here match the const pin numbers and macros above:
	let closet_led_control_gpio = peripheral_pins.gpio33.into_push_pull_output();
	let window_led_control_gpio = peripheral_pins.gpio23.into_push_pull_output();
	let door_led_control_gpio = peripheral_pins.gpio25.into_push_pull_output();

	let mut pins = PinControl {
		p1: closet_led_control_gpio,
		p2: window_led_control_gpio,
		p3: door_led_control_gpio,
	};

	loop {
		office_strip.send_all_sequential(&mut pins);
		delay(CORE_HZ);
	}
}
