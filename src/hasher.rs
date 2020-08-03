// fn fetch64(input:&[u8]) -> u64 {
// 	u32::from_le_bytes([input[0], input[1], input[2], input[3], input[4], input[5], input[6], input[7]])
// }
// fn fetch32(input:&[u8]) -> u32 {
// 	u32::from_le_bytes([input[0], input[1], input[2], input[3]])
// }
// fn fetch8(input:&[u8]) -> u8 {
// 	input[0]
// }

mod xx_hasher_32 {
	const P1:u32 = 0x9E3779B1;
	const P2:u32 = 0x85EBCA77;
	const P3:u32 = 0xC2B2AE3D;
	const P4:u32 = 0x27D4EB2F;
	const P5:u32 = 0x165667B1;

	// fn init(seed:u32) -> [u32;4] {
	// 	[
	// 		seed + P1 + P2,
	// 		seed + P2,
	// 		seed,
	// 		seed - P1,
	// 	]
	// }

	// fn stripe(v:&mut [u32;4]) {
	// 	for s:[u32;4] in data {
	// 		v[0] = (v[0] + (s[0] * P2)).rotate_left(13) * P1;
	// 		v[1] = (v[1] + (s[1] * P2)).rotate_left(13) * P1;
	// 		v[2] = (v[2] + (s[2] * P2)).rotate_left(13) * P1;
	// 		v[3] = (v[3] + (s[3] * P2)).rotate_left(13) * P1;
	// 	}
	// }

	// fn merge() {
	// 	hash = v[0].rotate_left( 1) +
	// 				 v[1].rotate_left( 7) +
	// 				 v[2].rotate_left(12) +
	// 				 v[3].rotate_left(18);
	// }

	// fn last_4_15() {
	// 	for v:u32 in data {
	// 		hash = (hash + v * P3).rotate_left(17) * P4;
	// 	}
	// }
	
	// fn last_0_3() {
	// 	for v:u8 in data {
	// 		hash = (hash + (v as u32 * P5)).rotate_left(11) * P1;
	// 	}
	// }

	fn finalize(mut hash:u32) -> u32 {
		hash ^= hash >> 15;
		hash *= P2;
		hash ^= hash >> 13;
		hash *= P3;
		hash ^= hash >> 16;
		return hash;
	}

	// pub fn hash(seed:u32, input:&[u8]) {
	// 	let mut hash = 0;
	// 	if length >= 16 as u64 {
	// 		let v = init();
	// 		stripe();
	// 		merge();
	// 	} else {
	// 		hash = seed + P5;
	// 	}
	
	// 	hash += length as u32;
	
	// 	last_4_15(&mut hash);
	// 	last_0_3(&mut hash);
	
	// 	return finalize(hash);
	// }

	pub fn hash8(seed:u32, v:u8) -> u32 {
		let mut hash = seed + P5 + 1;
		hash = (hash + (v as u32 * P5)).rotate_left(11) * P1;
		return finalize(hash);
	}

	pub fn hash16(seed:u32, mut v:u16) -> u32  {
		v = v.to_le();
		let mut hash = seed + P5 + 2;
		hash = (hash + ((v & 0xFF) as u32 * P5)).rotate_left(11) * P1;
		hash = (hash + ((v >> 8  ) as u32 * P5)).rotate_left(11) * P1;
		return finalize(hash);
	}
 
	pub fn hash32(seed:u32, mut v:u32) -> u32  {
		v = v.to_le();
		let mut hash = seed + P5 + 4;
		hash = (hash + (v * P3)).rotate_left(17) * P4;
		return finalize(hash);
	}
	pub fn hash64(seed:u32, mut v:u64) -> u32 {
		v = v.to_le();
		let mut hash = seed + P5 + 8;
		hash = (hash + ((v      ) as u32 * P3)).rotate_left(17) * P4;
		hash = (hash + ((v >> 32) as u32 * P3)).rotate_left(17) * P4;
		return finalize(hash);
	}
}

mod xx_hasher_64 {
	const P1:u64 = 0x9E3779B185EBCA87;
	const P2:u64 = 0xC2B2AE3D27D4EB4F;
	const P3:u64 = 0x165667B19E3779F9;
	const P4:u64 = 0x85EBCA77C2B2AE63;
	const P5:u64 = 0x27D4EB2F165667C5;

	// fn init(seed:u64) -> [u64;4] {
	// 	[
	// 		seed + P1 + P2,
	// 		seed + P2,
	// 		seed,
	// 		seed - P1,
	// 	]
	// }

	// fn stripe(v:&mut [u64;4]) {
	// 	for s:[u64;4] in data {
	// 		v[0] = (v[0] + (s[0] * P2)).rotate_left(31) * P1;
	// 		v[1] = (v[1] + (s[1] * P2)).rotate_left(31) * P1;
	// 		v[2] = (v[2] + (s[2] * P2)).rotate_left(31) * P1;
	// 		v[3] = (v[3] + (s[3] * P2)).rotate_left(31) * P1;
	// 	}
	// }

