#![no_std]
#![no_main]

use esp32_hal::target;
use hal::prelude::*;
use xtensa_lx::timer::{delay, get_cycle_count};
use panic_halt as _;
use esp32_hal as hal;

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
//TODO: Figure out timing shit.
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

//hardware specific config for tim's closet:
const WINDOW_STRIP_PIN: u8 = 13;
const DOOR_STRIP_PIN: u8 = 25;
const CLOSET_STRIP_PIN: u8 = 33;

const NUM_LEDS_WINDOW_STRIP: u8 = 74;
const NUM_LEDS_DOOR_STRIP: u8 = 61;
const NUM_LEDS_CLOSET_STRIP: u8 = 34;

const WINDOW_STRIP_FIRST_LED_INDEX: u8 = 0;
const DOOR_STRIP_FIRST_LED_INDEX: u8 = WINDOW_STRIP_FIRST_LED_INDEX + NUM_LEDS_WINDOW_STRIP;
const CLOSET_STRIP_FIRST_LED_INDEX: u8 = DOOR_STRIP_FIRST_LED_INDEX + NUM_LEDS_DOOR_STRIP;
const NUM_LEDS: u8 = CLOSET_STRIP_FIRST_LED_INDEX + NUM_LEDS_CLOSET_STRIP;

const DELAY_OVERHEAD_CLOCKS: u32 = 12;
const SINGLE_OUTPUT_SET_OVERHEAD: u32 = 4;
const NUM_OUTPUTS: u32 = 3;
const LED_FULL_CYCLE_TIME: u32 = 200;


/// GPIO output enable reg
const GPIO_ENABLE_W1TS_REG: u32 = 0x3FF44024;

/// GPIO output set register
const GPIO_OUT_W1TS_REG: u32 = 0x3FF44008;
/// GPIO output clear register
const GPIO_OUT_W1TC_REG : u32 = 0x3FF4400C;

/// The GPIO hooked up to the onboard LED
const BLINKY_GPIO: u32 = 23;

/// GPIO function mode
const GPIO_FUNCX_OUT_BASE: u32 = 0x3FF44530;
const GPIO_FUNCX_OUT_SEL_CFG: u32 = GPIO_FUNCX_OUT_BASE + (BLINKY_GPIO * 4);

#[derive(Copy, Clone)]
struct Color {
	r: u8,
	g: u8,
	b: u8,
}

fn delay_from_start(start_clocks: u32, clocks_to_delay: u32) {
	let target = start_clocks + clocks_to_delay;
	loop {
		if get_cycle_count() > target {
			break;
		}
	}
}

// fn send_bit(state: bool, start_time: u32, idx: u32) {
// 	window_led_control_pin.set_high();
// 	door_led_control_pin.set_high();
// 	closet_led_control_pin.set_high();
// 	let high_time = if state == HIGH {WS2811_1H_TIME_CLOCKS} else {WS2811_0H_TIME_CLOCKS};
// 	let current_loop_delay = high_time - DELAY_OVERHEAD_CLOCKS - (SINGLE_OUTPUT_SET_OVERHEAD * NUM_OUTPUTS);
// 	delay(current_loop_delay);
// 	window_led_control_pin.set_low();
// 	door_led_control_pin.set_low();
// 	closet_led_control_pin.set_low();
// 	delay_from_start(start_time, (idx + 1) * LED_FULL_CYCLE_TIME);
// }

#[entry]
fn main() -> ! {

	//an array that stores the current color of all LEDs:
	let mut led_colors: [Color; NUM_LEDS as usize] = [Color {
		r: 255,
		g: 127,
		b: 0,
	}; NUM_LEDS as usize];

	let device_peripherals = target::Peripherals::take().expect("Failed to obtain Peripherals");

	let pins = device_peripherals.GPIO.split();
	let mut window_led_control_pin = pins.gpio23.into_push_pull_output();
	let mut door_led_control_pin = pins.gpio25.into_push_pull_output();
	let mut closet_led_control_pin = pins.gpio33.into_push_pull_output();

	loop {
		let start_time = get_cycle_count();
		for idx in 0..(NUM_LEDS as u32 * 8 * 3) {
			window_led_control_pin.set_high();
			door_led_control_pin.set_high();
			closet_led_control_pin.set_high();
			// let high_time = if state == HIGH {WS2811_1H_TIME_CLOCKS} else {WS2811_0H_TIME_CLOCKS};
			let high_time = WS2811_1H_TIME_CLOCKS;
			let current_loop_delay = high_time - DELAY_OVERHEAD_CLOCKS - (SINGLE_OUTPUT_SET_OVERHEAD * NUM_OUTPUTS);
			delay(current_loop_delay);
			window_led_control_pin.set_low();
			door_led_control_pin.set_low();
			closet_led_control_pin.set_low();
			delay_from_start(start_time, (idx + 1) * LED_FULL_CYCLE_TIME);
		}
		delay(CORE_HZ);
	}
}
