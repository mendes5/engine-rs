use glutin::{ContextWrapper, PossiblyCurrent};
use std::ffi::CStr;
use std::os::raw::c_void;
use math::{Mat4, Vec2, Vec3, Vec4};
use winit::window::Window;

mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

pub use self::gl::types::*;
pub use self::gl::*;

#[inline]
pub fn get_program_iv(program: GLuint, pname: GLenum) -> GLint {
    unsafe {
        let mut params: GLint = 0;
        GetProgramiv(program, pname, &mut params);
        params
    }
}

#[inline]
pub fn get_shader_iv(program: GLuint, pname: GLenum) -> GLint {
    unsafe {
        let mut params: GLint = 0;
        GetShaderiv(program, pname, &mut params);
        params
    }
}

#[inline]
pub fn get_shader_info_log(shader: GLuint, buf_size: GLsizei) -> String {
    unsafe {
        let mut actual_length: GLint = 0;
        let mut buf: Vec<u8> = Vec::with_capacity(buf_size as usize);

        GetShaderInfoLog(
            shader,
            buf_size,
            &mut actual_length,
            buf.as_mut_ptr() as *mut _,
        );

        buf.set_len(actual_length as usize);
        String::from_utf8(buf).unwrap()
    }
}

#[inline]
pub fn get_program_info_log(program: GLuint, buf_size: GLsizei) -> String {
    unsafe {
        let mut actual_length: GLint = 0;
        let mut buf: Vec<u8> = Vec::with_capacity(buf_size as usize);

        GetProgramInfoLog(
            program,
            buf_size,
            &mut actual_length,
            buf.as_mut_ptr() as *mut _,
        );

        buf.set_len(actual_length as usize);
        String::from_utf8(buf).unwrap()
    }
}

#[inline]
pub fn attach_shader(program: GLuint, shader: GLuint) {
    unsafe {
        AttachShader(program, shader);
    }
}

#[inline]
pub fn link_program(program: GLuint) {
    unsafe {
        LinkProgram(program);
    }
}

#[inline]
pub fn compile_shader(shader: GLuint) {
    unsafe {
        CompileShader(shader);
    }
}

#[inline]
pub fn create_shader(xtype: GLenum) -> GLuint {
    unsafe { CreateShader(xtype) }
}

#[inline]
pub fn create_program() -> GLuint {
    unsafe { CreateProgram() }
}

#[inline]
pub fn shader_source(shader: GLuint, string: &String) {
    unsafe {
        let len: [GLint; 1] = [string.len() as GLint];

        ShaderSource(shader, 1, &(string.as_ptr() as *const _), len.as_ptr());
    }
}

#[inline]
pub fn buffer_data<T>(target: GLenum, data: &[T], usage: GLenum) {
    unsafe {
        BufferData(
            target,
            (data.len() * std::mem::size_of::<T>()) as GLsizeiptr,
            data.as_ptr() as *const _,
            usage,
        );
    }
}

#[inline]
pub fn get_attrib_location(program: GLuint, name: &str) -> GLuint {
    unsafe { GetAttribLocation(program, [name, "\0"].concat().as_ptr() as *const _) as GLuint }
}

#[inline]
pub fn enable_vertex_attrib_array(attrib_location: GLuint) {
    unsafe {
        EnableVertexAttribArray(attrib_location);
    }
}

#[inline]
pub fn vertex_attrib_pointer(
    index: GLuint,
    size: GLint,
    type_: GLenum,
    normalized: GLboolean,
    stride: usize,
    pointer: *const c_void,
) {
    unsafe { VertexAttribPointer(index, size, type_, normalized, stride as GLsizei, pointer) }
}

#[inline]
pub fn gen_vertex_arrays(count: i32) -> GLuint {
    unsafe {
        let mut vao = 0;
        GenVertexArrays(count, &mut vao);
        vao
    }
}

