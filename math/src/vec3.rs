// REF:: https://github.com/mrdoob/three.js/blob/dev/src/math/Vector3.js

use super::{one_when_zero, Mat4, Quat, Vec2};

#[repr(C)]
#[derive(Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn from_components(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn from_scalar(s: f32) -> Self {
        Self { x: s, y: s, z: s }
    }

    pub fn unit_x() -> Self {
        Self {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn unit_y() -> Self {
        Self {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    pub fn unit_z() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }

    pub fn set(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    // Scalar Operations

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn multiply_scalar(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }

    pub fn divide_scalar(&mut self, scalar: f32) {
        self.multiply_scalar(1.0 / scalar);
    }

    pub fn normalize(&mut self) {
        self.divide_scalar(one_when_zero(self.length()));
    }

    // Cross

    pub fn from_cross(a: &Vec3, b: &Vec3) -> Self {
        Self::from_components(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }

    pub fn cross_with(&mut self, a: &Vec3) {
        let y = self.y * a.z - self.z * a.y;
        let z = self.z * a.x - self.x * a.z;
        let x = self.x * a.y - self.y * a.x;

        self.set(x, y, z);
    }

    pub fn cross_vectors(&mut self, a: &Vec3, b: &Vec3) {
        self.set(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        );
    }

    // Sphere

    pub fn to_sphere_coord(&mut self, v: &Vec2) {
        let (sy, cy) = v.y.sin_cos();
        let (sx, cx) = v.x.sin_cos();

        self.x = cy * cx;
        self.y = sy;
        self.z = cy * sx;
    }

    pub fn from_sphere_coord(v: &Vec2) -> Self {
        let mut vec = Self::new();
        vec.to_sphere_coord(v);
        vec
    }

    pub fn add(&mut self, v: &Vec3) {
        let x = self.x + v.x;
        let y = self.y + v.y;
        let z = self.z + v.z;

        self.set(x, y, z);
    }

    pub fn add_vectors(&mut self, a: &Vec3, b: &Vec3) {
        self.set(a.x + b.x, a.y + b.y, a.z + b.z);
    }

    pub fn sub_vectors(&mut self, a: &Vec3, b: &Vec3) {
        self.set(a.x - b.x, a.y - b.y, a.z - b.z);
    }

    pub fn from_sub_vectors(a: &Vec3, b: &Vec3) -> Self {
        let mut vec = Self::new();
        vec.sub_vectors(&a, &b);
        vec
    }

    pub fn dot(&self, v: &Vec3) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn dot_vectors(a: &Vec3, b: &Vec3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn from(v: &Vec3) -> Self {
        Self::from_components(v.x.clone(), v.y.clone(), v.z.clone())
    }

    pub fn copy_from(&mut self, v: &Vec3) {
        self.set(v.x.clone(), v.y.clone(), v.z.clone());
    }

    pub fn apply_quaternion(&mut self, q: &Quat) {
        // calculate quat * vector
        let ix = q.w * self.x + q.y * self.z - q.z * self.y;
        let iy = q.w * self.y + q.z * self.x - q.x * self.z;
        let iz = q.w * self.z + q.x * self.y - q.y * self.x;
        let iw = -q.x * self.x - q.y * self.y - q.z * self.z;

        // calculate result * inverse quat
        self.x = ix * q.w + iw * -q.x + iy * -q.z - iz * -q.y;
        self.y = iy * q.w + iw * -q.y + iz * -q.x - ix * -q.z;
        self.z = iz * q.w + iw * -q.z + ix * -q.y - iy * -q.x;
    }

    pub fn set_from_matrix_position(&mut self, m: &Mat4) {
        self.x = m.elements[12];
        self.y = m.elements[13];
        self.z = m.elements[14];
    }
}
