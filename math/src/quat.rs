// REF:: https://github.com/mrdoob/three.js/blob/dev/src/math/Quaternion.js
use super::{Mat4, Vec3};

pub struct Quat {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}

impl Quat {
	pub fn new() -> Self {
		Self {
			x: 0.0,
			y: 0.0,
			z: 0.0,
			w: 1.0,
		}
	}

	pub fn set(&mut self, x: f32, y: f32, z: f32, w: f32) {
		self.x = x;
		self.y = y;
		self.z = z;
		self.w = w;
	}

	pub fn copy_from(&mut self, q: &Quat) {
		self.x = q.x;
		self.y = q.y;
		self.z = q.z;
		self.w = q.w;
	}

	pub fn from(q: &Quat) -> Self {
		Self {
			x: q.x,
			y: q.y,
			z: q.z,
			w: q.w,
		}
	}

	pub fn set_from_axis_angle(&mut self, axis: &Vec3, angle: f32) {
		// http://www.euclideanspace.com/maths/geometry/rotations/conversions/angleToQuaternion/index.htm

		// assumes axis is normalized

		let half_angle = angle / 2.0;
		let (s, c) = half_angle.sin_cos();

		self.x = axis.x * s;
		self.y = axis.y * s;
		self.z = axis.z * s;
		self.w = c;
	}

	pub fn multiply(&mut self, q: &Quat) {
		let x = self.x * q.w + self.w * q.x + self.y * q.z - self.z * q.y;
		let y = self.y * q.w + self.w * q.y + self.z * q.x - self.x * q.z;
		let z = self.z * q.w + self.w * q.z + self.x * q.y - self.y * q.x;
		let w = self.w * q.w - self.x * q.x - self.y * q.y - self.z * q.z;

		self.x = x;
		self.y = y;
		self.z = z;
		self.w = w;
	}

	pub fn premultiply(&mut self, q: &Quat) {
		let x = q.x * self.w + q.w * self.x + q.y * self.z - q.z * self.y;
		let y = q.y * self.w + q.w * self.y + q.z * self.x - q.x * self.z;
		let z = q.z * self.w + q.w * self.z + q.x * self.y - q.y * self.x;
		let w = q.w * self.w - q.x * self.x - q.y * self.y - q.z * self.z;

		self.x = x;
		self.y = y;
		self.z = z;
		self.w = w;
	}

	pub fn multiply_quaternions(&mut self, a: &Quat, b: &Quat) {
		// from http://www.euclideanspace.com/maths/algebra/realNormedAlgebra/quaternions/code/index.htm

		self.x = a.x * b.w + a.w * b.x + a.y * b.z - a.z * b.y;
		self.y = a.y * b.w + a.w * b.y + a.z * b.x - a.x * b.z;
		self.z = a.z * b.w + a.w * b.z + a.x * b.y - a.y * b.x;
		self.w = a.w * b.w - a.x * b.x - a.y * b.y - a.z * b.z;
	}

	pub fn set_from_rotation_matrix(&mut self, m: &Mat4) {
		// http://www.euclideanspace.com/maths/geometry/rotations/conversions/matrixToQuaternion/index.htm

		// assumes the upper 3x3 of m is a pure rotation matrix (i.e, unscaled)

		let m11 = m.elements[0];
		let m12 = m.elements[4];
		let m13 = m.elements[8];
		let m21 = m.elements[1];
		let m22 = m.elements[5];
		let m23 = m.elements[9];
		let m31 = m.elements[2];
		let m32 = m.elements[6];
		let m33 = m.elements[10];

		let trace = m11 + m22 + m33;

		if trace > 0.0 {
			let s = 0.5 / (trace + 1.0).sqrt();

			self.w = 0.25 / s;
			self.x = (m32 - m23) * s;
			self.y = (m13 - m31) * s;
			self.z = (m21 - m12) * s;
		} else if m11 > m22 && m11 > m33 {
			let s = 2.0 * (1.0 + m11 - m22 - m33).sqrt();

			self.w = (m32 - m23) / s;
			self.x = 0.25 * s;
			self.y = (m12 + m21) / s;
			self.z = (m13 + m31) / s;
		} else if m22 > m33 {
			let s = 2.0 * (1.0 + m22 - m11 - m33).sqrt();

			self.w = (m13 - m31) / s;
			self.x = (m12 + m21) / s;
			self.y = 0.25 * s;
			self.z = (m23 + m32) / s;
		} else {
			let s = 2.0 * (1.0 + m33 - m11 - m22).sqrt();

			self.w = (m21 - m12) / s;
			self.x = (m13 + m31) / s;
			self.y = (m23 + m32) / s;
			self.z = 0.25 * s;
		}
	}
}
