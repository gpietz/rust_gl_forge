use std::os::raw::c_void;
use std::path::Path;
use anyhow::{anyhow, Context};
use gl::types::{GLenum, GLint};
use image::GenericImageView;
use crate::gl_prelude::{check_gl_error, Deletable, TextureTarget};

const ERR_DELETE_NON_OWNER: &str = r#"Attempted to delete a Texture that is not owned.
Only the owner should attempt to delete the texture to avoid
multiple deletion attempts of the same GPU resource."#;
const ERR_CLONE_NON_CLONEABLE: &str = r#"Attempted to clone a Texture instance that is not clonable.
Only the original owner-created instances should be cloned to prevent
multiple instances attempting to manage the same GPU resource lifecycle."#;


//////////////////////////////////////////////////////////////////////////////
// - Texture -
//////////////////////////////////////////////////////////////////////////////

pub struct Texture {
    id: u32,
    path: String,
    alpha: bool,
    flip: [bool; 2],
    dimension: [u32; 2],
    pub uniform_name: Option<String>,
    texture_type: TextureTarget,
    cloneable: bool,
}

impl Texture {
    pub fn new<P: AsRef<Path>>(
        path: P,
        has_alpha: bool,
        flip_horizontal: bool,
        flip_vertical: bool,
        uniform_name: &str,
        texture_type: TextureTarget,
    ) -> anyhow::Result<Self> {
        let mut img = image::open(path.as_ref())
            .with_context(|| format!("Failed to load texture from {:?}", path.as_ref()))?;

        // Flipping
        if flip_horizontal {
            img = img.fliph();
        }
        if flip_vertical {
            img = img.flipv();
        }

        let (width, height) = img.dimensions();
        let img_raw = if has_alpha {
            img.into_rgba8().into_raw()
        } else {
            img.into_rgb8().into_raw()
        };

        let mut texture_id = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            check_gl_error()
                .with_context(|| format!("Failed to create texture object: {:?}", path.as_ref()))?;
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            check_gl_error().with_context(|| {
                format!("Failed to bind to texture: {:?} (id: {})", path.as_ref(), texture_id)
            })?;

            // Set texture parameters here (e.g. GL_TEXTURE_WRAP_S, GL_TEXTURE_MIN_FILTER)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

            let format = if has_alpha {
                gl::RGBA
            } else {
                gl::RGB
            };

            let gl_texture_type = texture_type.to_gl_enum();
            gl::TexImage2D(
                gl_texture_type,
                0,
                format as GLint,
                width as GLint,
                height as GLint,
                0,
                format,
                gl::UNSIGNED_BYTE,
                img_raw.as_ptr() as *const c_void,
            );

            gl::GenerateMipmap(gl_texture_type);
            check_gl_error().with_context(|| {
                format!("Failed to generate mipmap: {:?} (id: {})", path.as_ref(), texture_id)
            })?;

            // Unbind the texture
            gl::BindTexture(gl_texture_type, 0);
        }

        #[rustfmt::skip]
        println!("Loaded texture: {} (id: {}, {}x{})", path.as_ref().to_string_lossy(), texture_id, width, height);

        let uniform_name = if uniform_name.is_empty() {
            None
        } else {
            Some(uniform_name.to_string())
        };

        Ok(Texture {
            id: texture_id,
            path: path.as_ref().to_string_lossy().to_string(),
            alpha: has_alpha,
            flip: [flip_horizontal, flip_vertical],
            dimension: [width, height],
            uniform_name,
            texture_type,
            cloneable: true,
        })
    }

    pub(crate) fn clone_as_non_owner(&self) -> anyhow::Result<Self> {
        if !self.cloneable {
            Err(anyhow!(ERR_CLONE_NON_CLONEABLE))
        } else {
            Ok(Texture {
                id: self.id,
                path: self.path.clone(),
                alpha: self.alpha,
                flip: self.flip,
                dimension: self.dimension,
                uniform_name: self.uniform_name.clone(),
                texture_type: self.texture_type,
                cloneable: false,
            })
        }
    }

    /// Returns id associated with this texture.
    pub fn get_texture_id(&self) -> u32 {
        self.id
    }

    pub fn texture_type(&self) -> TextureTarget {
        self.texture_type
    }

    /// Binds the texture for use in rendering.
    pub fn bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    /// Binds the texture to a specified texture unit.
    ///
    /// This function activates a given texture unit and binds the current texture object
    /// to the GL_TEXTURE_2D target within that unit. This is necessary for multitexturing
    /// and when you need to assign multiple textures to different texture units for use
    /// in a shader.
    ///
    /// # Parameters
    ///
    /// - `texture_unit`: The index of the texture unit to which this texture will be bound.
    ///   This index is zero-based, with `0` corresponding to `GL_TEXTURE0`, `1` to `GL_TEXTURE1`, and so on.
    ///   The actual OpenGL texture unit used is `GL_TEXTURE0 + texture_unit`.
    pub fn bind_as_unit(&self, texture_unit: u32) {
        if texture_unit > 31 {
            panic!("Texture unit must be between 0 and 31.");
        }
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + texture_unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    /// Binds the texture represented by `self` to a specific texture unit.
    ///
    /// Activates the specified texture unit and binds this texture as the current `TEXTURE_2D` target
    /// to that unit. This is essential for multi-texture rendering where each texture is assigned to
    /// a different texture unit.
    /// /// # Arguments
    ///
    /// * `texture_unit` - The texture unit to activate before binding this texture. This should be
    ///   `gl::TEXTURE0` + n, where n is the index of the desired texture unit (starting from 0).
    pub fn bind_as_gl_enum(&self, texture_unit: GLenum) {
        if !(gl::TEXTURE0..=gl::TEXTURE31).contains(&texture_unit) {
            panic!("Texture unit must be in range from GL_TEXTURE0 to GL_TEXTURE31.");
        }
        unsafe {
            gl::ActiveTexture(texture_unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    /// Unbinds the texture.
    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn has_alpha(&self) -> bool {
        self.alpha
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn flipped_horizontal(&self) -> bool {
        self.flip[0]
    }

    pub fn flipped_vertical(&self) -> bool {
        self.flip[1]
    }

    pub fn width(&self) -> u32 {
        self.dimension[0]
    }

    pub fn height(&self) -> u32 {
        self.dimension[1]
    }
}

impl Deletable for Texture {
    fn delete(&mut self) -> anyhow::Result<()> {
        if !self.cloneable {
            return Err(anyhow!(ERR_DELETE_NON_OWNER));
        }
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
        if self.cloneable {
            if let Err(err) = self.delete() {
                eprintln!("Error while dropping texture: {}", err);
                // You might choose to log the error or take other appropriate actions here.
            }
        }
    }
}
