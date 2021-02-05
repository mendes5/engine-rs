use super::super::gl;

pub struct CullFaceState {
    cull_face: gl::GLenum,
    enabled: bool,
}

impl CullFaceState {
    pub fn build_initialized() -> Self {
        let state = Self {
            cull_face: gl::FRONT_AND_BACK,
            enabled: true,
        };

        state.upload();

        state
    }

    pub fn set_mode(&mut self, cull_face: gl::GLenum) -> &mut Self {
        self.cull_face = cull_face;
        gl::cull_face(self.cull_face);

        self
    }

    pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
        self.enabled = enabled;
        if self.enabled {
            gl::enable(gl::CULL_FACE);
        } else {
            gl::disable(gl::CULL_FACE);
        }

        self
    }

    pub fn upload(&self) {
        gl::cull_face(self.cull_face);

        if self.enabled {
            gl::enable(gl::CULL_FACE);
        } else {
            gl::disable(gl::CULL_FACE);
        }
    }
}
