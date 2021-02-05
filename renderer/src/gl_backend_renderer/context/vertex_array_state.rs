use super::super::gl;

#[derive(Debug)]
pub struct GLVertexArray(pub gl::GLuint);

pub struct VertexArrayState {
    current_bound_buffer: Option<GLVertexArray>,
}

impl VertexArrayState {
    pub fn build_initialized() -> Self {
        Self {
            current_bound_buffer: None,
        }
    }

    pub fn create(&self) -> GLVertexArray {
        GLVertexArray(gl::gen_vertex_arrays(1))
    }

    pub fn bind(&mut self, buffer: &GLVertexArray) {
        self.current_bound_buffer = Some(GLVertexArray(buffer.0));

        gl::bind_vertex_array(buffer.0);
    }
}
