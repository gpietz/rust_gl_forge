use crate::gl_shader::{ShaderFactory, ShaderProgram};
use anyhow::Result;
use std::collections::HashMap;

pub struct ShaderManager {
    shaders: HashMap<String, ShaderProgram>,
}

impl ShaderManager {
    pub fn new() -> Self {
        ShaderManager {
            shaders: HashMap::new(),
        }
    }

    pub fn load_shader<S: AsRef<str>>(
        &mut self,
        name: S,
        vertex_shader: S,
        fragment_shader: S,
    ) -> Result<u32> {
        let name = name.as_ref().to_string();
        let shader_filenames = [
            vertex_shader.as_ref().to_string(),
            fragment_shader.as_ref().to_string(),
        ];

        let shader = ShaderFactory::from_files(&shader_filenames[0], &shader_filenames[1])?;
        let shader_program_id = shader.program_id();
        self.shaders.insert(name, shader);

        Ok(shader_program_id)
    }

    pub fn get_shader_by_name<S: AsRef<str>>(&self, name: &S) -> Option<&ShaderProgram> {
        let name = name.as_ref().to_string();
        self.shaders.get(&name)
    }

    pub fn get_shader_by_id(&self, shader_id: u32) -> Option<&ShaderProgram> {
        for (_, shader) in self.shaders.iter() {
            if shader.program_id() == shader_id {
                return Some(&shader);
            }
        }
        None
    }
}
