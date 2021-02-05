// REF:: https://github.com/mrdoob/three.js/blob/dev/src/math/Matrix4.js
use super::{Quat, Vec3};

#[derive(Debug)]
pub struct Mat4 {
    pub elements: [f32; 16],
}

impl Mat4 {
    pub fn new() -> Self {
        Self {
            #[rustfmt::skip]
            elements: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn to_perspective(
        &mut self,
        left: f32,
        right: f32,
        top: f32,
        bottom: f32,
        near: f32,
        far: f32,
    ) {
        let x = 2.0 * near / (right - left);
        let y = 2.0 * near / (top - bottom);

        let a = (right + left) / (right - left);
        let b = (top + bottom) / (top - bottom);
        let c = -(far + near) / (far - near);
        let d = -2.0 * far * near / (far - near);

        // 1st column
        self.elements[0] = x;
        self.elements[4] = 0.0;
        self.elements[8] = a;
        self.elements[12] = 0.0;

        // 2nd column
        self.elements[1] = 0.0;
        self.elements[5] = y;
        self.elements[9] = b;
        self.elements[13] = 0.0;

        // 3rd column
        self.elements[2] = 0.0;
        self.elements[6] = 0.0;
        self.elements[10] = c;
        self.elements[14] = d;

        // 4th column
        self.elements[3] = 0.0;
        self.elements[7] = 0.0;
        self.elements[11] = -1.0;
        self.elements[15] = 0.0;
    }

    pub fn from_scale(scale: f32) -> Self {
        Self {
            #[rustfmt::skip]
            elements: [
                scale, 0.0, 0.0, 0.0,
                0.0, scale, 0.0, 0.0,
                0.0, 0.0, scale, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn from_x_rotation(rad: f32) -> Self {
        let s = rad.sin();
        let c = rad.cos();
        Self {
            #[rustfmt::skip]
            elements: [
                1.0, 0.0, 0.0, 0.0,
                0.0, c, s, 0.0,
                0.0, -s, c, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn from_y_rotation(rad: f32) -> Self {
        let s = rad.sin();
        let c = rad.cos();

        Self {
            #[rustfmt::skip]
            elements: [
                c, 0.0, -s, 0.0,
                0.0, 1.0, 0.0, 0.0,
                s, 0.0, c, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn to_projection_matrix(&mut self, near: f32, far: f32, fov: f32, aspect: f32, zoom: f32) {
        let top = near * (0.5 * fov).tan() / zoom;

        let height = 2.0 * top;
        let width = aspect * height;
        let left = -0.5 * width;

        self.to_perspective(left, left + width, top, top - height, near, far);
    }

    pub fn look_at(&mut self, eye: &Vec3, center: &Vec3, up: &Vec3) {
        let mut dir = Vec3::from_sub_vectors(center, eye);
        dir.normalize();

        let f = Vec3::from(&dir);

        let mut s = Vec3::from_cross(&f, &up);
        s.normalize();

        let u = Vec3::from_cross(&s, &f);

        self.set_common(
            // 1st row
            s.x,
            u.x,
            -f.x,
            // 2nd row
            s.y,
            u.y,
            -f.y,
            // 3rd row
            s.z,
            u.z,
            -f.z,
            // 4th row
            -eye.dot(&s),
            -eye.dot(&u),
            eye.dot(&f),
        );
    }

    pub fn look_at_target(&mut self, eye: &Vec3, center: &Vec3, up: &Vec3) {
        let mut dir = Vec3::from(center);
        dir.normalize();

        let f = Vec3::from(&dir);

        let mut s = Vec3::from_cross(&f, &up);
        s.normalize();

        let u = Vec3::from_cross(&s, &f);

        self.set_common(
            // 1st row
            s.x,
            u.x,
            -f.x,
            // 2nd row
            s.y,
            u.y,
            -f.y,
            // 3rd row
            s.z,
            u.z,
            -f.z,
            // 4th row
            -eye.dot(&s),
            -eye.dot(&u),
            eye.dot(&f),
        );
    }

    pub fn set_position(&mut self, v: &Vec3) {
        self.elements[12] = v.x;
        self.elements[13] = v.y;
        self.elements[14] = v.z;
    }

    pub fn set_common(
        &mut self,
        // 1st row
        r0c0: f32,
        r0c1: f32,
        r0c2: f32,
        // 2nd row
        r1c0: f32,
        r1c1: f32,
        r1c2: f32,
        // 3rd row
        r2c0: f32,
        r2c1: f32,
        r2c2: f32,
        // 4th row
        r3c0: f32,
        r3c1: f32,
        r3c2: f32,
    ) {
        // 1st row
        self.elements[0] = r0c0;
        self.elements[1] = r0c1;
        self.elements[2] = r0c2;
        // 2nd row
        self.elements[4] = r1c0;
        self.elements[5] = r1c1;
        self.elements[6] = r1c2;
        // 3rd row
        self.elements[8] = r2c0;
        self.elements[9] = r2c1;
        self.elements[10] = r2c2;
        // 4th row
        self.elements[12] = r3c0;
        self.elements[13] = r3c1;
        self.elements[14] = r3c2;
    }

    pub fn copy_from(&mut self, m: &Mat4) {
        // 1st row
        self.elements[0] = m.elements[0];
        self.elements[1] = m.elements[1];
        self.elements[2] = m.elements[2];
        // 2nd row
        self.elements[4] = m.elements[4];
        self.elements[5] = m.elements[5];
        self.elements[6] = m.elements[6];
        // 3rd row
        self.elements[8] = m.elements[8];
        self.elements[9] = m.elements[9];
        self.elements[10] = m.elements[10];
        // 4th row
        self.elements[12] = m.elements[12];
        self.elements[13] = m.elements[13];
        self.elements[14] = m.elements[14];
    }

    pub fn compose(&mut self, position: &Vec3, quaternion: &Quat, scale: &Vec3) {
        let x = quaternion.x;
        let y = quaternion.y;
        let z = quaternion.z;
        let w = quaternion.w;

        let x2 = x + x;
        let y2 = y + y;
        let z2 = z + z;
        let xx = x * x2;
        let xy = x * y2;
        let xz = x * z2;
        let yy = y * y2;
        let yz = y * z2;
        let zz = z * z2;
        let wx = w * x2;
        let wy = w * y2;
        let wz = w * z2;

        let sx = scale.x;
        let sy = scale.y;
        let sz = scale.z;

        self.elements[0] = (1.0 - (yy + zz)) * sx;
        self.elements[1] = (xy + wz) * sx;
        self.elements[2] = (xz - wy) * sx;
        self.elements[3] = 0.0;

        self.elements[4] = (xy - wz) * sy;
        self.elements[5] = (1.0 - (xx + zz)) * sy;
        self.elements[6] = (yz + wx) * sy;
        self.elements[7] = 0.0;

        self.elements[8] = (xz + wy) * sz;
        self.elements[9] = (yz - wx) * sz;
        self.elements[10] = (1.0 - (xx + yy)) * sz;
        self.elements[11] = 0.0;

        self.elements[12] = position.x;
        self.elements[13] = position.y;
        self.elements[14] = position.z;
        self.elements[15] = 1.0;
    }

    pub fn multiply_matrices(&mut self, a: &Mat4, b: &Mat4) {
        let ae = a.elements;
        let be = b.elements;

        let a11 = ae[0];
        let a12 = ae[4];
        let a13 = ae[8];
        let a14 = ae[12];

        let a21 = ae[1];
        let a22 = ae[5];
        let a23 = ae[9];
        let a24 = ae[13];

        let a31 = ae[2];
        let a32 = ae[6];
        let a33 = ae[10];
        let a34 = ae[14];

        let a41 = ae[3];
        let a42 = ae[7];
        let a43 = ae[11];
        let a44 = ae[15];

        let b11 = be[0];
        let b12 = be[4];
        let b13 = be[8];
        let b14 = be[12];

        let b21 = be[1];
        let b22 = be[5];
        let b23 = be[9];
        let b24 = be[13];

        let b31 = be[2];
        let b32 = be[6];
        let b33 = be[10];
        let b34 = be[14];

        let b41 = be[3];
        let b42 = be[7];
        let b43 = be[11];
        let b44 = be[15];

        self.elements[0] = a11 * b11 + a12 * b21 + a13 * b31 + a14 * b41;
        self.elements[4] = a11 * b12 + a12 * b22 + a13 * b32 + a14 * b42;
        self.elements[8] = a11 * b13 + a12 * b23 + a13 * b33 + a14 * b43;
        self.elements[12] = a11 * b14 + a12 * b24 + a13 * b34 + a14 * b44;

        self.elements[1] = a21 * b11 + a22 * b21 + a23 * b31 + a24 * b41;
        self.elements[5] = a21 * b12 + a22 * b22 + a23 * b32 + a24 * b42;
        self.elements[9] = a21 * b13 + a22 * b23 + a23 * b33 + a24 * b43;
        self.elements[13] = a21 * b14 + a22 * b24 + a23 * b34 + a24 * b44;

        self.elements[2] = a31 * b11 + a32 * b21 + a33 * b31 + a34 * b41;
        self.elements[6] = a31 * b12 + a32 * b22 + a33 * b32 + a34 * b42;
        self.elements[10] = a31 * b13 + a32 * b23 + a33 * b33 + a34 * b43;
        self.elements[14] = a31 * b14 + a32 * b24 + a33 * b34 + a34 * b44;

        self.elements[3] = a41 * b11 + a42 * b21 + a43 * b31 + a44 * b41;
        self.elements[7] = a41 * b12 + a42 * b22 + a43 * b32 + a44 * b42;
        self.elements[11] = a41 * b13 + a42 * b23 + a43 * b33 + a44 * b43;
        self.elements[15] = a41 * b14 + a42 * b24 + a43 * b34 + a44 * b44;
    }
}
