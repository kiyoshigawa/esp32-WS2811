pub mod animations {
	use crate::c;
	pub struct Animation {
		buffer_size: usize,
	}
	impl Animation {
		pub fn new(buffer_size: usize) -> Self {
			Animation {
				buffer_size,
			}
		}
	}
}
