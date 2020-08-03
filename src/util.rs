use core::num::NonZeroI8;
use core::ops::*;
use core::cmp::Ordering;
use core::f32::consts::PI;
use crate::matrices::*;
use crate::vector::*;

#[inline(always)]
pub fn projection_matrix(fov_degrees:f32, aspect_ratio:f32, plane_near:f32, plane_far:f32) -> Matrix4x4 {
	let fov_rad = 1f32 / (fov_degrees * PI / 360f32).tan();
	let mut matrix = Matrix4x4::ZERO;
	matrix.m[0][0] = aspect_ratio * fov_rad;
	matrix.m[1][1] = fov_rad;
	matrix.m[2][2] = plane_far / (plane_far - plane_near);
	matrix.m[3][2] = (-plane_far * plane_near) / (plane_far - plane_near);
	matrix.m[2][3] = 1f32;
	matrix.m[3][3] = 0f32;
	return matrix;
}

#[inline(always)]
pub fn transition_matrix(pos:&Vec3) -> Matrix4x4 {
	let mut matrix = Matrix4x4::IDENTITY;
	matrix.m[3][0] = pos.x;
	matrix.m[3][1] = pos.y;
	matrix.m[3][2] = pos.z;
	return matrix;
}

#[inline(always)]
pub fn rotation_matrix_z(a:f32) -> Matrix3x3 {
	let a_cos = a.cos();
	let a_sin = a.sin();

	Matrix3x3 {
		m: [
			[  a_cos,  a_sin,   0f32],
			[ -a_sin,  a_cos,   0f32],
			[   0f32,   0f32,   1f32],
		]
	}
}
#[inline(always)]
pub fn rotation_matrix_x(a:f32) -> Matrix3x3 {
	let a_cos = a.cos();
	let a_sin = a.sin();

	Matrix3x3 {
		m: [
			[   1f32,   0f32,   0f32],
			[   0f32,  a_cos,  a_sin],
			[   0f32, -a_sin,  a_cos],
		]
	}
}
#[inline(always)]
pub fn rotation_matrix_y(a:f32) -> Matrix3x3 {
	let a_cos = a.cos();
	let a_sin = a.sin();

	Matrix3x3 {
		m: [
			[  a_cos,   0f32, -a_sin],
			[   0f32,   1f32,   0f32],
			[  a_sin,   0f32,  a_cos],
		]
	}
}

pub fn point_at_matrix(pos:&Vec3, target:&Vec3, up:&Vec3) -> Matrix4x4 {
	// TODO: нужны ли все эти new_up, new_right, или их можно просчитать заранее?
	// TODO: need normalization?
	let new_forward:Vec3 = (target - pos).normalize();

	let a:Vec3 = new_forward * Vec3::dot(up, &new_forward);
	let new_up:Vec3 = (up - &a).normalize();

	let new_right = Vec3::cross_product(&new_up, &new_forward);

	return Matrix4x4 {
		m: [
			[  new_right.x,   new_right.y,   new_right.z, 0f32],
			[     new_up.x,      new_up.y,      new_up.z, 0f32],
			[new_forward.x, new_forward.y, new_forward.z, 0f32],
			[        pos.x,         pos.y,         pos.z, 1f32]
		]
	};
}

pub fn inverse_transformation_matrix(matrix:&Matrix4x4) -> Matrix4x4 {
	return Matrix4x4 {
		m: [
			[matrix.m[0][0], matrix.m[1][0], matrix.m[2][0], 0f32],
			[matrix.m[0][1], matrix.m[1][1], matrix.m[2][1], 0f32],
			[matrix.m[0][2], matrix.m[1][2], matrix.m[2][2], 0f32],
			[
				-(matrix.m[3][0] * matrix.m[0][0] + matrix.m[3][1] * matrix.m[0][1] + matrix.m[3][2] * matrix.m[0][2]),
				-(matrix.m[3][0] * matrix.m[1][0] + matrix.m[3][1] * matrix.m[1][1] + matrix.m[3][2] * matrix.m[1][2]),
				-(matrix.m[3][0] * matrix.m[2][0] + matrix.m[3][1] * matrix.m[2][1] + matrix.m[3][2] * matrix.m[2][2]),
				1f32
			]
		]
	};
}

