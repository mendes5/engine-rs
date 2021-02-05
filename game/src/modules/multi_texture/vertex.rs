use renderer::gl_vertex_format::{get_attribute_format, FVec2, FVec3, VertexFormat};
use renderer::offset_of;
use std::mem::size_of;
use std::os::raw::c_void;

#[repr(C)]
pub struct Vertex {
    position: FVec3,
    uv: FVec2,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, u: f32, v: f32) -> Self {
        Vertex {
            position: FVec3 { x, y, z },
            uv: FVec2 { x: u, y: v },
        }
    }
}

impl VertexFormat for Vertex {
    fn size() -> usize {
        size_of::<Self>()
    }

    fn on_vertex_layout() -> Vec<(i32, u32, u8, *const c_void)> {
        unsafe {
            vec![
                get_attribute_format::<FVec3>(offset_of!(Self, position)),
                get_attribute_format::<FVec2>(offset_of!(Self, uv)),
            ]
        }
    }
}