#[inline]
pub fn gen_buffers(count: i32) -> GLuint {
    unsafe {
        let mut vbo = 0;
        GenBuffers(count, &mut vbo);
        vbo
    }
}

#[inline]
pub fn bind_buffer(target: GLenum, buffer: GLuint) {
    unsafe {
        BindBuffer(target, buffer);
    }
}

#[inline]
pub fn bind_buffer_base(target: GLenum, binding: GLuint, buffer: GLuint) {
    unsafe {
        BindBufferBase(target, binding, buffer);
    }
}

#[inline]
pub fn bind_vertex_array(array: GLuint) {
    unsafe {
        BindVertexArray(array);
    }
}

#[inline]
pub fn use_program(program: GLuint) {
    unsafe {
        UseProgram(program);
    }
}

#[inline]
pub fn delete_shader(shader: GLuint) {
    unsafe {
        DeleteShader(shader);
    }
}
#[inline]
pub fn get_string(name: GLenum) -> String {
    unsafe {
        let data = CStr::from_ptr(GetString(name) as *const _)
            .to_bytes()
            .to_vec();
        String::from_utf8(data).unwrap()
    }
}

#[inline]
pub fn init_from_window(window_context: &ContextWrapper<PossiblyCurrent, Window>) {
    load_with(|ptr| window_context.context().get_proc_address(ptr) as *const _);
}

#[inline]
pub fn get_uniform_location(program: GLuint, name: &str) -> GLint {
    unsafe { GetUniformLocation(program, [name, "\0"].concat().as_ptr() as *const _) }
}

#[inline]
pub fn get_uniform_block_index(program: GLuint, name: &str) -> GLuint {
    unsafe { GetUniformBlockIndex(program, [name, "\0"].concat().as_ptr() as *const _) }
}

#[inline]
pub fn uniform_block_binding(program: GLuint, block_index: GLuint, block_binding: GLuint) {
    unsafe { UniformBlockBinding(program, block_index, block_binding) }
}

#[inline]
pub fn uniform_matrix_4f(location: GLint, value: &Mat4) {
    unsafe {
        UniformMatrix4fv(location, 1, FALSE, &value.elements[0] as *const _);
    }
}

#[inline]
pub fn uniform_1ui(location: GLint, value: GLuint) {
    unsafe {
        Uniform1ui(location, value);
    }
}

#[inline]
pub fn uniform_1i(location: GLint, value: GLint) {
    unsafe {
        Uniform1i(location, value);
    }
}

#[inline]
pub fn uniform_1f(location: GLint, value: GLfloat) {
    unsafe {
        Uniform1f(location, value);
    }
}

#[inline]
pub fn uniform_2f(location: GLint, value: &Vec2) {
    unsafe {
        Uniform2f(location, value.x, value.y);
    }
}

#[inline]
pub fn uniform_3f(location: GLint, value: &Vec3) {
    unsafe {
        Uniform3f(location, value.x, value.y, value.z);
    }
}

#[inline]
pub fn uniform_4f(location: GLint, value: &Vec4) {
    unsafe {
        Uniform4f(location, value.x, value.y, value.z, value.w);
    }
}

#[inline]
pub fn clear_color(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
    unsafe {
        ClearColor(red, green, blue, alpha);
    }
}

#[inline]
pub fn clear(mask: GLbitfield) {
    unsafe {
        Clear(mask);
    }
}

#[inline]
pub fn draw_arrays(mode: GLenum, first: GLint, count: usize) {
    unsafe {
        DrawArrays(mode, first, count as GLsizei);
    }
}

#[inline]
pub fn draw_elements(mode: GLenum, count: usize, type_: GLenum) {
    unsafe {
        // TODO:: Support offset?
        DrawElements(mode, count as GLsizei, type_, std::ptr::null());
    }
}

#[inline]
pub fn viewport(x: i32, y: i32, width: u32, height: u32) {
    unsafe {
        Viewport(x, y, width as i32, height as i32);
    }
}