pub fn multiply_vector_matrix(inp:&Vec4, matrix:&Matrix4x4) -> Vec4 {
	let mut out = Vec4 {
		x: inp.x * matrix.m[0][0] + inp.y * matrix.m[1][0] + inp.z * matrix.m[2][0] + matrix.m[3][0],
		y: inp.x * matrix.m[0][1] + inp.y * matrix.m[1][1] + inp.z * matrix.m[2][1] + matrix.m[3][1],
		z: inp.x * matrix.m[0][2] + inp.y * matrix.m[1][2] + inp.z * matrix.m[2][2] + matrix.m[3][2],
		w: inp.x * matrix.m[0][3] + inp.y * matrix.m[1][3] + inp.z * matrix.m[2][3] + matrix.m[3][3],
	};

	let w = inp.x * matrix.m[0][3] + inp.y * matrix.m[1][3] + inp.z * matrix.m[2][3] + matrix.m[3][3];

	if w != 0f32 { // TODO: когда w == 0 ?
		out.x /= w;
		out.y /= w;
		out.z /= w;
	}

	return out;
}

pub trait QuickInverseSQRT {
	fn quick_inverse_sqrt(self) -> Self;
}

pub trait Square {
	fn sq(self) -> Self;
}

#[repr(C)]
union IntegerFloat32Union {
	i: u32,
	f: f32,
}

impl QuickInverseSQRT for f32 {
	fn quick_inverse_sqrt(self) -> f32 {
		const MAGIC_NUMBER:u32 = 0x5f3759df; // TODO: maybe 0x5f375a86 is more accurate?

		const THREE_HALFS:f32 = 1.5;

		let half_x = 0.5f32 * self;

		let mut y = unsafe {
			let mut union = IntegerFloat32Union { f: self };

			// gives initial guess y 0
			union.i = MAGIC_NUMBER - (union.i >> 1);

			// convert bits back to float
			union.f
		};

		// Newton step, repeating increases accuracy
		y = y * ( THREE_HALFS - ( half_x * y * y ) ); // 1st iteration
		// this can be removed
		// y = y * ( THREE_HALFS - ( half_x * y * y ) ); // 2nd iteration

		return y;
	}
}

impl Square for f32 {
	#[inline(always)]
	fn sq(self) -> f32 { self*self }
}

#[derive(Copy, Clone, Ord, Eq)]
pub struct NonZeroSignum(NonZeroI8);

impl NonZeroSignum {
	// Unsafe, because the argument might be 0. But we are only using 1 and -1.
	// When const_fn will become stable, we can just const `.unwrap()` the result and not use unsafe.
	pub const POS:NonZeroSignum = NonZeroSignum(unsafe { NonZeroI8::new_unchecked( 1) });
	pub const NEG:NonZeroSignum = NonZeroSignum(unsafe { NonZeroI8::new_unchecked(-1) });

	#[inline(always)]
	pub fn is_positive(&self) -> bool {
		self.0.get() > 0
	}
	#[inline(always)]
	pub fn is_negative(&self) -> bool {
		self.0.get() < 0
	}
}
impl Neg for NonZeroSignum {
	type Output = NonZeroSignum;
	fn neg(self) -> NonZeroSignum {
		// Unsafe, because `self.0` might be 0. But it's guaranteed not to be 0, because we are using NonZero struct.
		NonZeroSignum(unsafe { NonZeroI8::new_unchecked(-self.0.get()) })
	}
}

