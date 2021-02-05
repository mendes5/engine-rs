use super::super::gl;

#[derive(Debug)]
pub struct GLBufferBinding {
    target: gl::GLenum,
    usage: gl::GLenum,
}

#[derive(Debug)]
pub struct GLBuffer {
    target: gl::GLenum,
    usage: gl::GLenum,
    handle: gl::GLuint,
}

pub struct BufferState {
    current_bound_buffer: Option<GLBufferBinding>,
}

impl BufferState {
    pub fn build_initialized() -> Self {
        Self {
            current_bound_buffer: None,
        }
    }

    pub fn create_buffer(&self, target: gl::GLenum, usage: gl::GLenum) -> GLBuffer {
        let state = GLBuffer {
            target,
            usage,
            handle: gl::gen_buffers(1),
        };

        state
    }

    pub fn bind_buffer(&mut self, buffer: &GLBuffer) -> &mut Self {
        self.current_bound_buffer = Some(GLBufferBinding {
            target: buffer.target,
            usage: buffer.usage,
        });

        gl::bind_buffer(buffer.target, buffer.handle);

        self
    }

    pub fn bind_buffer_base(&mut self, bind_point: gl::GLuint, buffer: &GLBuffer) -> &mut Self {
        gl::bind_buffer_base(buffer.target, bind_point, buffer.handle);

        self
    }

    pub fn set_data<T>(&mut self, data: &[T]) -> &mut Self {
        if let Some(buffer) = &self.current_bound_buffer {
            gl::buffer_data(buffer.target, data, buffer.usage);
        }

        self
    }
}
