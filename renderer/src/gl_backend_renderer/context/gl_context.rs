use super::super::gl;
use super::{
    BlendState, BufferState, ColorBufferState, CullFaceState, DepthBufferState, FrontFaceState,
    ProgramState, TextureState, VertexArrayState, ViewportState,
};
use glutin::{ContextWrapper, PossiblyCurrent};
use winit::window::Window;

pub struct OpenGLContext {
    pub color_buffer: ColorBufferState,
    pub depth_buffer: DepthBufferState,
    pub texture: TextureState,
    pub blend: BlendState,
    pub buffer: BufferState,
    pub vertex_array: VertexArrayState,
    pub program: ProgramState,
    pub viewport: ViewportState,
    pub culling: CullFaceState,
    pub front_face: FrontFaceState,
}

impl OpenGLContext {
    pub fn build_initialize(window_context: &ContextWrapper<PossiblyCurrent, Window>) -> Self {
        gl::init_from_window(window_context);

        let color_buffer = ColorBufferState::build_initialized();
        let depth_buffer = DepthBufferState::build_initialized();
        let texture = TextureState::build_initialized();
        let blend = BlendState::build_initialized();
        let buffer = BufferState::build_initialized();
        let vertex_array = VertexArrayState::build_initialized();
        let program = ProgramState::build_initialized();
        let viewport = ViewportState::build_initialized();
        let culling = CullFaceState::build_initialized();
        let front_face = FrontFaceState::build_initialized();

        Self {
            color_buffer,
            depth_buffer,
            texture,
            blend,
            buffer,
            vertex_array,
            program,
            viewport,
            culling,
            front_face,
        }
    }

    pub fn clear_buffers(&self, mask: gl::GLbitfield) {
        gl::clear(mask);
    }

    pub fn reset_state(&mut self) {
        self.front_face.set(gl::CW);
        self.culling.set_enabled(false);
        self.color_buffer.set(0.0, 0.0, 0.0, 1.0);
        self.clear_buffers(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        self.depth_buffer.set_enabled(true);
        self.blend.set_enabled(true).configure(
            gl::FUNC_ADD,
            gl::SRC_ALPHA,
            gl::ONE_MINUS_SRC_ALPHA,
        );
    }
}
