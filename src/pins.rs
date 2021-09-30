pub mod pins {
	#[allow(unused_imports)]
	use esp32_hal::gpio::{OutputPin, PushPull, Output};

	//hardware specific config for tim's office.
	//make sure to add the pins you're using here and in main.rs:
	pub const CLOSET_STRIP_PIN: u8 = 33;
	pub const WINDOW_STRIP_PIN: u8 = 13;
	pub const DOOR_STRIP_PIN: u8 = 25;

	//struct to hold the actual pins.
	//all pins must be of type OutputPin with a Push trait. The push trait allows
	//them to be used with set_low() and set_high() even though they are
	//technically different types.
	pub struct PinControl<
		P1: OutputPin + Push,
		P2: OutputPin + Push,
		P3: OutputPin + Push,
	> {
		pub p1: P1,
		pub p2: P2,
		pub p3: P3,
	}

	impl<P1, P2, P3> PinControl<P1, P2, P3>
		where P1: OutputPin + Push,
			  P2: OutputPin + Push,
			  P3: OutputPin + Push,
	{
		//this allows us to use the pin number in a match statement to call the set_low() function.
		pub fn pull_low(pin: u8, pins: &mut PinControl<P1, P2, P3>) {
			match pin {
				CLOSET_STRIP_PIN => pins.p1.our_set_low(),
				WINDOW_STRIP_PIN => pins.p2.our_set_low(),
				DOOR_STRIP_PIN => pins.p3.our_set_low(),
				_ => unreachable!(),
			}
		}
		//this allows us to use the pin number in a match statement to call the set_high() function.
		pub fn push_high(pin: u8, pins: &mut PinControl<P1, P2, P3>) {
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
	pub trait Push {
		fn our_set_low(&mut self);
		fn our_set_high(&mut self);
	}
}
