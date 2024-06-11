use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::panic::panic_any;
use std::path::Path;
use std::str::from_utf8;
use std::{fs, ptr};

use crate::core::file_utils;
use anyhow::{anyhow, Context, Result};
use cgmath::Matrix;
use gl::types::{GLboolean, GLchar, GLenum, GLint, GLuint};

use crate::gl_traits::Deletable;
use crate::gl_types::ShaderType;
use crate::gl_utils::check_gl_error;
use crate::string_utils::*;

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
    /// ```
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

//////////////////////////////////////////////////////////////////////////////
// - ShaderProgram -
//////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct ShaderProgram {
    id: u32,
    uniform_ids: RefCell<HashMap<String, i32>>,
    shader_sources: HashMap<ShaderType, String>,
    shader_files: HashMap<ShaderType, String>,
}

impl ShaderProgram {
    pub fn new() -> Self {
        ShaderProgram {
            id: 0,
            uniform_ids: RefCell::new(HashMap::new()),
            shader_sources: HashMap::new(),
            shader_files: HashMap::new(),
        }
    }

    pub fn from_files(shader_files: &[&str]) -> Result<ShaderProgram> {
        let program_id = unsafe { gl::CreateProgram() };

        // Attach shaders
        let mut shaders = Vec::new();
        for filename in shader_files {
            let extension = filename.rsplit_once('.').map(|(_, ext)| ext);
            let shadertype = match extension {
                Some("vert") => ShaderType::Vertex,
                Some("frag") => ShaderType::Fragment,
                Some("geom") => ShaderType::Geometry,
                Some("comp") => ShaderType::Compute,
                _ => return Err(anyhow::anyhow!(format!("Unknown shader type: {}", filename))),
            };

            let shader = Shader::from_file(filename, shadertype)
                .with_context(|| format!("Failed loading shader: {}", filename))?;

            unsafe {
                gl::AttachShader(program_id, shader.get_shader_id());
                check_gl_error()?;
            }

            shaders.push(shader);
        }

        // Link program
        unsafe {
            gl::LinkProgram(program_id);
            check_gl_error()?;

            // Check for linking errors
            let mut success = gl::FALSE as GLint;
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
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
                return Err(anyhow::anyhow!(error.to_string_lossy().into_owned()));
            }
        }

        // Detach shaders after successful linking
        unsafe {
            for shader in shaders.iter_mut() {
                gl::DetachShader(program_id, shader.get_shader_id());
                shader.delete()?;
            }
        }

        println!("Shader program created successfully (id: {})", program_id);

