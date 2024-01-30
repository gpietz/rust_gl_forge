use crate::gl_types::ShaderType;
use crate::string_utils::*;
use anyhow::{anyhow, Context, Result};
use gl::types::{GLchar, GLenum, GLint, GLuint};
use std::collections::HashMap;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::ptr;

//////////////////////////////////////////////////////////////////////////////
// - Shader -
//////////////////////////////////////////////////////////////////////////////

pub struct Shader {
    id: u32,
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
            gl::CompileShader(shader);

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
    /// ```
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
        shader_file
            .read_to_string(&mut shader_content)
            .with_context(|| {
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
    /// ```
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

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            //println!("Drop shader");
            gl::DeleteShader(self.id);
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// - ShaderProgram -
//////////////////////////////////////////////////////////////////////////////

pub struct ShaderProgram {
    id: u32,
    uniform_ids: HashMap<String, i32>,
}

impl ShaderProgram {
    pub fn new(vertex_shader: Shader, fragment_shader: Shader) -> Result<ShaderProgram> {
        let program_id = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(program_id, vertex_shader.get_shader_id());
            gl::AttachShader(program_id, fragment_shader.get_shader_id());
            gl::LinkProgram(program_id);
            drop(vertex_shader);
            drop(fragment_shader);

            // Check for linking errors
            let mut success = gl::FALSE as GLint;
            gl::GetShaderiv(program_id, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                let mut len = 0;
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
                let error = create_whitespace_cstring_with_len(len as usize);
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    ptr::null_mut(),
                    error.as_ptr() as *mut GLchar,
                );
            }
        }

        Ok(ShaderProgram {
            id: program_id,
            uniform_ids: HashMap::new(),
        })
    }

    pub fn get_shader_program_id(&self) -> u32 {
        self.id
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    /// Retrieves the location of a uniform variable within the shader program.
    ///
    /// This method looks up the location of a uniform variable in the shader program.
    /// If the location is already cached in `self.uniform_ids`, it returns that value.
    /// Otherwise, it queries OpenGL to get the location, caches the result, and then returns it.
    ///
    /// # Arguments
    /// * `name` - A string slice representing the name of the uniform variable.
    ///
    /// # Returns
    /// * `Ok(GLint)` containing the location of the uniform variable if found.
    /// * `Err(anyhow::Error)` if the uniform variable is not found or if there's an error
    ///   during string conversion.
    ///
    /// # Errors
    /// This function returns an error if the uniform variable is not found in the shader program.
    /// It also returns an error if there's an issue with converting the provided name to a CString.
    ///
    /// # Examples
    /// ```
    /// let shader_program = ShaderProgram::new(vertex_shader, fragment_shader)?;
    /// let location = shader_program.get_uniform_location("myUniform")?;
    /// // Use the location for setting the uniform variable...
    /// ```
    pub fn get_uniform_location(&mut self, name: &str) -> Result<i32> {
        if let Some(&location) = self.uniform_ids.get(name) {
            return Ok(location);
        }

        let c_str = CString::new(name).unwrap();
        let location = unsafe { gl::GetUniformLocation(self.id, c_str.as_ptr()) };

        if location != -1 {
            self.uniform_ids.insert(name.to_string(), location);
            Ok(location)
        } else {
            Err(anyhow!("Uniform '{}' not found in shader program", name))
        }
    }

    /// Sets a uniform value in the shader program using a generic type.
    ///
    /// This method sets the value of a uniform variable in the shader program.
    /// It uses a generic type that must implement the `UniformValue` trait, allowing
    /// flexibility in the types of values that can be passed as uniforms (e.g., f32, vec3).
    ///
    /// # Type Parameters
    /// * `T`: The type of the uniform value. This type must implement the `UniformValue` trait.
    ///
    /// # Arguments
    /// * `name` - A string slice representing the name of the uniform variable.
    /// * `value` - The value of the uniform. The type of this value must implement the `UniformValue` trait.
    ///
    /// # Returns
    /// * `Ok(())` if the uniform was successfully set.
    /// * `Err(anyhow::Error)` if the uniform location is not found or if there's an error in the process.
    ///
    /// # Errors
    /// This function returns an error in the following cases:
    /// - If the uniform variable name is not found in the shader program, indicated by a `-1` location.
    /// - If there's an issue with the underlying `get_uniform_location` function, such as a CString conversion error.
    ///
    /// # Examples
    /// ```
    /// let shader_program = ShaderProgram::new(vertex_shader, fragment_shader)?;
    /// shader_program.set_uniform("myUniform", 0.5f32)?;
    /// shader_program.set_uniform("myVec3Uniform", (1.0, 0.0, 0.0))?;
    /// // Use shader_program with the updated uniforms...
    /// ```
    ///
    /// # Notes
    /// The actual setting of the uniform is delegated to the `set_uniform` method of the `UniformValue` trait,
    /// which must be implemented for each type that can be used as a uniform.
    pub fn set_uniform<T: UniformValue>(&mut self, name: &str, value: T) -> Result<()> {
        let location = self.get_uniform_location(name)?;
        if location == -1 {
            return Err(anyhow!("Uniform '{}' not found in shader", name));
        }

        value.set_uniform(location);
        Ok(())
    }

    /// Retrieves the names of all active uniform variables in the shader program.
    ///
    /// This method queries the shader program for all active uniform variables and returns
    /// their names. It can be particularly useful for debugging, shader inspection, or dynamic
    /// shader manipulation.
    ///
    /// # Returns
    /// * `Ok(Vec<String>)` containing a vector of strings with the names of all active uniforms.
    /// * `Err(anyhow::Error)` if there's an error during the uniform name retrieval process.
    ///
    /// # Errors
    /// This function returns an error if there's a failure in retrieving the name of any uniform
    /// variable. This could occur due to issues like exceeding buffer size or OpenGL state errors.
    ///
    /// # Examples
    /// ```
    /// let shader_program = ShaderProgram::new(vertex_shader, fragment_shader)?;
    /// let uniform_names = shader_program.get_all_uniform_names()?;
    /// for name in uniform_names {
    ///     println!("Uniform name: {}", name);
    /// }
    /// ```
    ///
    /// # Notes
    /// The buffer size for uniform names is set to 256 characters, which should be sufficient for
    /// most use cases. However, if you have uniforms with longer names, you may need to adjust
    /// this size. The method uses OpenGL's `gl::GetActiveUniform` function for querying uniform
    /// information.
    pub fn get_all_uniform_names(&self) -> Result<Vec<String>> {
        let mut num_uniforms = 0;
        unsafe {
            gl::GetProgramiv(self.id, gl::ACTIVE_UNIFORMS, &mut num_uniforms);
        }

        let mut names = Vec::new();
        for i in 0..num_uniforms {
            let mut len = 0;
            let mut size = 0;
            let mut utype = 0;
            let mut name_buf = vec![0; 256];

            unsafe {
                gl::GetActiveUniform(
                    self.id,
                    i as GLuint,
                    name_buf.len() as i32,
                    &mut len,
                    &mut size,
                    &mut utype,
                    name_buf.as_mut_ptr() as *mut GLchar,
                );
            }

            if len > 0 {
                let name = String::from_utf8_lossy(&name_buf[..len as usize]).to_string();
                names.push(name);
            } else {
                return Err(anyhow!(
                    "Failed to retrieve the name for uniform at index {}",
                    i
                ));
            }
        }

        Ok(names)
    }

    //=== Concepts  ===

    //Loading and Setting Textures:
    //Functions to bind textures to the shader program, useful for multi-texturing, texture animations, etc.
    //pub fn set_texture(&self, name: &str, texture: &Texture) -> Result<()>

    //Handling Transformation Matrices:
    //Functions to set transformation matrices like model, view, and projection matrices.
    //pub fn set_uniform_mat4(&mut self, name: &str, matrix: &Matrix4<f32>) -> Result<()>

    //Shader Reloading:
    //Ability to reload shaders on the fly, useful during development for hot-reloading shader code.
    //pub fn reload_shaders(&mut self) -> Result<()>

    //Uniform Block Binding: If using uniform blocks, functions to bind these blocks can be crucial.
    //pub fn bind_uniform_block(&self, block_name: &str, binding_point: u32) -> Result<()>

    //Handling Light Properties:
    //In 3D rendering, setting light properties (like position, color, intensity) can be important.
    //pub fn set_light_properties(&mut self, light: &Light) -> Result<()>

    //Cleanup and Resource Management: Proper cleanup functions to delete the shader program and free up resources.
    //pub fn cleanup(&mut self);

    //Querying Shader Info:
    //Methods to retrieve information about the shader, such as compile/link status, log messages, etc.
    //pub fn get_shader_info_log(&self) -> Result<String>;

    //Handling Custom Shader Attributes: Methods for enabling or disabling custom vertex attributes.
    //pub fn enable_vertex_attrib(&self, attrib_name: &str) -> Result<()>
    //pub fn disable_vertex_attrib(&self, attrib_name: &str) -> Result<()>

    //Setting Custom Shader Flags: For dynamic shaders, methods to set flags or toggle shader features can be useful.
    //pub fn set_shader_flag(&mut self, flag_name: &str, value: bool) -> Result<()>

    //Geometry Shader Support: If using geometry shaders, functions to handle them effectively.
    //pub fn set_geometry_shader(&mut self, shader: Shader) -> Result<()>
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// - ShaderFactory -
//////////////////////////////////////////////////////////////////////////////

// pub struct ShaderFactory;
//
// impl ShaderFactory {
//     pub fn create(shader_sources: Vec<ShaderSource>) -> Result<ShaderProgram> {
//     }
// }

//////////////////////////////////////////////////////////////////////////////
// - ShaderSource -
//////////////////////////////////////////////////////////////////////////////

// pub struct ShaderSource {}

//////////////////////////////////////////////////////////////////////////////
// - UniformValue -
//////////////////////////////////////////////////////////////////////////////

pub trait UniformValue {
    fn set_uniform(&self, location: i32);
}

impl UniformValue for f32 {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::Uniform1f(location, *self);
        }
    }
}

impl UniformValue for (f32, f32) {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::Uniform2f(location, self.0, self.1);
        }
    }
}

impl UniformValue for (f32, f32, f32) {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::Uniform3f(location, self.0, self.1, self.2);
        }
    }
}

impl UniformValue for cgmath::Vector2<f32> {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::Uniform2f(location, self.x, self.y);
        }
    }
}

impl UniformValue for cgmath::Vector3<f32> {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::Uniform3f(location, self.x, self.y, self.z);
        }
    }
}
