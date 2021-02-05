use renderer::gl_vertex_format::{get_attribute_format, FVec2, FVec3, VertexFormat};
use renderer::offset_of;
use std::mem::size_of;
use std::os::raw::c_void;

#[repr(C)]
pub struct Vertex {
    position: FVec3,
    color: FVec3,
}



impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, r: f32, g: f32, b: f32) -> Self {
        Self {
            position: FVec3 { x, y, z },
            color: FVec3 { x: r, y: g, z: b },
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
                get_attribute_format::<FVec3>(offset_of!(Self, color)),
            ]
        }
    }
}


#[repr(C)]
pub struct Vertex2 {
    position: FVec3,
    uv: FVec2,
}

impl Vertex2 {
    pub fn new(x: f32, y: f32, z: f32, u: f32, v: f32) -> Self {
        Vertex2 {
            position: FVec3 { x, y, z },
            uv: FVec2 { x: u, y: v },
        }
    }
}

impl VertexFormat for Vertex2 {
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