        let mut shader_program = Self::new();
        Ok(shader_program)
    }

    #[deprecated]
    pub fn new_dumb(
        vertex_shader: &mut Shader,
        fragment_shader: &mut Shader,
    ) -> Result<ShaderProgram> {
        let program_id = unsafe { gl::CreateProgram() };
        unsafe {
            // Attach vertex shader
            gl::AttachShader(program_id, vertex_shader.get_shader_id());
            check_gl_error()?;

            // Attach fragment shader
            gl::AttachShader(program_id, fragment_shader.get_shader_id());
            check_gl_error()?;

            // Link the program
            gl::LinkProgram(program_id);
            check_gl_error()?;

            // Check for linking errors
            let mut success = gl::FALSE as GLint;
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
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
                return Err(anyhow::anyhow!(error.to_string_lossy().into_owned()));
            }
        }

        // Detach shaders after successful linking
        unsafe {
            gl::DetachShader(program_id, vertex_shader.get_shader_id());
            gl::DetachShader(program_id, fragment_shader.get_shader_id());
        }

        // Delete shaders cause they are no longer required
        vertex_shader.delete()?;
        fragment_shader.delete()?;

        println!("Shader program created successfully (id: {})", program_id);

        let mut shader_program = ShaderProgram::new();
        Ok(shader_program)
    }

    pub fn program_id(&self) -> u32 {
        self.id
    }

    pub fn activate(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn deactivate(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn is_active(&self) -> bool {
        unsafe {
            let mut program_id: GLint = 0;
            gl::GetIntegerv(gl::CURRENT_PROGRAM, &mut program_id);
            program_id == self.id as i32
        }
    }

    pub fn clear_uniform_locations(&self) {
        let mut uniforms = self.uniform_ids.borrow_mut();
        uniforms.clear();
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
    pub fn get_uniform_location(&self, name: &str) -> Result<i32> {
        if let Some(&location) = self.uniform_ids.borrow().get(name) {
            return Ok(location);
        }

        let c_str = CString::new(name).unwrap();
        let location = unsafe { gl::GetUniformLocation(self.id, c_str.as_ptr()) };

        if location != -1 {
            let mut uniforms = self.uniform_ids.borrow_mut();
            uniforms.insert(name.to_string(), location);
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
    pub fn set_uniform<T: UniformValue>(&self, name: &str, value: T) -> Result<()> {
        let location = self.get_uniform_location(name)?;
        if location == -1 {
            return Err(anyhow!("Uniform '{}' not found in shader", name));
        }

        value.set_uniform(location);
        Ok(())
    }

    /// Sets the value of a shader uniform variable.
    ///
    /// This function applies a value to a specified uniform location within a shader program.
    /// It is generic over types that implement the `UniformValue` trait, allowing for flexibility
    /// in the types of values that can be passed as uniform variables (e.g., integers, floats, vectors, matrices).
    ///
    /// # Arguments
    /// * `location` - The location identifier of the uniform variable within the shader program.
    ///                This should be obtained via `glGetUniformLocation`.
    /// * `value` - The value to be set for the uniform variable. The type `T` must implement the `UniformValue` trait.
    ///
    /// # Returns
    /// * `Ok(())` if the uniform value was successfully set.
    /// * `Err(anyhow::Error)` if the provided `location` is invalid (`-1`), indicating the uniform location was not found.
    ///
    /// # Examples
    /// ```no-run
    /// // Assuming `shader_program` is a compiled and linked shader program and `UniformValue` is implemented for `f32`.
    /// let location = gl::GetUniformLocation(shader_program, c_str!("someUniform"));
    /// set_uniform_value(location, 0.5f32).expect("Failed to set uniform value");
    /// ```
    ///
    /// # Errors
    /// This function returns an error if the uniform location is invalid (i.e., `location == -1`), which typically
    /// indicates that the uniform name does not exist or was not active in the shader program.
    pub fn set_uniform_value<T: UniformValue>(&self, location: i32, value: T) -> Result<()> {
        if location == -1 {
            return Err(anyhow!("Uniform location is invalid: -1"));
        }

        value.set_uniform(location);
        Ok(())
    }

    pub fn set_uniform_matrix<T: UniformMatrix>(
        &self,
        name: &str,
        transpose: bool,
        matrix: &T,
    ) -> Result<()> {
        let location = self.get_uniform_location(name)?;
        if location == -1 {
            return Err(anyhow!("Uniform '{}' not found in shader", name));
        }
        matrix.set_uniform_matrix(location, transpose);
        Ok(())
    }

    /// Sets a uniform variable with a three-component floating-point vector value in the shader program.
    ///
    /// This method allows you to set the value of a uniform variable in the shader program
    /// to a three-component vector of floating-point values. It is typically used to pass
    /// data such as positions, colors, or other vector-based information to the shader.
    ///
    /// # Parameters
    ///
    /// - `name`: A string slice that holds the name of the uniform variable in the shader.
    /// - `value0`: The first component of the three-component vector.
    /// - `value1`: The second component of the three-component vector.
    /// - `value2`: The third component of the three-component vector.
    ///
    /// # Returns
    ///
    /// - `Result<()>`: Returns `Ok(())` if the uniform variable was successfully set,
    ///   or an error if there was a problem setting the uniform variable.
    ///
    /// # Errors
    ///
    /// This method will return an error if the uniform variable could not be found or set
    /// in the shader program. Common errors include providing an invalid uniform name or
    /// having a mismatch between the expected and provided data types.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut shader_program = ShaderProgram::new();
    /// shader_program.set_uniform_3f("u_Color", 1.0, 0.0, 0.0)?;
    /// ```
    ///
    /// In this example, the uniform variable `u_Color` in the shader program is set to
    /// the color red with RGB components (1.0, 0.0, 0.0).
    ///
    /// # Note
    ///
    /// Ensure that the shader program is currently in use before setting uniform variables.
    /// The shader program should be bound using appropriate methods before calling this function.
    pub fn set_uniform_3f(&self, name: &str, value0: f32, value1: f32, value2: f32) -> Result<()> {
        self.set_uniform(name, (value0, value1, value2))
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
                return Err(anyhow!("Failed to retrieve the name for uniform at index {}", i));
            }
        }

        Ok(names)
    }

    pub fn add_file(&mut self, r#type: ShaderType, file: &str) -> Result<()> {
        if self.is_type_defined(&r#type) {
            return Err(anyhow!("ShaderType already defined: {}", r#type));
        }
        self.shader_files.insert(r#type, file.to_string());
        Ok(())
    }

    pub fn add_source(&mut self, r#type: ShaderType, source: &[u8]) -> Result<()> {
        if self.is_type_defined(&r#type) {
            return Err(anyhow!("ShaderType already defined: {}", r#type));
        }
        let source_str =
            std::str::from_utf8(source).map_err(|e| anyhow!("Invalid UTF-8 sequence: {}", e))?;
        self.shader_sources.insert(r#type, source_str.to_string());
        Ok(())
    }

    pub fn is_type_defined(&self, r#type: &ShaderType) -> bool {
        self.shader_sources.contains_key(r#type) || self.shader_files.contains_key(r#type)
    }

    pub fn compile(&mut self) -> Result<()> {
        let mut shader_sources: HashMap<ShaderType, CString> = HashMap::new();
        for shader_file in &self.shader_files {
            // Get and display size of the shader file
            let file_size = match file_utils::file_size(shader_file.1) {
                Ok(size) => size,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    0
                }
            };

            // Load shader source from file
            let source = fs::read_to_string(shader_file.1)?;
            let source = CString::new(source.as_bytes())?;
            shader_sources.insert(*shader_file.0, source);
            println!("Shader file loaded: {} ({})", shader_file.1, readable_bytes(file_size));
        }
        for shader_source in &self.shader_sources {
            let source_bytes = shader_source.1.as_bytes();
            let source = CString::new(source_bytes)?;
            shader_sources.insert(*shader_source.0, source);
            println!("Shader source added: {}", readable_bytes(source_bytes.len() as u64));
        }

        unsafe {
            let shader_program = gl::CreateProgram();
            let mut shader_ids = Vec::new();

            // Compile shaders
            for shader_source in shader_sources {
                let shader_type_name = shader_source.0.to_string();
                let shader = gl::CreateShader(shader_source.0.into());
                gl::ShaderSource(shader, 1, &shader_source.1.as_ptr(), ptr::null());
                gl::CompileShader(shader);
                check_compile_errors(shader, &shader_type_name)?;
                gl::AttachShader(shader_program, shader);
                shader_ids.push(shader);
            }

            // Link program
            gl::LinkProgram(shader_program);
            check_compile_errors(shader_program, "PROGRAM")?;

            // Delete shaders
            for shader_id in shader_ids {
                gl::DeleteShader(shader_id);
            }

            self.id = shader_program as u32;
        }

        Ok(())
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

    // let mut vertex_shader = Shader::from_file(vertex_shader, ShaderType::Vertex)?;
    // let mut fragment_shader = Shader::from_file(fragment_shader, ShaderType::Fragment)?;
    // let shader_program = ShaderProgram::new(&mut vertex_shader, &mut fragment_shader)?;
    // vertex_shader.delete()?;
    // fragment_shader.delete()?;
}

impl Deletable for ShaderProgram {
    fn delete(&mut self) -> Result<()> {
        if self.id != 0 {
            unsafe {
                gl::DeleteProgram(self.id);
            }
            self.id = 0;
        }
        Ok(())
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        if let Err(err) = self.delete() {
            eprintln!("Error while dropping shader program: {}", err);
            // You might choose to log the error or take other appropriate actions here.
        }
    }
}

fn check_compile_errors(shader: GLuint, shader_type: &str) -> Result<()> {
    let mut success: GLint = 1;
    let mut info_log = vec![0; 1024];

    unsafe {
        match shader_type {
            "PROGRAM" => {
                gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
                if success == 0 {
                    gl::GetShaderInfoLog(
                        shader,
                        1024,
                        ptr::null_mut(),
                        info_log.as_mut_ptr() as *mut GLchar,
                    );
                    let error_message = from_utf8(&info_log).unwrap_or("Failed to read log");
                    return Err(anyhow!(
                        "ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n",
                        shader_type,
                        error_message
                    ));
                }
            }
            _ => {
                gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
                if success == 0 {
                    gl::GetProgramInfoLog(
                        shader,
                        1024,
                        ptr::null_mut(),
                        info_log.as_mut_ptr() as *mut GLchar,
                    );
                    let error_message = from_utf8(&info_log).unwrap_or("Failed to read log");
                    return Err(anyhow!(
                        "ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n",
                        shader_type,
                        error_message
                    ));
                }
            }
        }
    }

    Ok(())
}

//////////////////////////////////////////////////////////////////////////////
// - ShaderFactory -
//////////////////////////////////////////////////////////////////////////////

/// A factory for creating shader programs.
///
/// The `ShaderFactory` struct provides a simple interface for creating shader programs
/// by loading and compiling vertex and fragment shaders from different sources. It abstracts
/// the process of shader creation, allowing you to quickly generate shader programs for
/// use in your graphics applications.
///
/// # Example
///
/// ```rust
/// use gl_shader::ShaderFactory;
///
/// // Create a shader program using the ShaderFactory
/// match ShaderFactory::from_files("vertex_shader.glsl", "fragment_shader.glsl") {
///     Ok(shader_program) => {
///         // Successfully created a shader program
///     },
///     Err(error) => {
///         eprintln!("Error: {}", error);
///     },
/// }
/// ```
pub struct ShaderFactory;

impl ShaderFactory {
    /// Creates a `ShaderProgram` from given vertex and fragment shader sources.
    ///
    /// This function compiles the provided vertex and fragment shader source code
    /// and links them into a `ShaderProgram`. After linking, the individual shader
    /// objects are deleted as they are no longer needed.
    ///
    /// # Parameters
    /// - `vertex_shader`: A string slice containing the vertex shader source code.
    /// - `fragment_shader`: A string slice containing the fragment shader source code.
    ///
    /// # Returns
    /// - `Result<ShaderProgram>`: Returns a `ShaderProgram` if the shaders compile
    ///   and link successfully, otherwise returns an error.
    ///
    /// # Errors
    /// Returns an error if the shader compilation or program linking fails.
    ///
    /// # Example
    /// ```rust
    /// let shader_program = ShaderProgram::from_source(vertex_src, fragment_src)?;
    /// ```
    pub fn from_source(vertex_shader: &str, fragment_shader: &str) -> Result<ShaderProgram> {
        let mut vertex_shader = Shader::from_source(vertex_shader, ShaderType::Vertex)?;
        let mut fragment_shader = Shader::from_source(fragment_shader, ShaderType::Fragment)?;
        let shader_program = ShaderProgram::new_dumb(&mut vertex_shader, &mut fragment_shader)?;
        vertex_shader.delete()?;
        fragment_shader.delete()?;
        Ok(shader_program)
    }

    /// Creates a new shader program from vertex and fragment shader source files.
    ///
    /// This function takes the file paths to the vertex and fragment shaders, compiles
    /// them, links them into a shader program, and returns a `Result` containing the
    /// resulting `ShaderProgram` if successful.
    ///
    /// # Arguments
    ///
    /// * `vertex_shader` - A string representing the file path to the vertex shader source file.
    /// * `fragment_shader` - A string representing the file path to the fragment shader source file.
    ///
    /// # Returns
    ///
    /// * `Result<ShaderProgram>` - If the shader program creation is successful, it returns
    ///   a `ShaderProgram`. If there are any errors during shader compilation or program linking,
    ///   it returns an `Err` with an error message.
    ///
    /// # Errors
    ///
    /// This function may return an `Err` if:
    ///
    /// * The vertex shader file cannot be loaded or compiled.
    /// * The fragment shader file cannot be loaded or compiled.
    /// * The shader program cannot be linked successfully.
    ///
    /// # Example
    ///
    /// ```
    /// use gl_shader::{ShaderFactory, ShaderProgram};
    ///
    /// match ShaderFactory::from_files("vertex_shader.glsl", "fragment_shader.glsl") {
    ///     Ok(shader_program) => {
    ///         // Successfully created a shader program
    ///     },
    ///     Err(error) => {
    ///         eprintln!("Error: {}", error);
    ///     },
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This function assumes the existence of a custom shader library (your_shader_library)
    /// with appropriate types and functions for shader handling (e.g., `Shader`, `ShaderType`, etc.).
    pub fn from_files(vertex_shader: &str, fragment_shader: &str) -> Result<ShaderProgram> {
        let mut vertex_shader = Shader::from_file(vertex_shader, ShaderType::Vertex)?;
        let mut fragment_shader = Shader::from_file(fragment_shader, ShaderType::Fragment)?;
        let shader_program = ShaderProgram::new_dumb(&mut vertex_shader, &mut fragment_shader)?;
        vertex_shader.delete()?;
        fragment_shader.delete()?;
        Ok(shader_program)
    }
}

//////////////////////////////////////////////////////////////////////////////
// - ShaderSource -
//////////////////////////////////////////////////////////////////////////////

/// Represents a shader source, which can be loaded either from a file or provided as a String.
pub struct ShaderSource {
    /// The type of shader (e.g., vertex, fragment, etc.).
    pub r#type: ShaderType,
    /// The source code of the shader.
    pub source: String,
    /// Indicates whether the source should be loaded from a file (`true`) or provided directly (`false`).
    pub is_file: bool,
}

impl ShaderSource {
    /// Checks if the shader source is valid, ensuring it is not empty or consists only of whitespace.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the shader source is not empty or contains non-whitespace characters.
    /// - `Err` with an error message if the shader source is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use my_shader_library::Shader;
    ///
    /// let shader = Shader::new("vertex_shader.glsl", "   ").unwrap();
    /// let result = shader.is_valid();
    ///
    /// assert!(result.is_err());
    /// ```
    pub fn is_valid(&self) -> Result<()> {
        if self.source.trim().is_empty() {
            Err(anyhow!("Empty shader source is invalid"))
        } else {
            Ok(())
        }
    }

    pub fn from_file(r#type: ShaderType, source: &str) -> Self {
        Self {
            r#type,
            source: source.to_owned(),
            is_file: true,
        }
    }

    pub fn from_source(r#type: ShaderType, source: &str) -> Self {
        Self {
            r#type,
            source: source.to_owned(),
            is_file: false,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// - UniformValue -
//////////////////////////////////////////////////////////////////////////////

pub trait UniformValue {
    fn set_uniform(&self, location: i32);
}

impl UniformValue for bool {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::Uniform1i(location, *self as GLint);
        }
    }
}

impl UniformValue for i32 {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::Uniform1i(location, *self as GLint);
        }
    }
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

impl UniformValue for [f32; 3] {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::Uniform3f(location, self[0], self[1], self[2]);
        }
    }
}

impl UniformValue for [f32; 4] {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::Uniform4f(location, self[0], self[1], self[2], self[3]);
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

//////////////////////////////////////////////////////////////////////////////
// - UniformMatrix -
//////////////////////////////////////////////////////////////////////////////

/// `UniformMatrix` is a trait designed for setting uniform matrix values in a shader program.
///
/// This trait abstracts the operation of passing matrix data from your Rust code to shaders
/// within your graphics API (e.g., OpenGL, Vulkan, etc.). Implementing this trait allows for
/// a uniform way to set matrix uniforms across different shader programs and matrix types.
///
/// # Parameters
/// - `location`: The location identifier for the uniform variable in the shader program. This
///   is typically obtained by querying the shader program with the name of the uniform variable.
///
/// - `transpose`: Specifies whether the supplied matrix should be transposed before being
///   sent to the shader. If `true`, the matrix's transpose (i.e., its rows and columns are
///   swapped) is used. This is particularly useful because Rust and some graphics APIs like
///   OpenGL expect matrices in different formats (row-major vs column-major).
pub trait UniformMatrix {
    fn set_uniform_matrix(&self, location: i32, transpose: bool);
}

impl UniformMatrix for cgmath::Matrix4<f32> {
    fn set_uniform_matrix(&self, location: i32, transpose: bool) {
        unsafe {
            let matrix_ptr = self.as_ptr();
            gl::UniformMatrix4fv(location, 1, transpose as GLboolean, matrix_ptr);
        }
    }
}