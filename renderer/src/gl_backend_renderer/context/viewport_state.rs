use super::super::gl;

pub struct ViewportState {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl ViewportState {
    pub fn build_initialized() -> Self {
        let mut state = Self {
            x: 0,
            y: 0,
            width: 1,
            height: 1,
        };

        state.upload();

        state
    }

    pub fn set(&mut self, x: i32, y: i32, width: u32, height: u32) {
        self.x = x;
        self.y = y;
        self.width = width;
        self.height = height;

        self.upload();
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;

        self.upload();
    }

    fn upload(&self) {
        gl::viewport(self.x, self.y, self.width, self.height);
    }
}
