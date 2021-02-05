use super::gl;
use super::*;
use image::{DynamicImage, GenericImageView};
use std::ffi::c_void;
use std::path::Path;
use math::Mat4;
use generational_arena::Index;

#[derive(Clone, Copy)]
pub enum TextureWrapping {
    ClampToEdge = gl::CLAMP_TO_EDGE as isize,
}

#[derive(Clone, Copy)]
pub enum TextureFiltering {
    Smooth = gl::LINEAR as isize,
    Pixelated = gl::NEAREST as isize,
}

pub enum TextureStorage {
    Canvas2D(SkiaCanvas),
    Image(DynamicImage),
    Zeroed,
}

impl TextureStorage {
    pub fn from_canvas(width: u32, height: u32) -> TextureStorage {
        TextureStorage::Canvas2D(SkiaCanvas::new(500, 500))
    }
    pub fn from_image<P>(path: P) -> TextureStorage
    where
        P: AsRef<Path>,
    {
        TextureStorage::Image(image::open(path).unwrap())
    }
}

pub struct Texture {
    pub wrapping: TextureWrapping,
    pub filtering: TextureFiltering,
    pub storage: TextureStorage,
    pub needs_update: bool,
    pub handle: Option<GLTexture>,
}

impl Texture {
    pub fn new(wrapping: TextureWrapping, filtering: TextureFiltering) -> Self {
        Self {
            wrapping,
            filtering,
            storage: TextureStorage::Zeroed,
            needs_update: true,
            handle: None,
        }
    }

    pub fn new_initialized(
        wrapping: TextureWrapping,
        filtering: TextureFiltering,
        storage: TextureStorage,
    ) -> Self {
        Self {
            wrapping,
            filtering,
            storage,
            needs_update: true,
            handle: None,
        }
    }

    pub fn set_handle(&mut self, handle: GLTexture) {
        self.handle = Some(handle);
    }
}

pub struct RenderComponent {
    pub material: GLShader,
    pub textures: Vec<Index<Texture>>,
    pub vao: GLVertexArray,
    pub ibo: Option<GLBuffer>,
    pub vbo: GLBuffer,
    pub depth_write: bool,
    pub draw_mode: u32,
    pub is_indexed: bool,
    pub vertex_count: usize,
}

impl RenderComponent {
    pub fn new(
        material: GLShader,
        textures: Vec<Index<Texture>>,
        vao: GLVertexArray,
        vbo: GLBuffer,
        ibo: Option<GLBuffer>,
        depth_write: bool,
        draw_mode: u32,
        vertex_count: usize,
    ) -> Self {
        let is_indexed = ibo.is_some();
        let component = Self {
            material,
            textures,
            vao,
            ibo,
            vbo,
            depth_write,
            draw_mode,
            is_indexed,
            vertex_count,
        };

        component
    }
}
