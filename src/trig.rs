pub use core::f32::consts::PI;
pub use core::f32::consts::FRAC_PI_2 as PI_HALF;
pub const PI_TWO:f32 = PI * 2f32;
pub const PI_SQUARED:f32 = PI * PI;
pub const TAU:f32 = core::f32::consts::TAU;
pub const TAU_SQUARED:f32 = TAU * TAU;

pub const fn cos_approx_a1(a:f32) -> f32 {
	let a2 = a*a;
	let a4 = a2*a2;
	return 1.0 - a2/2.0 + a4/24.0;
}

/// Bhaskara I's sine approximation formula
///
/// Source: https://en.wikipedia.org/wiki/Bhaskara_I%27s_sine_approximation_formula
pub const fn cos_bhaskara_1(a:f32) -> f32 {
	const FIVE_PI:f32 = 5f32 * PI_SQUARED;

	let a = a + PI_HALF;

	let a_times_pi_minus_a = a * (PI - a);
	return (
		16.0 * a_times_pi_minus_a
	) / (
		FIVE_PI - 4.0 * a_times_pi_minus_a
	);
}
pub const fn cos_bhaskara_2(a:f32) -> f32 {
	let a2 = a*a;
	return (
		PI_SQUARED - 4.0 * a2
	) / (
		PI_SQUARED + a2
	);
}
pub const fn cos_bhaskara_3(a:f32) -> f32 {
	let a2 = a*a;
	return 1.0 - (
		20.0 * a2
	) / (
		4.0 * a2 + TAU_SQUARED
	);
}

pub fn sin32(a:f32) -> f32 {
	cos32(PI_HALF - a)
}
pub fn cos32(a:f32) -> f32 {
	let a = a.abs(); // cos(-x) = cos(x)
//	let a = unsafe { intrinsics::fabsf32(a) };
	let a = a % PI_TWO; // 0 <= a <= 2*PI
	let quad = (a / PI_HALF) as u8;
	return match quad {
		1 => -cos73s(PI-a),
		2 => -cos73s(a-PI),
		3 =>  cos73s(PI_TWO-a),
		_ =>  cos73s(a),
	};
}
pub const fn cos32s(a:f32) -> f32 {
	const C1:f32 =  0.99940307;
	const C2:f32 = -0.49558072;
	const C3:f32 =  0.03679168;

	let a2 = a*a;

	return C1 + a2 * (C2 + a2 * C3);
}
pub const fn cos52s(a:f32) -> f32 {
	const C1:f32 =  0.9999932946;
	const C2:f32 = -0.4999124376;
	const C3:f32 =  0.0414877472;
	const C4:f32 = -0.0012712095;

	let a2 = a*a;

	return C1 + a2 * (C2 + a2 * (C3 + a2 * C4));
}
pub const fn cos73s(a:f32) -> f32 {
	const C1:f32 =  0.999999953464;
	const C2:f32 = -0.4999999053455;
	const C3:f32 =  0.0416635846769;
	const C4:f32 = -0.0013853704264;
	const C5:f32 =  0.000023233;

	let a2 = a*a;

	return C1 + a2 * (C2 + a2 * (C3 + a2 * (C4 + a2 * C5)));
}

mod benchmarks {
	extern crate test;
	use test::Bencher;
	use super::*;

	#[bench]
	fn bench_cos_std(b: &mut Bencher) {
		b.iter(|| accumulate(f32::cos));
	}

	#[bench]
	fn bench_cos_32(b: &mut Bencher) {
		b.iter(|| accumulate(cos32));
	}

	const ITERATIONS:u32 = 1000;
	fn accumulate(f:fn(a:f32)->f32) -> u32 {
		let mut ret = 0;
		let mut number = 0f32;
		for _ in 0..ITERATIONS {
			ret ^= f(number).to_bits();
			number += 0.01f32;
		}
		return ret;
	}
}