use crate::gl_types::ShaderType;
use crate::gl_utils::check_gl_error;
use crate::string_utils::create_whitespace_cstring_with_len;
use anyhow::{anyhow, Context, Result};
use gl::types::{GLchar, GLenum, GLint};
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::ptr;
use crate::gl_traits::Deletable;

//////////////////////////////////////////////////////////////////////////////
// - Shader -
//////////////////////////////////////////////////////////////////////////////

pub struct Shader {
    id: u32,
    shader_type: ShaderType,
    shader_file: Option<String>,
}

impl Shader {
    pub fn from_source(source: &str, shader_type: ShaderType) -> Result<Shader> {
        let id = unsafe {
            let shader_type = shader_type.to_gl_enum() as GLenum;
            let shader = gl::CreateShader(shader_type);
            let error = gl::GetError();
            if error != gl::NO_ERROR {
                println!("Error !!");
            }
            let c_str = CString::new(source.as_bytes())
                .context("Failed to create CString from shader source")?;
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            check_gl_error()?;

            gl::CompileShader(shader);
            check_gl_error()?;

            // Error checking
            let mut success = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let error = create_whitespace_cstring_with_len(len as usize);
                gl::GetShaderInfoLog(shader, len, ptr::null_mut(), error.as_ptr() as *mut GLchar);
                return Err(anyhow!(error.into_string().unwrap()));
            }

            shader
        };

        Ok(Shader {
            id,
            shader_type,
            shader_file: None,
        })
    }

    /// Creates a new `Shader` from a file.
    ///
    /// This method reads a shader's source code from a specified file and creates
    /// a new `Shader` object using that source. It handles the opening and reading
    /// of the file, error checking, and shader compilation.
    ///
    /// # Type Parameters
    /// * `P`: A generic parameter that can be converted to a `Path` reference, allowing
    ///   for flexible path handling. Accepts both `String` and `&str`.
    ///
    /// # Parameters
    /// * `shader_path`: The path to the shader file.
    /// * `shader_type`: The type of the shader (e.g., vertex, fragment).
    ///
    /// # Returns
    /// A `Result` which is `Ok` containing the new `Shader` object if the shader
    /// compilation is successful, or an `Err` with an error message in case of failure.
    ///
    /// # Errors
    /// This method can return an error in several cases:
    /// - If the shader file cannot be opened.
    /// - If reading from the shader file fails.
    /// - If shader compilation fails.
    ///
    /// # Examples
    /// ```no-run
    /// let shader_type = ShaderType::Vertex; // Assuming ShaderType is defined
    /// let shader = Shader::from_file("path/to/shader.glsl", shader_type);
    ///
    /// match shader {
    ///     Ok(shader) => { /* Use the shader */ }
    ///     Err(e) => eprintln!("Error creating shader: {}", e),
    /// }
    /// ```
    pub fn from_file<P: AsRef<Path>>(shader_path: P, shader_type: ShaderType) -> Result<Shader> {
        // Open shader file
        let mut shader_file = File::open(shader_path.as_ref()).with_context(|| {
            format!("Failed top open shader: {}", shader_path.as_ref().display())
        })?;

        // Load content from file
        let mut shader_content = String::new();
        shader_file.read_to_string(&mut shader_content).with_context(|| {
            format!("Failed to read shader: {}", shader_path.as_ref().display())
        })?;

        // Assuming `from_source` creates the shader and returns its id
        let shader = Self::from_source(&shader_content, shader_type)
            .map_err(|e| anyhow!("Failed to create shader: {}", e))?;

        // Convert the shader path to a String
        let shader_file_path = shader_path.as_ref().to_string_lossy().into_owned();
        println!("Shader loaded: {} (id: {})", shader_file_path, shader.id);

        Ok(shader)
    }

    pub fn get_shader_id(&self) -> u32 {
        self.id
    }

    /// Retrieves a reference to the shader file path.
    ///
    /// This method returns an `Option` containing a reference to the `String` that
    /// holds the path of the shader file, if it exists. If the shader was not
    /// created from a file or if the file path is not available, it returns `None`.
    ///
    /// # Returns
    /// * `Some(&String)` - A reference to the `String` containing the shader file path.
    /// * `None` - If the shader file path is not available or not applicable.
    ///
    /// # Examples
    /// ```no-run
    /// let shader = Shader::from_file("path/to/shader.glsl", shader_type)?;
    /// let shader_file_path = shader.get_shader_file();
    ///
    /// match shader_file_path {
    ///     Some(path) => println!("Shader file path: {}", path),
    ///     None => println!("No shader file path available"),
    /// }
    /// ```
    pub fn get_shader_file(&self) -> Option<&String> {
        self.shader_file.as_ref()
    }

    pub fn load_vertex_shader(source: &str) -> Result<Shader> {
        Shader::from_source(source, ShaderType::Vertex).context("Failed to load vertex shader")
    }

    pub fn load_fragment_shader(source: &str) -> Result<Shader> {
        Shader::from_source(source, ShaderType::Fragment).context("Failed to load fragment shader")
    }
}

impl Deletable for Shader {
    /// Deletes the shader from the OpenGL context.
    ///
    /// This method safely deletes the shader associated with this instance from the GPU,
    /// provided it has been created and not already deleted. It ensures that the shader
    /// is only deleted if it exists (i.e., `self.id` is non-zero), preventing redundant
    /// deletion calls. After deletion, the shader ID is reset to zero to indicate that
    /// the shader has been deleted and to prevent potential misuse of an invalid ID.
    ///
    /// This method is called automatically when a `Shader` instance is dropped, but it can
    /// also be called explicitly to manage the shader's lifecycle manually.
    ///
    /// # Safety
    ///
    /// This method contains `unsafe` code to interact with the underlying OpenGL API.
    /// It is considered safe under the assumption that it is called with a valid shader
    /// ID and that no other OpenGL errors occur outside of this function. However, as with
    /// all `unsafe` code, caution should be exercised to ensure that the preconditions
    /// for safe use are met.
    ///
    /// # Examples
    ///
    /// ```no-run
    /// let mut shader = Shader::new(vertex_source, ShaderType::Vertex)?;
    /// // Use the shader...
    /// shader.delete(); // Explicitly delete the shader when done
    /// ```
    fn delete(&mut self) -> Result<()> {
        unsafe {
            if self.id != 0 {
                gl::DeleteShader(self.id);
                self.id = 0;
            }
        }
        Ok(())
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        if let Err(err) = self.delete() {
            eprintln!("Error while dropping shader: {}", err);
            // You might choose to log the error or take other appropriate actions here.
        }
    }
}
