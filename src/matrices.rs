use core::ops::Mul;
use crate::vector::*;

pub struct Matrix3x3 {
	pub m:[[f32;3];3],
}
impl Matrix3x3 {
	pub const ZERO:Matrix3x3 = Matrix3x3 {
		m: [
			[0f32, 0f32, 0f32],
			[0f32, 0f32, 0f32],
			[0f32, 0f32, 0f32]
		]
	};
	pub const IDENTITY:Matrix3x3 = Matrix3x3 {
		m: [
			[1f32, 0f32, 0f32],
			[0f32, 1f32, 0f32],
			[0f32, 0f32, 1f32]
		]
	};
}

impl Mul for &Matrix3x3 {
	type Output = Matrix3x3;
	#[inline(always)]
	fn mul(self, rhs: &Matrix3x3) -> Matrix3x3 {
		let mut ret = Matrix3x3::ZERO;
		for c in 0..3 {
			for r in 0..3 {
				ret.m[r][c] =
					self.m[r][0] * rhs.m[0][c] +
					self.m[r][1] * rhs.m[1][c] +
					self.m[r][2] * rhs.m[2][c]
				;
			}
		}
		return ret;
	}
}

impl Mul for Matrix3x3 {
	type Output = Matrix3x3;
	#[inline(always)]
	fn mul(self, rhs: Matrix3x3) -> Matrix3x3 { (&self) * &rhs }
}

impl Mul<&Matrix3x3> for &Vec3 {
	type Output = Vec3;
	#[inline(always)]
	fn mul(self, matrix:&Matrix3x3) -> Vec3 {
		Vec3 {
			x: self.x * matrix.m[0][0] + self.y * matrix.m[1][0] + self.z * matrix.m[2][0],
			y: self.x * matrix.m[0][1] + self.y * matrix.m[1][1] + self.z * matrix.m[2][1],
			z: self.x * matrix.m[0][2] + self.y * matrix.m[1][2] + self.z * matrix.m[2][2],
		}
	}
}
impl Mul<&Matrix3x3> for  Vec3 { type Output = Vec3; #[inline(always)] fn mul(self, rhs:&Matrix3x3) -> Vec3 { (&self) *  rhs } }
impl Mul<&Vec3> for &Matrix3x3 { type Output = Vec3; #[inline(always)] fn mul(self, rhs:&Vec3     ) -> Vec3 {     rhs * self } }
impl Mul< Vec3> for &Matrix3x3 { type Output = Vec3; #[inline(always)] fn mul(self, rhs: Vec3     ) -> Vec3 {     rhs * self } }

pub struct Matrix4x4 {
	pub m:[[f32;4];4],
}
impl Matrix4x4 {
	pub const ZERO:Matrix4x4 = Matrix4x4 {
		m: [
			[0f32, 0f32, 0f32, 0f32],
			[0f32, 0f32, 0f32, 0f32],
			[0f32, 0f32, 0f32, 0f32],
			[0f32, 0f32, 0f32, 0f32]
		]
	};
	pub const IDENTITY:Matrix4x4 = Matrix4x4 {
		m: [
			[1f32, 0f32, 0f32, 0f32],
			[0f32, 1f32, 0f32, 0f32],
			[0f32, 0f32, 1f32, 0f32],
			[0f32, 0f32, 0f32, 1f32]
		]
	};
}

impl Mul for &Matrix4x4 {
	type Output = Matrix4x4;
	#[inline(always)]
	fn mul(self, rhs: &Matrix4x4) -> Matrix4x4 {
		let mut ret = Matrix4x4::ZERO;
		for c in 0..4 {
			for r in 0..4 {
				ret.m[r][c] =
					self.m[r][0] * rhs.m[0][c] +
					self.m[r][1] * rhs.m[1][c] +
					self.m[r][2] * rhs.m[2][c] +
					self.m[r][3] * rhs.m[3][c]
				;
			}
		}
		return ret;
	}
}

impl Mul for Matrix4x4 {
	type Output = Matrix4x4;
	#[inline(always)]
	fn mul(self, rhs: Matrix4x4) -> Matrix4x4 { (&self) * &rhs }
}

impl Mul<&Matrix4x4> for &Vec4 {
	type Output = Vec4;
	#[inline(always)]
	fn mul(self, matrix:&Matrix4x4) -> Vec4 {
		Vec4 {
			x: self.x * matrix.m[0][0] + self.y * matrix.m[1][0] + self.z * matrix.m[2][0] + self.w * matrix.m[3][0],
			y: self.x * matrix.m[0][1] + self.y * matrix.m[1][1] + self.z * matrix.m[2][1] + self.w * matrix.m[3][1],
			z: self.x * matrix.m[0][2] + self.y * matrix.m[1][2] + self.z * matrix.m[2][2] + self.w * matrix.m[3][2],
			w: self.x * matrix.m[0][3] + self.y * matrix.m[1][3] + self.z * matrix.m[2][3] + self.w * matrix.m[3][3],
		}
	}
}
impl Mul<&Matrix4x4> for  Vec4 { type Output = Vec4; #[inline(always)] fn mul(self, rhs:&Matrix4x4) -> Vec4 { (&self) *  rhs } }
impl Mul<&Vec4> for &Matrix4x4 { type Output = Vec4; #[inline(always)] fn mul(self, rhs:&Vec4     ) -> Vec4 {     rhs * self } }
impl Mul< Vec4> for &Matrix4x4 { type Output = Vec4; #[inline(always)] fn mul(self, rhs: Vec4     ) -> Vec4 {     rhs * self } }

impl Matrix3x3 {
	#[inline(always)]
	pub fn to_4x4(&self) -> Matrix4x4 {
		Matrix4x4 {
			m: [
				[self.m[0][0], self.m[0][1], self.m[0][2], 0f32],
				[self.m[1][0], self.m[1][1], self.m[1][2], 0f32],
				[self.m[2][0], self.m[2][1], self.m[2][2], 0f32],
				[        0f32,         0f32,         0f32, 0f32]
			]
		}
	}
}