pub mod colors {
	#[allow(dead_code)]

	pub const MAX_COLORS_IN_RAINBOW: usize = 10;

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

		//this maps a color to a fractional mid-color based on the position of the factor between the in_min and in_max values.
		//It will automatically truncate any values below 0 or larger than 255 when it is cast back to a u8 at the end of the calculation.
		pub fn color_map(factor: i32, in_min: i32, in_max: i32, start_color: Color, end_color: Color) -> Color {
			let mut mid_color = C_OFF;
			mid_color.r = ((factor - in_min) * (end_color.r as i32 - start_color.r as i32) / (in_max - in_min) + start_color.r) as u8;
			mid_color.g = ((factor - in_min) * (end_color.g as i32 - start_color.r as i32) / (in_max - in_min) + start_color.g) as u8;
			mid_color.b = ((factor - in_min) * (end_color.b as i32 - start_color.r as i32) / (in_max - in_min) + start_color.b) as u8;
			mid_color
		}
	}

	//the rainbow struct contains a list of colors in order and a number of colors.
	#[derive(Default, Copy, Clone)]
	pub struct Rainbow {
		colors: [Option<Color>; MAX_COLORS_IN_RAINBOW],
		num_colors: u8,
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
	pub const C_RED: Color = Color { r: GAMMA8[255], g: GAMMA8[0], b: GAMMA8[0] };
	pub const C_ORANGE: Color = Color { r: GAMMA8[255], g: GAMMA8[127], b: GAMMA8[0] };
	pub const C_YELLOW: Color = Color { r: GAMMA8[255], g: GAMMA8[255], b: GAMMA8[0] };
	pub const C_YELLOW_GREEN: Color = Color { r: GAMMA8[127], g: GAMMA8[255], b: GAMMA8[0] };
	pub const C_GREEN: Color = Color { r: GAMMA8[0], g: GAMMA8[255], b: GAMMA8[0] };
	pub const C_GREEN_BLUE: Color = Color { r: GAMMA8[0], g: GAMMA8[255], b: GAMMA8[127] };
	pub const C_SKY_BLUE: Color = Color { r: GAMMA8[0], g: GAMMA8[255], b: GAMMA8[255] };
	pub const C_DEEP_BLUE: Color = Color { r: GAMMA8[0], g: GAMMA8[127], b: GAMMA8[255] };
	pub const C_BLUE: Color = Color { r: GAMMA8[0], g: GAMMA8[0], b: GAMMA8[255] };
	pub const C_BLUE_PURPLE: Color = Color { r: GAMMA8[127], g: GAMMA8[0], b: GAMMA8[255] };
	pub const C_PURPLE: Color = Color { r: GAMMA8[255], g: GAMMA8[0], b: GAMMA8[255] };
	pub const C_DARK_PURPLE: Color = Color { r: GAMMA8[255], g: GAMMA8[0], b: GAMMA8[127] };
	pub const C_WHITE: Color = Color { r: GAMMA8[255], g: GAMMA8[255], b: GAMMA8[127] };
	pub const C_OFF: Color = Color { r: GAMMA8[0], g: GAMMA8[0], b: GAMMA8[0] };
	pub const C_T_3000K: Color = Color { r: GAMMA8[255], g: GAMMA8[180], b: GAMMA8[107] };
	pub const C_T_3500K: Color = Color { r: GAMMA8[255], g: GAMMA8[196], b: GAMMA8[137] };
	pub const C_T_4000K: Color = Color { r: GAMMA8[255], g: GAMMA8[209], b: GAMMA8[163] };
	pub const C_T_5000K: Color = Color { r: GAMMA8[255], g: GAMMA8[228], b: GAMMA8[206] };

	//generic rainbows:
	pub const R_OFF: Rainbow = Rainbow {
		colors: [
			Some(C_OFF),
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
		],
		num_colors: 1,
	};
	pub const R_ON: Rainbow = Rainbow {
		colors: [
			Some(C_WHITE),
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
		],
		num_colors: 1,
	};
	pub const R_RED: Rainbow = Rainbow {
		colors: [
			Some(C_RED),
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
		],
		num_colors: 1,
	};
	pub const R_ORANGE: Rainbow = Rainbow {
		colors: [
			Some(C_ORANGE),
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
		],
		num_colors: 1,
	};
	pub const R_YELLOW: Rainbow = Rainbow {
		colors: [
			Some(C_YELLOW),
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
		],
		num_colors: 1,
	};
	pub const R_YELLOW_GREEN: Rainbow = Rainbow {
		colors: [
			Some(C_YELLOW_GREEN),
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
		],
		num_colors: 1,
	};
	pub const R_GREEN: Rainbow = Rainbow {
		colors: [
			Some(C_GREEN),
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
		],
		num_colors: 1,
	};
	pub const R_GREEN_BLUE: Rainbow = Rainbow {
		colors: [
			Some(C_GREEN_BLUE),
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
		],
		num_colors: 1,
	};
	pub const R_SKY_BLUE: Rainbow = Rainbow {
		colors: [
			Some(C_SKY_BLUE),
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
		],
		num_colors: 1,
	};
	pub const R_DEEP_BLUE: Rainbow = Rainbow {
		colors: [
			Some(C_DEEP_BLUE),
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
		],
		num_colors: 1,
	};
	pub const R_BLUE: Rainbow = Rainbow {
		colors: [
			Some(C_BLUE),
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
		],
		num_colors: 1,
	};
	pub const R_BLUE_PURPLE: Rainbow = Rainbow {
		colors: [
			Some(C_BLUE_PURPLE),
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
		],
		num_colors: 1,
	};
	pub const R_PURPLE: Rainbow = Rainbow {
		colors: [
			Some(C_PURPLE),
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
		],
		num_colors: 1,
	};
	pub const R_DARK_PURPLE: Rainbow = Rainbow {
		colors: [
			Some(C_DARK_PURPLE),
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
			None,
		],
		num_colors: 1,
	};
	pub const R_ROYGBIV: Rainbow = Rainbow {
		colors: [
			Some(C_RED),
			Some(C_YELLOW),
			Some(C_GREEN),
			Some(C_SKY_BLUE),
			Some(C_BLUE),
			Some(C_PURPLE),
			None,
			None,
			None,
			None,
		],
		num_colors: 6,
	};
	pub const R_DOUBLE_ROYGBIV: Rainbow = Rainbow {
		colors: [
			Some(C_RED),
			Some(C_GREEN),
			Some(C_BLUE),
			Some(C_RED),
			Some(C_GREEN),
			Some(C_BLUE),
			None,
			None,
			None,
			None,
		],
		num_colors: 6,
	};
	pub const R_RYB: Rainbow = Rainbow {
		colors: [
			Some(C_RED),
			Some(C_OFF),
			Some(C_YELLOW),
			Some(C_OFF),
			Some(C_BLUE),
			Some(C_OFF),
			None,
			None,
			None,
			None,
		],
		num_colors: 6,
	};
	pub const R_OGP: Rainbow = Rainbow {
		colors: [
			Some(C_OFF),
			Some(C_ORANGE),
			Some(C_OFF),
			Some(C_GREEN),
			Some(C_OFF),
			Some(C_PURPLE),
			None,
			None,
			None,
			None,
		],
		num_colors: 6,
	};
	pub const R_RGB: Rainbow = Rainbow {
		colors: [
			Some(C_RED),
			Some(C_OFF),
			Some(C_GREEN),
			Some(C_OFF),
			Some(C_BLUE),
			Some(C_OFF),
			None,
			None,
			None,
			None,
		],
		num_colors: 6,
	};
	pub const R_RB: Rainbow = Rainbow {
		colors: [
			Some(C_OFF),
			Some(C_RED),
			Some(C_OFF),
			Some(C_SKY_BLUE),
			Some(C_OFF),
			Some(C_RED),
			Some(C_OFF),
			Some(C_SKY_BLUE),
			None,
			None,
		],
		num_colors: 8,
	};
	pub const R_BY: Rainbow = Rainbow {
		colors: [
			Some(C_OFF),
			Some(C_BLUE),
			Some(C_OFF),
			Some(C_YELLOW),
			Some(C_OFF),
			Some(C_BLUE),
			Some(C_OFF),
			Some(C_YELLOW),
			None,
			None,
		],
		num_colors: 8,
	};
	pub const R_OB: Rainbow = Rainbow {
		colors: [
			Some(C_OFF),
			Some(C_ORANGE),
			Some(C_OFF),
			Some(C_DEEP_BLUE),
			Some(C_OFF),
			Some(C_ORANGE),
			Some(C_OFF),
			Some(C_DEEP_BLUE),
			None,
			None,
		],
		num_colors: 8,
	};
	pub const R_BW: Rainbow = Rainbow {
		colors: [
			Some(C_OFF),
			Some(C_BLUE),
			Some(C_OFF),
			Some(C_WHITE),
			Some(C_OFF),
			Some(C_BLUE),
			Some(C_OFF),
			Some(C_WHITE),
			None,
			None,
		],
		num_colors: 8,
	};
	pub const R_GW: Rainbow = Rainbow {
		colors: [
			Some(C_OFF),
			Some(C_GREEN),
			Some(C_OFF),
			Some(C_WHITE),
			Some(C_OFF),
			Some(C_GREEN),
			Some(C_OFF),
			Some(C_WHITE),
			None,
			None,
		],
		num_colors: 8,
	};
	pub const R_RW: Rainbow = Rainbow {
		colors: [
			Some(C_OFF),
			Some(C_RED),
			Some(C_OFF),
			Some(C_WHITE),
			Some(C_OFF),
			Some(C_RED),
			Some(C_OFF),
			Some(C_WHITE),
			None,
			None,
		],
		num_colors: 8,
	};
	pub const R_VU: Rainbow = Rainbow {
		colors: [
			Some(C_GREEN),
			Some(C_GREEN),
			Some(C_GREEN),
			Some(C_GREEN),
			Some(C_GREEN),
			Some(C_GREEN),
			Some(C_GREEN),
			Some(C_YELLOW),
			Some(C_YELLOW),
			Some(C_RED),
		],
		num_colors: 10,
	};
	pub const R_DARK_RED_PATTERN: Rainbow = Rainbow {
		colors: [
			Some(Color {r: 127, g: 0, b: 0}),
			Some(Color {r: 64, g: 0, b: 0}),
			Some(Color {r: 127, g: 0, b: 0}),
			Some(Color {r: 64, g: 0, b: 0}),
			Some(Color {r: 127, g: 0, b: 0}),
			Some(Color {r: 64, g: 0, b: 0}),
			None,
			None,
			None,
			None,
		],
		num_colors: 6,
	};
	pub const R_DARK_YELLOW_PATTERN: Rainbow = Rainbow {
		colors: [
			Some(Color {r: 127, g: 127, b: 0}),
			Some(Color {r: 64, g: 64, b: 0}),
			Some(Color {r: 127, g: 127, b: 0}),
			Some(Color {r: 64, g: 64, b: 0}),
			Some(Color {r: 127, g: 127, b: 0}),
			Some(Color {r: 64, g: 64, b: 0}),
			None,
			None,
			None,
			None,
		],
		num_colors: 6,
	};
	pub const R_DARK_GREEN_PATTERN: Rainbow = Rainbow {
		colors: [
			Some(Color {r: 0, g: 127, b: 0}),
			Some(Color {r: 0, g: 64, b: 0}),
			Some(Color {r: 0, g: 127, b: 0}),
			Some(Color {r: 0, g: 64, b: 0}),
			Some(Color {r: 0, g: 127, b: 0}),
			Some(Color {r: 0, g: 64, b: 0}),
			None,
			None,
			None,
			None,
		],
		num_colors: 6,
	};
	pub const R_DARK_SKY_BLUE_PATTERN: Rainbow = Rainbow {
		colors: [
			Some(Color {r: 0, g: 127, b: 127}),
			Some(Color {r: 0, g: 64, b: 64}),
			Some(Color {r: 0, g: 127, b: 127}),
			Some(Color {r: 0, g: 64, b: 64}),
			Some(Color {r: 0, g: 127, b: 127}),
			Some(Color {r: 0, g: 64, b: 64}),
			None,
			None,
			None,
			None,
		],
		num_colors: 6,
	};
	pub const R_DARK_BLUE_PATTERN: Rainbow = Rainbow {
		colors: [
			Some(Color {r: 0, g: 0, b: 127}),
			Some(Color {r: 0, g: 0, b: 64}),
			Some(Color {r: 0, g: 0, b: 127}),
			Some(Color {r: 0, g: 0, b: 64}),
			Some(Color {r: 0, g: 0, b: 127}),
			Some(Color {r: 0, g: 0, b: 64}),
			None,
			None,
			None,
			None,
		],
		num_colors: 6,
	};
	pub const R_DARK_PURPLE_PATTERN: Rainbow = Rainbow {
		colors: [
			Some(Color {r: 127, g: 0, b: 127}),
			Some(Color {r: 64, g: 0, b: 64}),
			Some(Color {r: 127, g: 0, b: 127}),
			Some(Color {r: 64, g: 0, b: 64}),
			Some(Color {r: 127, g: 0, b: 127}),
			Some(Color {r: 64, g: 0, b: 64}),
			None,
			None,
			None,
			None,
		],
		num_colors: 6,
	};
	pub const R_WHITE_PATTERN: Rainbow = Rainbow {
		colors: [
			Some(C_WHITE),
			Some(Color {r: 64, g: 64, b: 64}),
			Some(C_WHITE),
			Some(Color {r: 64, g: 64, b: 64}),
			Some(C_WHITE),
			Some(Color {r: 64, g: 64, b: 64}),
			None,
			None,
			None,
			None,
		],
		num_colors: 6,
	};

	pub const NUM_RAINBOWS: usize = 32;

	pub const RAINBOW_ARRAY: [Rainbow; NUM_RAINBOWS] = [
		R_OFF,
		R_ON,
		R_RED,
		R_ORANGE,
		R_YELLOW,
		R_YELLOW_GREEN,
		R_GREEN,
		R_GREEN_BLUE,
		R_SKY_BLUE,
		R_DEEP_BLUE,
		R_BLUE,
		R_BLUE_PURPLE,
		R_PURPLE,
		R_DARK_PURPLE,
		R_ROYGBIV,
		R_DOUBLE_ROYGBIV,
		R_RYB,
		R_OGP,
		R_RGB,
		R_BY,
		R_RB,
		R_OB,
		R_BW,
		R_RW,
		R_GW,
		R_DARK_RED_PATTERN,
		R_DARK_YELLOW_PATTERN,
		R_DARK_GREEN_PATTERN,
		R_DARK_SKY_BLUE_PATTERN,
		R_DARK_BLUE_PATTERN,
		R_DARK_PURPLE_PATTERN,
		R_WHITE_PATTERN,
	];
}
