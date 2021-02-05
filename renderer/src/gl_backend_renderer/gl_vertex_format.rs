use lazy_static::lazy_static;
use regex::Regex;
use std::os::raw::c_void;

use super::gl;

lazy_static! {
    static ref EXTRACT_INPUTS: Regex =
        Regex::new(r"in\s+((?:[uib]?vec[234])|float)\s+(\w+)\s*;").unwrap();
}

#[macro_export]
macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        &(*(std::ptr::null() as *const $ty)).$field as *const _ as usize as *const c_void
    };
}

pub trait VertexAttribute {
    fn type_() -> gl::GLenum;
    fn normalized() -> gl::GLboolean;
    fn size() -> i32;
}

pub trait VertexFormat {
    fn size() -> usize;
    fn on_vertex_layout() -> Vec<(i32, u32, u8, *const c_void)>;
}

pub fn get_attribute_format<V: VertexAttribute>(
    offset: *const c_void,
) -> (i32, u32, u8, *const c_void) {
    (V::size(), V::type_(), V::normalized(), offset)
}

pub fn configure_vertex_attributes<V: VertexFormat>(program: gl::GLuint, items: Vec<&str>) {
    let vertex_size = V::size();
    for (i, (size, type_, normalized, stride)) in V::on_vertex_layout().iter().enumerate() {
        let x = gl::get_attrib_location(program, items[i]);
        gl::vertex_attrib_pointer(x, *size, *type_, *normalized, vertex_size, *stride);
        gl::enable_vertex_attrib_array(x);
    }
}

pub fn configure_vertex_attributes_from_source<V: VertexFormat>(
    program: gl::GLuint,
    string: String,
) {
    let vertex_size = V::size();

    let mut vec = Vec::<u32>::new();
    let captures = EXTRACT_INPUTS.captures_iter(&string);

    for cap in captures {
        vec.push(gl::get_attrib_location(program, &cap[2]));
    }

    for (i, (size, type_, normalized, stride)) in V::on_vertex_layout().iter().enumerate() {
        gl::vertex_attrib_pointer(vec[i], *size, *type_, *normalized, vertex_size, *stride);
        gl::enable_vertex_attrib_array(vec[i]);
    }
}

#[repr(C)]
pub struct FVec4 {
    x: f32,
    y: f32,
    z: f32,
}

impl VertexAttribute for FVec4 {
    fn type_() -> gl::GLenum {
        gl::FLOAT
    }
    fn normalized() -> gl::GLboolean {
        gl::FALSE
    }
    fn size() -> i32 {
        4
    }
}

#[repr(C)]
pub struct FVec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl VertexAttribute for FVec3 {
    fn type_() -> gl::GLenum {
        gl::FLOAT
    }
    fn normalized() -> gl::GLboolean {
        gl::FALSE
    }
    fn size() -> i32 {
        3
    }
}

#[repr(C)]
pub struct FVec2 {
    pub x: f32,
    pub y: f32,
}

impl VertexAttribute for FVec2 {
    fn type_() -> gl::GLenum {
        gl::FLOAT
    }
    fn normalized() -> gl::GLboolean {
        gl::FALSE
    }
    fn size() -> i32 {
        2
    }
}

#[repr(C)]
pub struct Float {
    pub x: f32,
}

impl VertexAttribute for Float {
    fn type_() -> gl::GLenum {
        gl::FLOAT
    }
    fn normalized() -> gl::GLboolean {
        gl::FALSE
    }
    fn size() -> i32 {
        1
    }
}

#[repr(C)]
pub struct IVec4 {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl VertexAttribute for IVec4 {
    fn type_() -> gl::GLenum {
        gl::INT
    }
    fn normalized() -> gl::GLboolean {
        gl::TRUE
    }
    fn size() -> i32 {
        4
    }
}

#[repr(C)]
pub struct IVec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl VertexAttribute for IVec3 {
    fn type_() -> gl::GLenum {
        gl::INT
    }
    fn normalized() -> gl::GLboolean {
        gl::TRUE
    }
    fn size() -> i32 {
        3
    }
}

#[repr(C)]
pub struct IVec2 {
    x: i32,
    y: i32,
}

impl VertexAttribute for IVec2 {
    fn type_() -> gl::GLenum {
        gl::INT
    }
    fn normalized() -> gl::GLboolean {
        gl::TRUE
    }
    fn size() -> i32 {
        2
    }
}

#[repr(C)]
pub struct Int {
    x: i32,
}

impl VertexAttribute for Int {
    fn type_() -> gl::GLenum {
        gl::INT
    }
    fn normalized() -> gl::GLboolean {
        gl::TRUE
    }
    fn size() -> i32 {
        1
    }
}

#[repr(C)]
pub struct UVec4 {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl VertexAttribute for UVec4 {
    fn type_() -> gl::GLenum {
        gl::UNSIGNED_INT
    }
    fn normalized() -> gl::GLboolean {
        gl::TRUE
    }
    fn size() -> i32 {
        4
    }
}

#[repr(C)]
pub struct UVec3 {
    x: u32,
    y: u32,
    z: u32,
}

impl VertexAttribute for UVec3 {
    fn type_() -> gl::GLenum {
        gl::UNSIGNED_INT
    }
    fn normalized() -> gl::GLboolean {
        gl::TRUE
    }
    fn size() -> i32 {
        3
    }
}

#[repr(C)]
pub struct UVec2 {
    x: u32,
    y: u32,
}

impl VertexAttribute for UVec2 {
    fn type_() -> gl::GLenum {
        gl::UNSIGNED_INT
    }
    fn normalized() -> gl::GLboolean {
        gl::TRUE
    }
    fn size() -> i32 {
        2
    }
}

#[repr(C)]
pub struct UInt {
    x: u32,
}

impl VertexAttribute for UInt {
    fn type_() -> gl::GLenum {
        gl::UNSIGNED_INT
    }
    fn normalized() -> gl::GLboolean {
        gl::TRUE
    }
    fn size() -> i32 {
        1
    }
}
