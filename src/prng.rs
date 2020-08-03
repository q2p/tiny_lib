// Middle Square Weyl Sequence PRNG

const WEYL_CONST:u64 = 0xb5ad4eceda1ce2a9;

pub type RNG = MSWS;

pub struct MSWS {
	x: u64,
	w: u64,
	s: u64,
}

impl MSWS {
	pub const fn new(seed:u64) -> MSWS {
		let mut ret = MSWS {
			x: 0,
			w: 0,
			s: (seed << 1).wrapping_add(WEYL_CONST),
		};
		ret.tick();
		return ret;
	}
	#[inline(always)]
	const fn tick(&mut self) {
		self.w = self.w.wrapping_add(self.s); // w += s
		self.x = self.x.wrapping_mul(self.x).wrapping_add(self.w); // x = x*x+w
		self.x = (self.x >> 32) | (self.x << 32);
	}
	pub const fn get_u32(&mut self) -> u32 {
		self.tick();
		return self.x as u32;
	}
	pub fn get_u64(&mut self) -> u64 {
		((self.get_u32() as u64)      ) |
		((self.get_u32() as u64) << 32)
	}
	/// Range of [0.0, 1.0] inclusive.
	pub fn get_f32(&mut self) -> f32 {
		self.get_u32() as f32 / u32::MAX as f32
	}
	pub fn new_seeded(&mut self) -> MSWS {
		MSWS::new(self.get_u64())
	}
}

impl Default for MSWS {
	fn default() -> MSWS {
		MSWS::new(0)
	}
}

pub static mut GLOBAL_PRNG:RNG = RNG::new(0);