#[inline]
pub fn enable(cap: GLenum) {
    unsafe {
        Enable(cap);
    }
}

#[inline]
pub fn disable(cap: GLenum) {
    unsafe {
        Disable(cap);
    }
}

#[inline]
pub fn depth_mask(cap: u8) {
    unsafe {
        DepthMask(cap);
    }
}

#[inline]
pub fn depth_func(cap: GLenum) {
    unsafe {
        DepthFunc(cap);
    }
}

#[inline]
pub fn clear_depth(cap: f64) {
    unsafe {
        ClearDepth(cap);
    }
}

#[inline]
pub fn front_face(cap: GLenum) {
    unsafe {
        FrontFace(cap);
    }
}

#[inline]
pub fn cull_face(cap: GLenum) {
    unsafe {
        CullFace(cap);
    }
}

#[inline]
pub fn gen_textures(count: GLint) -> GLuint {
    unsafe {
        let mut texture1 = 0;
        GenTextures(count, &mut texture1);
        texture1
    }
}

#[inline]
pub fn bind_texture(target: GLenum, texture: GLuint) {
    unsafe {
        BindTexture(target, texture);
    }
}

#[inline]
pub fn tex_parameteri(target: GLenum, pname: GLenum, param: GLenum) {
    unsafe {
        TexParameteri(target, pname, param as i32);
    }
}

#[inline]
pub fn generate_mipmap(target: GLenum) {
    unsafe {
        GenerateMipmap(target);
    }
}
#[inline]
pub fn tex_image_2d(
    target: GLenum,
    level: GLint,
    internal_format: GLenum,
    width: u32,
    height: u32,
    border: GLint,
    format: GLenum,
    type_: GLenum,
    pixels: &[u8],
) {
    unsafe {
        TexImage2D(
            target,
            level,
            internal_format as i32,
            width as i32,
            height as i32,
            border,
            format,
            type_,
            &pixels[0] as *const u8 as *const c_void,
        );
    }
}

#[inline]
pub fn tex_image_2d_raw(
    target: GLenum,
    level: GLint,
    internal_format: GLenum,
    width: u32,
    height: u32,
    border: GLint,
    format: GLenum,
    type_: GLenum,
    pixels: *const c_void,
) {
    unsafe {
        TexImage2D(
            target,
            level,
            internal_format as i32,
            width as i32,
            height as i32,
            border,
            format,
            type_,
            pixels,
        );
    }
}

#[inline]
pub fn active_texture(texture: GLenum) {
    unsafe {
        ActiveTexture(texture);
    }
}

#[inline]
pub fn get_integer_v(pname: GLenum) -> GLint {
    let mut value = 0;
    unsafe {
        GetIntegerv(pname, &mut value);
    }
    value
}

#[inline]
pub fn get_subroutine_index(program: GLuint, shader_type: GLenum, name: &str) -> GLuint {
    unsafe {
        GetSubroutineIndex(
            program,
            shader_type,
            [name, "\0"].concat().as_ptr() as *const _,
        )
    }
}

#[inline]
pub fn get_subroutine_uniform_location(program: GLuint, shader_type: GLenum, name: &str) -> GLint {
    unsafe {
        GetSubroutineUniformLocation(
            program,
            shader_type,
            [name, "\0"].concat().as_ptr() as *const _,
        )
    }
}

#[inline]
pub fn uniform_subroutines_uiv(shader_type: GLenum, count: GLint, data: &GLuint) {
    unsafe {
        UniformSubroutinesuiv(shader_type, count, data as *const _);
    }
}

#[inline]
pub fn blend_equation(func: GLenum) {
    unsafe {
        BlendEquation(func);
    }
}

#[inline]
pub fn blend_func(src: GLenum, dest: GLenum) {
    unsafe {
        BlendFunc(src, dest);
    }
}

#[inline]
pub fn texture_binding(binding: GLenum) -> i32 {
    (binding - gl::TEXTURE0) as i32
}
