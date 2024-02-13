use crate::gl_shader::ShaderProgram;
use crate::gl_traits::Deletable;
use crate::gl_types::TextureTarget;
use crate::gl_utils::check_gl_error;
use anyhow::{Context, Result};
use gl::types::{GLenum, GLint};
use image::GenericImageView;
use std::os::raw::c_void;
use std::path::Path;

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
}

impl Texture {
    pub fn new<P: AsRef<Path>>(
        path: P,
        has_alpha: bool,
        flip_horizontal: bool,
        flip_vertical: bool,
        uniform_name: &str,
        texture_type: TextureTarget,
    ) -> Result<Self> {
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
                format!(
                    "Failed to bind to texture: {:?} (id: {})",
                    path.as_ref(),
                    texture_id
                )
            })?;

            // Set texture parameters here (e.g. GL_TEXTURE_WRAP_S, GL_TEXTURE_MIN_FILTER)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

            let format = if has_alpha { gl::RGBA } else { gl::RGB };

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as GLint,
                width as GLint,
                height as GLint,
                0,
                format,
                gl::UNSIGNED_BYTE,
                img_raw.as_ptr() as *const c_void,
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

        let uniform_name = if uniform_name.len() == 0 {
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
        })
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

//////////////////////////////////////////////////////////////////////////////
// - TextureBuilder -
//////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug)]
pub struct TextureBuilder {
    path: Option<String>,
    has_alpha: bool,
    flip_horizontal: bool,
    flip_vertical: bool,
    uniform_name: Option<String>,
    texture_target: Option<TextureTarget>,
}

impl TextureBuilder {
    pub fn path<P: Into<String>>(mut self, path: P) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn has_alpha(mut self, value: bool) -> Self {
        self.has_alpha = value;
        self
    }

    pub fn flip_horizontal(mut self, value: bool) -> Self {
        self.flip_horizontal = value;
        self
    }

    pub fn flip_vertical(mut self, value: bool) -> Self {
        self.flip_vertical = value;
        self
    }

    pub fn with_uniform_name(mut self, uniform_name: &str) -> Self {
        self.uniform_name = Some(uniform_name.to_string());
        self
    }

    pub fn with_texture_target(mut self, texture_target: TextureTarget) -> Self {
        self.texture_target = Some(texture_target);
        self
    }

    pub fn build(&self) -> Result<Texture> {
        let uniform_name = self.uniform_name.clone().unwrap_or(String::new());
        let texture_target = self
            .texture_target
            .clone()
            .unwrap_or(TextureTarget::Texture2D);
        Texture::new(
            self.path.clone().with_context(|| "No path specified")?,
            self.has_alpha,
            self.flip_horizontal,
            self.flip_vertical,
            &uniform_name,
            texture_target,
        )
    }
}

//////////////////////////////////////////////////////////////////////////////
// - MultiTexture -
//////////////////////////////////////////////////////////////////////////////

pub struct MultiTexture {
    textures: Vec<Texture>,
}

impl MultiTexture {
    pub fn new() -> Self {
        MultiTexture {
            textures: Vec::new(),
        }
    }

    pub fn add_texture(&mut self, texture: Texture) {
        self.textures.push(texture);
    }

    pub fn clear(&mut self) {
        self.textures.clear();
    }

    pub fn bind_textures(&self, shader_program: &mut ShaderProgram) -> Result<()> {
        for (index, texture) in self.textures.iter().enumerate() {
            if texture.uniform_name == None {
                continue;
            }
            texture.bind();
            let uniform_name = texture.uniform_name.as_ref().clone().unwrap();
            shader_program.set_uniform(&uniform_name, index as i32)?;
        }

        Ok(())
    }
}

impl Drop for MultiTexture {
    fn drop(&mut self) {
        self.textures.clear();
    }
}
