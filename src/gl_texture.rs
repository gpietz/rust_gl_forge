use crate::gl_traits::Deletable;
use crate::gl_utils::check_gl_error;
use anyhow::{Context, Result};
use gl::types::GLint;
use std::os::raw::c_void;
use std::path::Path;

//////////////////////////////////////////////////////////////////////////////
// - Texture -
//////////////////////////////////////////////////////////////////////////////

pub struct Texture {
    id: u32,
}

impl Texture {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let img = image::open(path.as_ref())
            .with_context(|| format!("Failed to load texture from {:?}", path.as_ref()))?
            .into_rgba8();
        let (width, height) = img.dimensions();

        let mut texture_id = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            check_gl_error()
                .with_context(|| format!("Failed to create texture object: {:?}", path.as_ref()))?;
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            check_gl_error().with_context(|| {
                format!(
                    "Failed to bind to texture: {:?} (id: {})",
                    path.as_ref(),
                    texture_id
                )
            })?;

            // Set texture parameters here (e.g. GL_TEXTURE_WRAP_S, GL_TEXTURE_MIN_FILTER)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as GLint,
                width as GLint,
                height as GLint,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.into_raw().as_ptr() as *const c_void,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
            check_gl_error().with_context(|| {
                format!(
                    "Failed to generate mipmap: {:?} (id: {})",
                    path.as_ref(),
                    texture_id
                )
            })?;

            // Unbind the texture
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        #[rustfmt::skip]
        println!("Loaded texture: {} (id: {}, {}x{})", path.as_ref().to_string_lossy(), texture_id, width, height);

        Ok(Texture { id: texture_id })
    }

    /// Returns id associated with this texture.
    pub fn get_texture_id(&self) -> u32 {
        self.id
    }

    /// Binds the texture for use in rendering.
    pub fn bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    /// Unbinds the texture.
    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

impl Deletable for Texture {
    fn delete(&mut self) -> Result<()> {
        if self.id != 0 {
            unsafe {
                gl::DeleteTextures(1, &self.id);
            }
            self.id = 0;
        }
        Ok(())
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        if let Err(err) = self.delete() {
            eprintln!("Error while dropping texture: {}", err);
            // You might choose to log the error or take other appropriate actions here.
        }
    }
}
