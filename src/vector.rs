use core::ops::*;
use crate::util::*;

pub trait Vec : Add + Sub + Mul<f32> + AddAssign + SubAssign + MulAssign<f32> + Copy + Clone {
	const ZERO:Self;
	const HALF:Self;
	const ONE:Self;
	fn dot(&self, v2:&Self) -> f32;
	fn len(&self) -> f32;
	fn inv_len(&self) -> f32;
	fn normalize(&self) -> Self;
	fn normalize2(&self) -> Self;
}

macro_rules! impl_vec {
	(
    $($vec:ident; ( $($var:ident),+ ));+
  ) => {
  	$(
			#[derive(Copy, Clone)]
			pub struct $vec {
				$(pub $var: f32),+
			}
			impl $vec {
				#[inline(always)]
				pub const fn new($($var: f32),+) -> $vec {
					$vec {
						$($var),+
					}
				}
			}
			impl Vec for $vec {
				const ZERO:$vec = $vec{
					$($var: 0f32),+
				};
				const HALF:$vec = $vec{
					$($var: 0.5f32),+
				};
				const ONE:$vec = $vec{
					$($var: 1f32),+
				};
				#[inline(always)]
				fn dot(&self, v2:&$vec) -> f32 {
					0f32 $(
						+ (self.$var * v2.$var)
					)+
				}
				#[inline(always)]
				fn len(&self) -> f32 {
					(0f32 $(
						+ (self.$var * self.$var)
					)+).sqrt()
				}
				#[inline(always)]
				fn inv_len(&self) -> f32 {
					(0f32 $(
						+ (self.$var * self.$var)
					)+).quick_inverse_sqrt()
				}
				#[inline(always)]
				fn normalize2(&self) -> $vec {
					let len = self.len();
					$vec {
						$($var: self.$var / len),+
					}
				}
				#[inline(always)]
				fn normalize(&self) -> $vec {
					let inv_len = self.inv_len();
					$vec {
						$($var: self.$var * inv_len),+
					}
				}
			}
			impl Add<&$vec> for &$vec {
				type Output = $vec;
				#[inline(always)]
				fn add(self, rhs: &$vec) -> $vec {
					$vec {
						$($var: self.$var + rhs.$var),+
					}
				}
			}
			impl Sub<&$vec> for &$vec {
				type Output = $vec;
				#[inline(always)]
				fn sub(self, rhs: &$vec) -> $vec {
					$vec {
						$($var: self.$var - rhs.$var),+
					}
				}
			}
			impl Add<$vec> for $vec { type Output = $vec; #[inline(always)] fn add(self, rhs: $vec) -> $vec { &self + &rhs } }
			impl Sub<$vec> for $vec { type Output = $vec; #[inline(always)] fn sub(self, rhs: $vec) -> $vec { &self - &rhs } }
			impl Add<f32> for $vec {
				type Output = $vec;
				#[inline(always)]
				fn add(self, rhs: f32) -> $vec {
					$vec {
						$($var: self.$var + rhs),+
					}
				}
			}
			impl Add<$vec> for f32 {
				type Output = $vec;
				#[inline(always)]
				fn add(self, rhs: $vec) -> $vec { rhs + self }
			}
			impl Mul<f32> for $vec {
				type Output = $vec;
				#[inline(always)]
				fn mul(self, rhs: f32) -> $vec {
					$vec {
						$($var: self.$var * rhs),+
					}
				}
			}
			impl Mul<$vec> for f32 {
				type Output = $vec;
				#[inline(always)]
				fn mul(self, rhs: $vec) -> $vec { rhs * self }
			}
			impl Div<$vec> for $vec {
				type Output = $vec;
				fn div(self, rhs: $vec) -> $vec {
					$vec {
						$($var: self.$var / rhs.$var),+
					}
				}
			}
			impl AddAssign for $vec {
				#[inline(always)]
				fn add_assign(&mut self, rhs: $vec) {
					$(self.$var += rhs.$var;)+
				}
			}
			impl SubAssign for $vec {
				#[inline(always)]
				fn sub_assign(&mut self, rhs: $vec) {
					$(self.$var -= rhs.$var;)+
				}
			}
			impl MulAssign<f32> for $vec {
				#[inline(always)]
				fn mul_assign(&mut self, rhs: f32) {
					$(self.$var *= rhs;)+
				}
			}
			impl Lerp for &$vec {
				type Output = $vec;
				#[inline(always)]
				fn lerp(self, v2:&$vec, t:f32) -> $vec {
					$vec {
						$($var: f32::lerp(self.$var, v2.$var, t)),+
					}
				}
				#[inline(always)]
				fn lerp_p(self, v2:&$vec, t:f32) -> $vec {
					$vec {
						$($var: f32::lerp_p(self.$var, v2.$var, t)),+
					}
				}
			}
			impl Lerp for $vec {
				type Output = $vec;
				#[inline(always)] fn lerp  (self, v2:$vec, t:f32) -> $vec { (&self).lerp  (&v2, t) }
				#[inline(always)] fn lerp_p(self, v2:$vec, t:f32) -> $vec { (&self).lerp_p(&v2, t) }
			}
    )+
  }
}
impl_vec!(
	Vec1; (x);
	Vec2; (x,y);
	Vec3; (x,y,z);
	Vec4; (x,y,z,w)
);

impl Vec2 {
	#[inline(always)]
	pub fn to_v3(&self, z:f32) -> Vec3 {
		Vec3 {
			x: self.x,
			y: self.y,
			z,
		}
	}
}

impl Vec3 {
	pub const fn xy1(x:f32, y:f32) -> Vec3 {
		Vec3 { x, y, z: 1f32 }
	}
	#[inline(always)] pub const fn u(&mut self) -> &mut f32 { &mut self.x }
	#[inline(always)] pub const fn v(&mut self) -> &mut f32 { &mut self.y }
	#[inline(always)] pub const fn w(&mut self) -> &mut f32 { &mut self.z }
	#[inline(always)] pub const fn us(&self) -> f32 { self.x }
	#[inline(always)] pub const fn vs(&self) -> f32 { self.y }
	#[inline(always)] pub const fn ws(&self) -> f32 { self.z }

	#[inline(always)]
	pub fn cross_product(&self, v2:&Vec3) -> Vec3 {
		Vec3 {
			x: self.y * v2.z - self.z * v2.y,
			y: self.z * v2.x - self.x * v2.z,
			z: self.x * v2.y - self.y * v2.x,
		}
	}

	#[inline(always)]
	pub fn to_v4(&self, w:f32) -> Vec4 {
		Vec4 {
			x: self.x,
			y: self.y,
			z: self.z,
			w,
		}
	}
}
impl Vec4 {
	#[inline(always)]
	pub const fn from_v3(vec3:Vec3, w:f32) -> Vec4 {
		Vec4 { x:vec3.x, y:vec3.y, z:vec3.z, w }
	}
	#[inline(always)]
	pub const fn xyz1(x:f32, y:f32, z:f32) -> Vec4 {
		Vec4 { x, y, z, w: 1f32 }
	}
	#[inline(always)]
	pub const fn xyz(&self) -> Vec3 {
		Vec3 {
			x: self.x,
			y: self.y,
			z: self.z
		}
	}
	#[inline(always)] pub const fn w(&mut self) -> &mut f32 { &mut self.z }
	#[inline(always)] pub const fn ws(&self) -> f32 { self.z }
}

pub fn mul_vec3d(vec:&Vec4, multiplier:f32) -> Vec4 {
	return Vec4::xyz1(vec.x * multiplier, vec.y * multiplier, vec.z * multiplier);
}

pub fn div_vec3d(vec:&Vec4, divider:f32) -> Vec4 {
	return Vec4::xyz1(vec.x / divider, vec.y / divider, vec.z / divider);
}