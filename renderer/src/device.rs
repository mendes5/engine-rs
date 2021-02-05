use super::*;
use generational_arena::{Arena, Index};
use glutin::{ContextWrapper, PossiblyCurrent};
use image::GenericImageView;
use lazy_static::lazy_static;
use std::path::Path;
use winit::window::Window;

const DEFAULT_TEXTURE_SIZE: u32 = 2;

lazy_static! {
    static ref DEFAULT_TEXTURE_DATA: Vec<u8> =
        vec![0, 0, 0, 128, 255, 0, 255, 128, 255, 0, 255, 128, 0, 0, 0, 128];
}

pub struct MeshFlags {
    depth_write: bool,
    mode: u32,
}

impl MeshFlags {
    pub fn new() -> Self {
        Self {
            depth_write: true,
            mode: gl::TRIANGLES,
        }
    }

    pub fn no_depth(mut self) -> MeshFlags {
        self.depth_write = false;
        self
    }

    pub fn triangles_mode(mut self) -> MeshFlags {
        self.mode = gl::TRIANGLES;
        self
    }

    pub fn lines_mode(mut self) -> MeshFlags {
        self.mode = gl::LINES;
        self
    }

    pub fn opt(self) -> Option<Self> {
        Some(self)
    }
}

impl Default for MeshFlags {
    fn default() -> Self {
        Self {
            depth_write: true,
            mode: gl::TRIANGLES,
        }
    }
}

pub struct RendererDevice {
    pub ctx: OpenGLContext,
    pub texture_cache: Arena<Texture>,
}

impl RendererDevice {
    pub fn from_window(window_context: &ContextWrapper<PossiblyCurrent, Window>) -> Self {
        let mut value = Self {
            ctx: OpenGLContext::build_initialize(window_context),
            texture_cache: Arena::new(),
        };

        value.resize(500, 500);
        value
    }

    pub fn register_texture(&mut self, texture_object: Texture) -> Index<Texture> {
        let mut texture_object = texture_object;
        let gl_texture = self.ctx.texture.create_texture(gl::TEXTURE_2D, None);
        self.ctx
            .texture
            .set_active_texture_unit(gl::TEXTURE0)
            .bind_texture(gl_texture)
            .set_wrappings(texture_object.wrapping as u32)
            .set_mig_mag_filter(texture_object.filtering as u32);

        texture_object.set_handle(gl_texture);

        self.texture_cache.insert(texture_object)
    }

    pub fn get_texture_mut(&mut self, index: Index<Texture>) -> Option<&mut Texture> {
        self.texture_cache.get_mut(index)
    }

    pub fn set_index_data<T>(&mut self, component: &mut RenderComponent, data: &[T]) {
        if let Some(ibo) = &component.ibo {
            self.ctx.buffer.bind_buffer(ibo).set_data(data);
        }
        if component.is_indexed {
            component.vertex_count = data.len();
        }
    }

    pub fn set_vertex_data<T>(&mut self, component: &mut RenderComponent, data: &[T]) {
        self.ctx.buffer.bind_buffer(&component.vbo).set_data(data);
        if !component.is_indexed {
            component.vertex_count = data.len();
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.ctx.viewport.resize(width, height);
    }

    pub fn new_mesh<T: VertexFormat>(
        &mut self,
        material_path: &Path,
        geometry: Vec<T>,
        index_buffer: Option<Vec<i32>>,
        textures: Vec<Index<Texture>>,
        flags: Option<MeshFlags>,
    ) -> RenderComponent {
        let flags = flags.unwrap_or_default();
        let vao = self.ctx.vertex_array.create();
        self.ctx.vertex_array.bind(&vao);

        let vb = self
            .ctx
            .buffer
            .create_buffer(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
        self.ctx.buffer.bind_buffer(&vb).set_data(&geometry);

        let (ib, size) = if let Some(index_buffer) = index_buffer {
            let ib = self
                .ctx
                .buffer
                .create_buffer(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
            self.ctx.buffer.bind_buffer(&ib).set_data(&index_buffer);

            (Some(ib), index_buffer.len())
        } else {
            (None, geometry.len())
        };

        let program = self.ctx.program.create_from_file::<T>(material_path);

        RenderComponent::new(
            program,
            textures,
            vao,
            vb,
            ib,
            flags.depth_write,
            flags.mode,
            size,
        )
    }

    pub fn render_component(&mut self, component: &mut RenderComponent) {
        self.ctx.program.bind(&component.material);
        self.ctx.vertex_array.bind(&component.vao);
        self.ctx.depth_buffer.set_mask(component.depth_write as u8);

        let mut index = 0;
        for text_id in component.textures.iter() {
            let text = if let Some(text) = self.texture_cache.get_mut(*text_id) {
                text
            } else {
                continue;
            };

            if let Some(txt) = text.handle {
                self.ctx
                    .texture
                    .set_active_texture_unit(gl::TEXTURE0 + index)
                    .bind_texture(txt);
            } else {
                continue;
            };

            if text.needs_update {
                match &mut text.storage {
                    TextureStorage::Canvas2D(canvas) => {
                        self.ctx.texture.set_raw_data(
                            canvas.width as u32,
                            canvas.height as u32,
                            canvas.data(),
                        );
                    }
                    TextureStorage::Image(image) => {
                        let data = image.raw_pixels();
                        let (width, height) = (image.width(), image.height());

                        self.ctx.texture.set_data(width, height, &data);
                    }
                    TextureStorage::Zeroed => {
                        self.ctx.texture.set_data(
                            DEFAULT_TEXTURE_SIZE,
                            DEFAULT_TEXTURE_SIZE,
                            &DEFAULT_TEXTURE_DATA,
                        );
                    }
                }
            }
            text.needs_update = false;
            index += 1;
        }

        if component.is_indexed {
            gl::draw_elements(
                component.draw_mode,
                component.vertex_count,
                gl::UNSIGNED_INT,
            );
        } else {
            gl::draw_arrays(component.draw_mode, 0, component.vertex_count);
        }
    }
}
