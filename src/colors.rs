pub mod colors {
	#[derive(Default, Copy, Clone)]
	pub struct Color {
		pub r: u8,
		pub g: u8,
		pub b: u8,
	}

	impl Color {
		//new color object takes rgb color values:
		pub fn new(r: u8, g: u8, b: u8) -> Color {
			Color {
				r: GAMMA8[r as usize],
				g: GAMMA8[g as usize],
				b: GAMMA8[b as usize],
			}
		}

		//change RGB color values for mutable color
		pub fn set_rgb(&mut self, r: u8, g: u8, b: u8) {
			self.r = GAMMA8[r as usize];
			self.g = GAMMA8[g as usize];
			self.b = GAMMA8[b as usize];
		}
	}

	//a color correction table for LEDs to make them look like the color you expect:
	//shamelessly stolen from Adafruit somewhere a long time ago.
	pub const GAMMA8: [u8; 256] = [
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,
		1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2,
		2, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5,
		5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 9, 9, 9, 10,
		10, 10, 11, 11, 11, 12, 12, 13, 13, 13, 14, 14, 15, 15, 16, 16,
		17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22, 23, 24, 24, 25,
		25, 26, 27, 27, 28, 29, 29, 30, 31, 32, 32, 33, 34, 35, 35, 36,
		37, 38, 39, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 50,
		51, 52, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 66, 67, 68,
		69, 70, 72, 73, 74, 75, 77, 78, 79, 81, 82, 83, 85, 86, 87, 89,
		90, 92, 93, 95, 96, 98, 99, 101, 102, 104, 105, 107, 109, 110, 112, 114,
		115, 117, 119, 120, 122, 124, 126, 127, 129, 131, 133, 135, 137, 138, 140, 142,
		144, 146, 148, 150, 152, 154, 156, 158, 160, 162, 164, 167, 169, 171, 173, 175,
		177, 180, 182, 184, 186, 189, 191, 193, 196, 198, 200, 203, 205, 208, 210, 213,
		215, 218, 220, 223, 225, 228, 231, 233, 236, 239, 241, 244, 247, 249, 252, 255
	];

	//generic colors:
	#[allow(dead_code)]
	pub const RED: Color = Color { r: GAMMA8[255], g: GAMMA8[0], b: GAMMA8[0] };
	pub const ORANGE: Color = Color { r: GAMMA8[255], g: GAMMA8[127], b: GAMMA8[0] };
	pub const YELLOW: Color = Color { r: GAMMA8[255], g: GAMMA8[255], b: GAMMA8[0] };
	pub const YELLOW_GREEN: Color = Color { r: GAMMA8[127], g: GAMMA8[255], b: GAMMA8[0] };
	pub const GREEN: Color = Color { r: GAMMA8[0], g: GAMMA8[255], b: GAMMA8[0] };
	pub const GREEN_BLUE: Color = Color { r: GAMMA8[0], g: GAMMA8[255], b: GAMMA8[127] };
	pub const SKY_BLUE: Color = Color { r: GAMMA8[0], g: GAMMA8[255], b: GAMMA8[255] };
	pub const DEEP_BLUE: Color = Color { r: GAMMA8[0], g: GAMMA8[127], b: GAMMA8[255] };
	pub const BLUE: Color = Color { r: GAMMA8[0], g: GAMMA8[0], b: GAMMA8[255] };
	pub const BLUE_PURPLE: Color = Color { r: GAMMA8[127], g: GAMMA8[0], b: GAMMA8[255] };
	pub const PURPLE: Color = Color { r: GAMMA8[255], g: GAMMA8[0], b: GAMMA8[255] };
	pub const DARK_PURPLE: Color = Color { r: GAMMA8[255], g: GAMMA8[0], b: GAMMA8[127] };
	pub const WHITE: Color = Color { r: GAMMA8[255], g: GAMMA8[255], b: GAMMA8[127] };
	pub const OFF: Color = Color { r: GAMMA8[0], g: GAMMA8[0], b: GAMMA8[0] };
	pub const T_3000K: Color = Color { r: GAMMA8[255], g: GAMMA8[180], b: GAMMA8[107] };
	pub const T_3500K: Color = Color { r: GAMMA8[255], g: GAMMA8[196], b: GAMMA8[137] };
	pub const T_4000K: Color = Color { r: GAMMA8[255], g: GAMMA8[209], b: GAMMA8[163] };
	pub const T_5000K: Color = Color { r: GAMMA8[255], g: GAMMA8[228], b: GAMMA8[206] };
}
