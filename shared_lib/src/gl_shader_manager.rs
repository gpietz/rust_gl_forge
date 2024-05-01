use crate::gl_shader::ShaderProgram;
use anyhow::{anyhow, Result};
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct ShaderManager {
    shaders: HashMap<String, Vec<String>>,
    shader_programs: HashMap<String, ShaderProgram>,
}

impl ShaderManager {
    pub fn add_shader(&mut self, key: String, file_path: String) {
        let entry = self.shaders.entry(key).or_default();
        entry.push(file_path);
    }

    pub fn get_shader(&mut self, key: &str) -> Result<&ShaderProgram> {
        // Check if the shader already exists.
        if self.shader_programs.contains_key(key) {
            // Safe to unwrap because we just checked it exists.
            return Ok(self.shader_programs.get(key).unwrap());
        }

        // If the shader program does not exist, check if source is available to compile
        if !self.shaders.contains_key(key) {
            return Err(anyhow!("No shader found for key: {}", key));
        }

        // Compile the shader, handling any error from compilation
        self.compile_shader(key)
    }

    pub fn get_shader_mut(&mut self, key: &str) -> Result<&mut ShaderProgram> {
        // Check if the shader already exists.
        if self.shader_programs.contains_key(key) {
            // Safe to unwrap because we just checked it exists.
            return Ok(self.shader_programs.get_mut(key).unwrap());
        }

        // If the shader program does not exist, check if source is available to compile
        if !self.shaders.contains_key(key) {
            return Err(anyhow!("No shader found for key: {}", key));
        }

        // Compile the shader, handling any error from compilation
        self.compile_shader(key)?;

        // Now retrieve and return a mutable reference to the newly compiled shader
        Ok(self.shader_programs.get_mut(key).unwrap())
    }

    pub fn compile_shader(&mut self, key: &str) -> Result<&ShaderProgram> {
        if let Some(paths) = self.shaders.get(key) {
            println!("Compiling shader: {}", paths.join(", "));
            let path_slices: Vec<&str> = paths.iter().map(|s| s.as_str()).collect();

            // Compile the shader program and add if to the map
            let shader_program = ShaderProgram::from_files(&path_slices)?;
            self.shader_programs.insert(key.to_string(), shader_program);

            // Retrieve a reference to the newly inserted shader to return it
            return self.shader_programs.get(key).ok_or_else(|| {
                anyhow!("Failed to retrieve newly compiled shader program: {}", key)
            });
        }
        Err(anyhow!("No shader found for key: {}", key))
    }

    pub fn shader_count(&self) -> usize {
        self.shaders.values().map(|shaders| shaders.len()).sum()
    }

    pub fn get_shader_keys(&self) -> Vec<String> {
        self.shaders.keys().map(|s| s.to_string()).collect()
    }

    pub fn get_shader_program_keys(&self) -> Vec<String> {
        self.shader_programs.keys().map(|sp| sp.to_string()).collect()
    }
}
