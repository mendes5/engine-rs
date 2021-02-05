use super::super::gl;
use std::os::raw::c_void;

#[derive(Copy, Clone, Debug)]
pub struct TextureFormat {
    level: gl::GLint,
    internal_format: gl::GLenum,
    format: gl::GLenum,
    type_: gl::GLenum,
}

impl Default for TextureFormat {
    fn default() -> Self {
        Self {
            level: 0,
            internal_format: gl::RGBA,
            format: gl::RGBA,
            type_: gl::UNSIGNED_BYTE,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GLTexture {
    texture: gl::GLuint,
    target: gl::GLenum,
    level: gl::GLint,
    internal_format: gl::GLenum,
    format: gl::GLenum,
    type_: gl::GLenum,
}

struct TextureUnit {
    handle: gl::GLuint,
    target: gl::GLenum,
}

pub struct TextureState {
    active_texture_unit: u32,
    bound_texture: Option<GLTexture>,
    bound_texture_units: Vec<TextureUnit>,
    max_texture_units: i32,
}

impl TextureState {
    pub fn build_initialized() -> Self {
        let max_texture_units = gl::get_integer_v(gl::MAX_COMBINED_TEXTURE_IMAGE_UNITS);
        let active_texture_unit = 0;
        let bound_texture = None;
        let mut bound_texture_units = Vec::with_capacity(max_texture_units as usize);

        for _ in 0..max_texture_units {
            bound_texture_units.push(TextureUnit {
                target: 0,
                handle: 0,
            });
        }

        let mut state = Self {
            active_texture_unit,
            bound_texture,
            bound_texture_units,
            max_texture_units,
        };

        state.set_active_texture_unit(gl::TEXTURE0);

        state
    }

    pub fn set_wrapping(&mut self, wrap_s: gl::GLenum, wrap_t: gl::GLenum) -> &mut Self {
        if let Some(texture) = &self.bound_texture {
            gl::tex_parameteri(texture.target, gl::TEXTURE_WRAP_S, wrap_s);
            gl::tex_parameteri(texture.target, gl::TEXTURE_WRAP_T, wrap_t);
        }

        self
    }

    pub fn set_wrappings(&mut self, wrap: gl::GLenum) -> &mut Self {
        if let Some(texture) = &self.bound_texture {
            gl::tex_parameteri(texture.target, gl::TEXTURE_WRAP_S, wrap);
            gl::tex_parameteri(texture.target, gl::TEXTURE_WRAP_T, wrap);
        }

        self
    }

    pub fn set_mig_mag_filters(&mut self, min: gl::GLenum, mag: gl::GLenum) -> &mut Self {
        if let Some(texture) = &self.bound_texture {
            gl::tex_parameteri(texture.target, gl::TEXTURE_MIN_FILTER, min);
            gl::tex_parameteri(texture.target, gl::TEXTURE_MAG_FILTER, mag);
        }

        self
    }

    pub fn set_mig_mag_filter(&mut self, filter: gl::GLenum) -> &mut Self {
        if let Some(texture) = &self.bound_texture {
            gl::tex_parameteri(texture.target, gl::TEXTURE_MIN_FILTER, filter);
            gl::tex_parameteri(texture.target, gl::TEXTURE_MAG_FILTER, filter);
        }

        self
    }

    pub fn set_raw_data(&mut self, width: u32, height: u32, data: *const c_void) -> &mut Self {
        if let Some(texture) = &self.bound_texture {
            gl::tex_image_2d_raw(
                texture.target,
                texture.level,
                texture.internal_format,
                width,
                height,
                0,
                texture.format,
                texture.type_,
                data,
            );

            if texture.level > 0 {
                gl::generate_mipmap(self.active_texture_unit);
            }
        }

        self
    }

    pub fn set_data(&mut self, width: u32, height: u32, data: &[u8]) -> &mut Self {
        if let Some(texture) = &self.bound_texture {
            gl::tex_image_2d(
                texture.target,
                texture.level,
                texture.internal_format,
                width,
                height,
                0,
                texture.format,
                texture.type_,
                data,
            );

            if texture.level > 0 {
                gl::generate_mipmap(self.active_texture_unit);
            }
        }

        self
    }

    pub fn create_texture(&self, target: gl::GLenum, format: Option<TextureFormat>) -> GLTexture {
        let texture = gl::gen_textures(1);

        let format = format.unwrap_or_default();

        GLTexture {
            texture,
            target,
            level: format.level,
            internal_format: format.internal_format,
            format: format.format,
            type_: format.type_,
        }
    }

    pub fn bind_texture(&mut self, texture: GLTexture) -> &mut Self {
        unsafe {
            let unit = self
                .bound_texture_units
                .get_unchecked_mut(self.active_texture_unit as usize);

            unit.target = texture.target;
            unit.handle = texture.texture;

            self.bound_texture = Some(texture);
            gl::bind_texture(unit.target, unit.handle);
        };

        self
    }

    pub fn set_active_texture_unit(&mut self, unit: u32) -> &mut Self {
        self.active_texture_unit = unit;

        gl::active_texture(self.active_texture_unit);

        self
    }
}
