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

//readability consts:
const HIGH: bool = true;
const LOW: bool = false;

/// The default clock source is the onboard crystal
/// In most cases 40mhz (but can be as low as 2mhz depending on the board)
/// The ESP WROOM 32 I was testing with seems to run at 80MHz
/// This is equivalent to 12.5ns per clock cycle.
const CORE_HZ: u32 = 80_000_000;
const CORE_PERIOD_NS:f32 = 12.5;

//Timing values for our 800kHz WS2811 Strips in nanoseconds:
const WS2811_0H_TIME_NS: u32 = 500;
const WS2811_0L_TIME_NS: u32 = 2000;
const WS2811_1H_TIME_NS: u32 = 1200;
const WS2811_1L_TIME_NS: u32 = 1300;

//Timing Values converted to equivalent clock cycle values:
const WS2811_0H_TIME_CLOCKS: u32 = (WS2811_0H_TIME_NS as f32 / CORE_PERIOD_NS) as u32;
const WS2811_0L_TIME_CLOCKS: u32 = (WS2811_0L_TIME_NS as f32 / CORE_PERIOD_NS) as u32;
const WS2811_1H_TIME_CLOCKS: u32 = (WS2811_1H_TIME_NS as f32 / CORE_PERIOD_NS) as u32;
const WS2811_1L_TIME_CLOCKS: u32 = (WS2811_1L_TIME_NS as f32 / CORE_PERIOD_NS) as u32;

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

const NUM_LEDS_WINDOW_STRIP: usize = 74;
const NUM_LEDS_DOOR_STRIP: usize = 61;
const NUM_LEDS_CLOSET_STRIP: usize = 34;

const NUM_LEDS: usize = get_total_num_leds(&ALL_STRIPS);

//these are to determine how many clocks to remove from the nominal timing values
//they were determined experimentally
const DELAY_OVERHEAD_CLOCKS: u32 = 12;
const SINGLE_OUTPUT_SET_OVERHEAD_CLOCKS: u32 = 4;
const NUM_OUTPUTS: u32 = 3;
const LED_FULL_CYCLE_CLOCKS: u32 = 200;

const fn get_total_num_leds(strips: &[WS2811PhysicalStrip]) -> usize {
	let mut index = 0;
	let mut total = 0;
	while index < strips.len() {
		total += strips[index].led_count;
		index += 1;
	}
	total
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

struct LogicalStrip<'a, const NUM_LEDS: usize> {
	buffer: [Color; NUM_LEDS],
	strips: &'a [WS2811PhysicalStrip],
}

impl <'a, const NUM_LEDS: usize> LogicalStrip<'a, NUM_LEDS> {
	fn new(strips: &'a [WS2811PhysicalStrip] ) -> Self {
		LogicalStrip::<NUM_LEDS> {
			buffer: [Color::default(); NUM_LEDS],
			strips
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

struct Pins<
	P1: OutputPin + Push,
	P2: OutputPin + Push,
	P3: OutputPin + Push,
> {
	p1: P1,
	p2: P2,
	p3: P3,
}

impl<P1: OutputPin + Push, P2: OutputPin + Push, P3: OutputPin + Push> Pins<P1, P2, P3> {
	fn pull_low(pin: u8, mut pins: Pins<P1, P2, P3>) -> Pins<P1, P2, P3> {
		match pin {
			CLOSET_STRIP_PIN => pins.p1.our_set_low(),
			WINDOW_STRIP_PIN => pins.p2.our_set_low(),
			DOOR_STRIP_PIN => pins.p3.our_set_low(),
			_ => (),
		};
		pins
	}

	fn push_high(pin: u8, mut pins: Pins<P1, P2, P3>) -> Pins<P1, P2, P3> {
		match pin {
			CLOSET_STRIP_PIN => pins.p1.our_set_high(),
			WINDOW_STRIP_PIN => pins.p2.our_set_high(),
			DOOR_STRIP_PIN => pins.p3.our_set_high(),
			_ => (),
		};
		pins
	}
}

trait Push {
	fn our_set_low(&mut self);
	fn our_set_high(&mut self);
}

fn delay_from_start(start_clocks: u32, clocks_to_delay: u32) {
	let target = start_clocks + clocks_to_delay;
	loop {
		if get_cycle_count() > target {
			break;
		}
	}
}

#[entry]
fn main() -> ! {
	//make the logical strip:
	let mut office_strip = LogicalStrip::<NUM_LEDS>::new(&ALL_STRIPS);

	let device_peripherals = target::Peripherals::take().expect("Failed to obtain Peripherals");

	let pins = device_peripherals.GPIO.split();

	//make sure the pin numbers here match the const pin numbers and macros above:
	let closet_led_control_gpio = pins.gpio33.into_push_pull_output();
	let window_led_control_gpio = pins.gpio23.into_push_pull_output();
	let door_led_control_gpio = pins.gpio25.into_push_pull_output();

	let mut pins = Pins {
		p1: closet_led_control_gpio,
		p2: window_led_control_gpio,
		p3: door_led_control_gpio,
	};

	loop {
		let start_time = get_cycle_count();
		for idx in 0..(NUM_LEDS as u32 * 8 * 3) {
			pins = Pins::push_high(ALL_STRIPS[1].pin, pins);
			let high_time = WS2811_0H_TIME_CLOCKS;
			let current_loop_delay = high_time - DELAY_OVERHEAD_CLOCKS - (SINGLE_OUTPUT_SET_OVERHEAD_CLOCKS * NUM_OUTPUTS);
			delay(current_loop_delay);
			pins = Pins::pull_low(ALL_STRIPS[1].pin, pins);
			delay_from_start(start_time, (idx + 1) * LED_FULL_CYCLE_CLOCKS);
		}
		delay(CORE_HZ);
	}
}
