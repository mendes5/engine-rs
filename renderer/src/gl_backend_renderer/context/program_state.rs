use super::super::gl;
use super::super::{configure_vertex_attributes_from_source, VertexFormat};
use std::fs;
use std::path::Path;
use math::{Mat4, Vec2, Vec3, Vec4};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref EXTRACT_TEXTURE_UNIFORMS: Regex =
        Regex::new(r"uniform\s+sampler2D\s+(\w+)\s*;").unwrap();
}

fn create_program(vertex: gl::GLuint, fragment: gl::GLuint) -> Result<gl::GLuint, String> {
    let program = gl::create_program();
    gl::attach_shader(program, vertex);
    gl::attach_shader(program, fragment);
    gl::link_program(program);

    let success = gl::get_program_iv(program, gl::LINK_STATUS);

    if success == i32::from(gl::TRUE) {
        Ok(program)
    } else {
        Err(get_program_info_log(program))
    }
}

fn get_program_info_log(program: gl::GLuint) -> String {
    let length = gl::get_program_iv(program, gl::INFO_LOG_LENGTH);

    gl::get_program_info_log(program, length)
}

fn get_shader_info_log(shader: gl::GLuint) -> String {
    let length = gl::get_shader_iv(shader, gl::INFO_LOG_LENGTH);
    gl::get_shader_info_log(shader, length)
}

fn create_shader(code: &String, xtype: gl::GLuint) -> Result<gl::GLuint, String> {
    let vs = gl::create_shader(xtype);
    gl::shader_source(vs, &code);
    gl::compile_shader(vs);

    let success = gl::get_shader_iv(vs, gl::COMPILE_STATUS);

    if success == gl::GLint::from(gl::TRUE) {
        Ok(vs)
    } else {
        let log = get_shader_info_log(vs);
        gl::delete_shader(vs);

        Err(log)
    }
}

pub struct GLShaderVariable<T> {
    pub value: gl::GLint,
    data: std::marker::PhantomData<T>,
}

pub struct GLUniformBlockIndex<T> {
    pub value: gl::GLuint,
    data: std::marker::PhantomData<T>,
}

impl<T> std::fmt::Debug for GLShaderVariable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<GLShaderVariable@{:?}>", self.value)
    }
}

impl<T> std::fmt::Debug for GLUniformBlockIndex<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<GLUniformBlockBinding@{:?}>", self.value)
    }
}

impl<T> GLShaderVariable<T> {
    fn from(value: gl::GLint) -> Self {
        Self {
            value,
            data: std::marker::PhantomData,
        }
    }

    pub fn uninitialized() -> Self {
        Self {
            value: 0,
            data: std::marker::PhantomData,
        }
    }
}

impl<T> GLUniformBlockIndex<T> {
    fn from(value: gl::GLuint) -> Self {
        GLUniformBlockIndex {
            value,
            data: std::marker::PhantomData,
        }
    }

    pub fn uninitialized() -> Self {
        GLUniformBlockIndex {
            value: 0,
            data: std::marker::PhantomData,
        }
    }
}

pub trait ShaderVariable<T> {
    fn set(&self, val: &T);
}

impl ShaderVariable<i32> for GLShaderVariable<i32> {
    fn set(&self, value: &i32) {
        gl::uniform_1i(self.value, *value);
    }
}

impl ShaderVariable<u32> for GLShaderVariable<u32> {
    fn set(&self, value: &u32) {
        gl::uniform_1ui(self.value, *value);
    }
}

impl ShaderVariable<f32> for GLShaderVariable<f32> {
    fn set(&self, value: &f32) {
        gl::uniform_1f(self.value, *value);
    }
}

impl ShaderVariable<Vec2> for GLShaderVariable<Vec2> {
    fn set(&self, value: &Vec2) {
        gl::uniform_2f(self.value, value);
    }
}

impl ShaderVariable<Vec3> for GLShaderVariable<Vec3> {
    fn set(&self, value: &Vec3) {
        gl::uniform_3f(self.value, value);
    }
}

impl ShaderVariable<Vec4> for GLShaderVariable<Vec4> {
    fn set(&self, value: &Vec4) {
        gl::uniform_4f(self.value, value);
    }
}

impl ShaderVariable<Mat4> for GLShaderVariable<Mat4> {
    fn set(&self, value: &Mat4) {
        gl::uniform_matrix_4f(self.value, &value);
    }
}

#[derive(Debug)]
pub struct GLShader(pub gl::GLuint);

impl GLShader {
    fn from<V: VertexFormat>(vertex_shader: String, fragment_shader: String) -> Self {
        let vs = create_shader(&vertex_shader, gl::VERTEX_SHADER)
            .expect(&format!("Failed to compile vertex shader \n\n{} \n\n", vertex_shader)[..]);

        let fs = create_shader(&fragment_shader, gl::FRAGMENT_SHADER).expect(
            &format!(
                "Failed to compile fragment shader \n\n{}\n\n",
                fragment_shader
            )[..],
        );

        let program = create_program(vs, fs).expect("Failed to link");
        gl::delete_shader(vs);
        gl::delete_shader(fs);

        gl::use_program(program);

        configure_vertex_attributes_from_source::<V>(program, vertex_shader);

        let program = GLShader(program);

        let mut index: u32 = 0;
        for capture in EXTRACT_TEXTURE_UNIFORMS.captures_iter(&fragment_shader) {
            let uniform_variable = program.get_variable::<i32>(&capture[1]);
            uniform_variable.set(&gl::texture_binding(gl::TEXTURE0 + index));
            index += 1;
        }

        program
    }

    fn from_file<V: VertexFormat>(path: &Path) -> Self {
        let contents = fs::read_to_string(path).unwrap();
        let shaders: Vec<&str> = contents.split_terminator("#pragma SHADER").collect();

        // First item is an empty string
        assert_eq!(shaders.len(), 3);

        Self::from::<V>(String::from(shaders[1]), String::from(shaders[2]))
    }

    pub fn get_variable<T>(&self, name: &str) -> GLShaderVariable<T> {
        GLShaderVariable::<T>::from(gl::get_uniform_location(self.0, name))
    }

    pub fn get_uniform_block_index<T>(&self, name: &str) -> GLUniformBlockIndex<T> {
        let x = gl::get_uniform_block_index(self.0, name);
        GLUniformBlockIndex::<T>::from(x)
    }

    pub fn uniform_block_binding<T>(
        &self,
        uniform_block_index: &GLUniformBlockIndex<T>,
        bind_point: gl::GLuint,
    ) {
        gl::uniform_block_binding(self.0, uniform_block_index.value, bind_point);
    }
}

pub struct ProgramState {
    current_bound_shader: Option<u32>,
}

impl ProgramState {
    pub fn build_initialized() -> Self {
        Self {
            current_bound_shader: None,
        }
    }

    pub fn create_from_strings<V: VertexFormat>(
        &self,
        vertex_shader: String,
        fragment_shader: String,
    ) -> GLShader {
        GLShader::from::<V>(vertex_shader, fragment_shader)
    }

    pub fn create_from_file<V: VertexFormat>(&self, path: &Path) -> GLShader {
        GLShader::from_file::<V>(path)
    }

    pub fn bind(&mut self, shader: &GLShader) {
        self.current_bound_shader = Some(shader.0);

        gl::use_program(shader.0);
    }
}
