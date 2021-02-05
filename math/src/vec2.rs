// REF:: https://github.com/mrdoob/three.js/blob/dev/src/math/Vector2.js

#[repr(C)]
#[derive(Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn from_components(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn from_scalar(s: f32) -> Self {
        Self { x: s, y: s }
    }

    pub fn set(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}