impl PartialEq for NonZeroSignum {
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool { self.0.get() == other.0.get() }
	#[inline(always)]
	fn ne(&self, other: &Self) -> bool { self.0.get() != other.0.get() }
}

impl PartialOrd for NonZeroSignum {
	#[inline(always)]
	fn partial_cmp(&self, other: &NonZeroSignum) -> Option<Ordering> { Some(self.0.get().cmp(&other.0.get())) }
	#[inline(always)]
	fn lt(&self, other: &NonZeroSignum) -> bool { self.0.get() <  other.0.get() }
	#[inline(always)]
	fn le(&self, other: &NonZeroSignum) -> bool { self.0.get() <= other.0.get() }
	#[inline(always)]
	fn gt(&self, other: &NonZeroSignum) -> bool { self.0.get() >  other.0.get() }
	#[inline(always)]
	fn ge(&self, other: &NonZeroSignum) -> bool { self.0.get() >= other.0.get() }
}

// TODO: tests
macro_rules! impl_signum_signed {
	($($t:ty),*) => {
		$(
			impl Into<$t> for NonZeroSignum {
				#[inline(always)]
				fn into(self) -> $t { self.0.get() as $t }
			}
			impl Mul<$t> for NonZeroSignum {
				type Output = $t;
				#[inline(always)]
				fn mul(self, rhs: $t) -> $t { Into::<$t>::into(self) * rhs }
			}
			impl Mul<NonZeroSignum> for $t {
				type Output = $t;
				#[inline(always)]
				fn mul(self, rhs: NonZeroSignum) -> $t { self * Into::<$t>::into(rhs) }
			}
			impl MulAssign<NonZeroSignum> for $t {
				#[inline(always)]
				fn mul_assign(&mut self, rhs: NonZeroSignum) { *self *= Into::<$t>::into(rhs) }
			}
			impl Div<NonZeroSignum> for $t {
				type Output = $t;
				#[inline(always)]
				fn div(self, rhs: NonZeroSignum) -> $t { self * Into::<$t>::into(rhs) }
			}
			impl DivAssign<NonZeroSignum> for $t {
				#[inline(always)]
				fn div_assign(&mut self, rhs: NonZeroSignum) { *self *= Into::<$t>::into(rhs) }
			}
			impl Add<$t> for NonZeroSignum {
				type Output = $t;
				#[inline(always)]
				fn add(self, rhs: $t) -> $t { Into::<$t>::into(self) + rhs }
			}
			impl Add<NonZeroSignum> for $t {
				type Output = $t;
				#[inline(always)]
				fn add(self, rhs: NonZeroSignum) -> $t { self + Into::<$t>::into(rhs) }
			}
			impl Sub<NonZeroSignum> for $t {
				type Output = $t;
				#[inline(always)]
				fn sub(self, rhs: NonZeroSignum) -> $t { self - Into::<$t>::into(rhs) }
			}
			impl PartialEq<$t> for NonZeroSignum {
				#[inline(always)]
				fn eq(&self, other: &$t) -> bool { self.0.get() as $t == *other }
				#[inline(always)]
				fn ne(&self, other: &$t) -> bool { self.0.get() as $t != *other }
			}
			impl PartialEq<NonZeroSignum> for $t {
				#[inline(always)]
				fn eq(&self, other: &NonZeroSignum) -> bool { *self == other.0.get() as $t }
				#[inline(always)]
				fn ne(&self, other: &NonZeroSignum) -> bool { *self != other.0.get() as $t }
			}
			impl PartialOrd<$t> for NonZeroSignum {
				#[inline(always)]
				fn partial_cmp(&self, other: &$t) -> Option<Ordering> { Some((self.0.get() as $t).cmp(other)) }
				#[inline(always)]
				fn lt(&self, other: &$t) -> bool { (self.0.get() as $t) <  (*other) }
				#[inline(always)]
				fn le(&self, other: &$t) -> bool { (self.0.get() as $t) <= (*other) }
				#[inline(always)]
				fn gt(&self, other: &$t) -> bool { (self.0.get() as $t) >  (*other) }
				#[inline(always)]
				fn ge(&self, other: &$t) -> bool { (self.0.get() as $t) >= (*other) }
			}
			impl PartialOrd<NonZeroSignum> for $t {
				#[inline(always)]
				fn partial_cmp(&self, other: &NonZeroSignum) -> Option<Ordering> { Some(self.cmp(&(other.0.get() as $t))) }
				#[inline(always)]
				fn lt(&self, other: &NonZeroSignum) -> bool { (*self) <  (other.0.get() as $t) }
				#[inline(always)]
				fn le(&self, other: &NonZeroSignum) -> bool { (*self) <= (other.0.get() as $t) }
				#[inline(always)]
				fn gt(&self, other: &NonZeroSignum) -> bool { (*self) >  (other.0.get() as $t) }
				#[inline(always)]
				fn ge(&self, other: &NonZeroSignum) -> bool { (*self) >= (other.0.get() as $t) }
			}
			impl_assign!($t);
		)*
	}
}
macro_rules! impl_signum_unsigned {
	($real:ty, $temp:ty) => {
		impl Add<NonZeroSignum> for $real {
			type Output = $real;
			#[inline(always)]
			fn add(self, rhs: NonZeroSignum) -> $real { (self as $temp + Into::<$temp>::into(rhs)) as $real }
		}
		impl Add<$real> for NonZeroSignum {
			type Output = $real;
			#[inline(always)]
			fn add(self, rhs: $real) -> $real { rhs + self }
		}
		impl Sub<NonZeroSignum> for $real {
			type Output = $real;
			#[inline(always)]
			fn sub(self, rhs: NonZeroSignum) -> $real { (self as $temp - Into::<$temp>::into(rhs)) as $real }
		}
		impl_assign!($real);
	};
}
macro_rules! impl_assign {
  ($t:ty) => {
		impl AddAssign<NonZeroSignum> for $t {
			#[inline(always)]
			fn add_assign(&mut self, rhs: NonZeroSignum) { *self = *self + rhs }
		}
		impl SubAssign<NonZeroSignum> for $t {
			#[inline(always)]
			fn sub_assign(&mut self, rhs: NonZeroSignum) { *self = *self - rhs }
		}
  }
}
impl_signum_signed!(i8,i16,i32,i64);
impl_signum_unsigned!(u8 , i8 );
impl_signum_unsigned!(u16, i16);
impl_signum_unsigned!(u32, i32);
impl_signum_unsigned!(u64, i64);


pub trait Lerp {
	type Output;
	fn lerp(self, v2:Self, t:f32) -> Self::Output;
	/// Precise method, which guarantees v = v1 when t = 1.
	fn lerp_p(self, v2:Self, t:f32) -> Self::Output;
}

impl Lerp for f32 {
	type Output = f32;
	#[inline(always)]
	fn lerp(self, v2:Self, t:f32) -> f32 {
		self + t * (v2 - self)
	}
	#[inline(always)]
	fn lerp_p(self, v2:Self, t:f32) -> f32 {
		(1f32 - t) * self + t * v2
	}
}

pub trait Smoothstep {
	type Output;
	fn linearstep(edge0:Self, edge1:Self, t:f32) -> Self::Output;
	fn smoothstep(edge0:Self, edge1:Self, t:f32) -> Self::Output;
}

impl Smoothstep for f32 {
	type Output = f32;
	fn linearstep(edge0:f32, edge1:f32, t:f32) -> f32 {
		((t - edge0) / (edge1 - edge0)).clamp(0.0, 1.0)
	}
	fn smoothstep(edge0:f32, edge1:f32, t:f32) -> f32 {
		let a = Self::linearstep(edge0, edge1, t);
		return a * a * (3f32 - 2f32 * a);
	}
}