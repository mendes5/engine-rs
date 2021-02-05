use super::super::gl;

pub struct FrontFaceState(pub gl::GLenum);

impl FrontFaceState {
    pub fn build_initialized() -> Self {
        let state = Self(gl::CCW);

        state.upload();

        state
    }

    pub fn set(&mut self, face: gl::GLenum) {
        self.0 = face;
        self.upload();
    }

    fn upload(&self) {
        gl::front_face(self.0);
    }
}
