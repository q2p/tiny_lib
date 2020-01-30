use core::num::NonZeroI8;
use core::ops::*;
use core::cmp::Ordering;

pub trait QuickInverseSQRT {
	fn quick_inverse_sqrt(self) -> Self;
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