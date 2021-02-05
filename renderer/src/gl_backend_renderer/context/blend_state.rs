use super::super::gl;

pub struct BlendState {
    enabled: bool,
    equation: gl::GLenum,
    blend_func_source: gl::GLenum,
    blend_func_destination: gl::GLenum,
}

impl BlendState {
    pub fn build_initialized() -> Self {
        let mut state = Self {
            enabled: false,
            equation: gl::FUNC_ADD,
            blend_func_source: gl::SRC_ALPHA,
            blend_func_destination: gl::DST_ALPHA,
        };

        state
            .set_enabled(false)
            .configure(gl::FUNC_ADD, gl::SRC_ALPHA, gl::DST_ALPHA);

        state
    }

    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        if enabled {
            gl::enable(gl::BLEND);
        } else {
            gl::disable(gl::BLEND);
        }

        self
    }

    pub fn configure(
        &mut self,
        equation: gl::GLenum,
        blend_func_source: gl::GLenum,
        blend_func_destination: gl::GLenum,
    ) -> &mut Self {
        gl::blend_equation(equation);
        gl::blend_func(blend_func_source, blend_func_destination);

        self
    }
}