	// fn merge(v:&mut [u64;4]) {
	// 	hash = v1.rotate_left( 1) +
	// 				v2.rotate_left( 7) +
	// 				v3.rotate_left(12) +
	// 				v4.rotate_left(18);

	// 	hash = (hash ^ (v1 * P2).rotate_left(31) * P1) * P1 + P4;
	// 	hash = (hash ^ (v2 * P2).rotate_left(31) * P1) * P1 + P4;
	// 	hash = (hash ^ (v3 * P2).rotate_left(31) * P1) * P1 + P4;
	// 	hash = (hash ^ (v4 * P2).rotate_left(31) * P1) * P1 + P4;
	// }

	// fn last_8_15() {
	// 	for v:u64 in data {
	// 		hash = hash ^ ((v * P2).rotate_left(31) * P1);
	// 		hash = hash.rotate_left(27) * P1 + P4;
	// 	}
	// }

	// fn last_4_7() {
	// 	for v:u32 in data {
	// 		hash = (hash ^ (v as u64 * P1)).rotate_left(23) * P2 + P3;
	// 	}
	// }

	// fn last_0_3() {
	// 	for v:u8 in data {
	// 		hash = (hash ^ (v as u64 * P5)).rotate_left(11) * P1;
	// 	}
	// }

	#[inline(always)]
	fn finalize(mut hash:u64) -> u64 {
		hash ^= hash >> 33;
		hash *= P2;
		hash ^= hash >> 29;
		hash *= P3;
		hash ^= hash >> 32;
		return hash;
	}

	// pub fn hash(seed:u64, input:&[u8]) -> u64 {
	// 	let mut hash:u64 = 0;
	
	// 	if remaining >= 32 {
	// 		let v = init(seed);
	
	// 		stripe(v);
	// 		merge(v);
	// 	} else {
	// 		hash = seed + P5;
	// 	}
	
	// 	hash += inputLength as u32;
	
	// 	last_8_15(&mut hash);
	// 	last_4_7(&mut hash);
	// 	last_0_3(&mut hash);
	
	// 	return finalize(hash);
	// }

	pub fn hash8(seed:u64, v:u8) -> u64 {
		let mut hash = seed + P5 + 1;
		hash = (hash ^ (v as u64 * P5)).rotate_left(11) * P1;
		return finalize(hash);
	}

	pub fn hash16(seed:u64, mut v:u16) -> u64  {
		v = v.to_le();
		let mut hash = seed + P5 + 2;
		hash = (hash ^ ((v & 0xFF) as u64 * P5)).rotate_left(11) * P1;
		hash = (hash ^ ((v >> 8  ) as u64 * P5)).rotate_left(11) * P1;
		return finalize(hash);
	}
 
	pub fn hash32(seed:u64, mut v:u32) -> u64  {
		v = v.to_le();
		let mut hash = seed + P5 + 4;
		hash = (hash ^ (v as u64 * P1)).rotate_left(23) * P2 + P3;
		return finalize(hash);
	}

	pub fn hash64(seed:u64, mut v:u64) -> u64 {
		v = v.to_le();
		let mut hash = seed + P5 + 8;
		hash = (hash ^ ((v * P2).rotate_left(31) * P1)).rotate_left(27) * P1 + P4;
		return finalize(hash);
	}
}

pub trait HasherTrait<S, H> {
	fn hash_u8( seed:S, v:u8 ) -> H;
	fn hash_u16(seed:S, v:u16) -> H;
	fn hash_u32(seed:S, v:u32) -> H;
	fn hash_u64(seed:S, v:u64) -> H;
}

pub type Hasher = HasherXX32;

#[derive(Copy, Clone)] pub struct HasherXX32();
#[derive(Copy, Clone)] pub struct HasherXX64();
#[derive(Copy, Clone)] pub struct HasherMurMur2A();

impl HasherTrait<u32, u32> for HasherXX32 {
	#[inline(always)] fn hash_u8( seed:u32, v:u8 ) -> u32 { xx_hasher_32::hash8( seed, v) }
	#[inline(always)] fn hash_u16(seed:u32, v:u16) -> u32 { xx_hasher_32::hash16(seed, v) }
	#[inline(always)] fn hash_u32(seed:u32, v:u32) -> u32 { xx_hasher_32::hash32(seed, v) }
	#[inline(always)] fn hash_u64(seed:u32, v:u64) -> u32 { xx_hasher_32::hash64(seed, v) }
}

impl HasherTrait<u64, u64> for HasherXX64 {
	#[inline(always)] fn hash_u8( seed:u64, v:u8 ) -> u64 { xx_hasher_64::hash8( seed, v) }
	#[inline(always)] fn hash_u16(seed:u64, v:u16) -> u64 { xx_hasher_64::hash16(seed, v) }
	#[inline(always)] fn hash_u32(seed:u64, v:u32) -> u64 { xx_hasher_64::hash32(seed, v) }
	#[inline(always)] fn hash_u64(seed:u64, v:u64) -> u64 { xx_hasher_64::hash64(seed, v) }
}
