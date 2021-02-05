use super::super::gl;

pub struct DepthBufferState {
    depth_mask: u8,
    depth_func: gl::GLenum,
    depth_clear: f64,
    depth_test: bool,
}

impl DepthBufferState {
    pub fn build_initialized() -> Self {
        let mut state = Self {
            depth_mask: 0,
            depth_func: gl::LEQUAL,
            depth_clear: 0.0,
            depth_test: false,
        };

        state.set_func(gl::LEQUAL);
        state.set_enabled(false);

        state
    }

    pub fn set_mask(&mut self, mask: u8) {
        self.depth_mask = mask;
        gl::depth_mask(self.depth_mask);
    }

    pub fn set_func(&mut self, func: gl::GLenum) {
        self.depth_func = func;
        gl::depth_func(func);
    }

    pub fn set_clear(&mut self, clear: f64) {
        self.depth_clear = clear;
        gl::clear_depth(self.depth_clear);
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.depth_test = enabled;

        if self.depth_test {
            gl::enable(gl::DEPTH_TEST);
        } else {
            gl::disable(gl::DEPTH_TEST);
        }
    }
}
