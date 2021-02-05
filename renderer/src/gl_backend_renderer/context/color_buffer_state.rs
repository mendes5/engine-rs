use super::super::gl;

pub struct ColorBufferState {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl ColorBufferState {
    pub fn build_initialized() -> Self {
        let mut state = Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        };

        state.reset();

        state
    }

    pub fn set(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.r = r;
        self.g = g;
        self.b = b;
        self.a = a;

        gl::clear_color(self.r, self.g, self.b, self.a);
    }

    pub fn reset(&mut self) {
        self.r = 0.0;
        self.g = 0.0;
        self.b = 0.0;
        self.a = 1.0;

        gl::clear_color(self.r, self.g, self.b, self.a);
    }
}
