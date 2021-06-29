#![no_std]
#![no_main]

use esp32_hal::target;
use hal::prelude::*;
use xtensa_lx::timer::delay;
use panic_halt as _;
use esp32_hal as hal;

/// The default clock source is the onboard crystal
/// In most cases 40mhz (but can be as low as 2mhz depending on the board)
/// The ESP WROOM 32 I was testing with seems to run at 80MHz
const CORE_HZ: u32 = 80_000_000;

#[entry]
fn main() -> ! {
	let dp = target::Peripherals::take().expect("Failed to obtain Peripherals");

	let pins = dp.GPIO.split();
	let mut led = pins.gpio2.into_push_pull_output();

	loop {
		led.set_high().unwrap();
		delay(CORE_HZ / 2);
		led.set_low().unwrap();
		delay(CORE_HZ / 2);
	}
}
