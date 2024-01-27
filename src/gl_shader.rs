use crate::gl_types::ShaderType;
use crate::string_utils;
use anyhow::{anyhow, Context, Result};
use gl::types::{GLchar, GLint, GLuint};
use std::ffi::CString;
use std::ptr;

//////////////////////////////////////////////////////////////////////////////
// - Shader -
//////////////////////////////////////////////////////////////////////////////

pub struct Shader {
    id: u32,
}

impl Shader {
    pub fn from_source(source: &str, shader_type: ShaderType) -> Result<Shader> {
        let id = unsafe {
            let shader = gl::CreateShader(shader_type.to_gl_enum());
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
                let error = string_utils::create_whitespace_cstring_with_len(len as usize);
                gl::GetShaderInfoLog(shader, len, ptr::null_mut(), error.as_ptr() as *mut GLchar);
                return Err(anyhow!(error.into_string().unwrap()));
            }
            shader
        };
        Ok(Shader { id })
    }

    pub fn get_shader_id(&self) -> u32 {
        self.id
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
            gl::DeleteShader(self.id);
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// - ShaderProgram -
//////////////////////////////////////////////////////////////////////////////

pub struct ShaderProgram {
    id: u32,
}

impl ShaderProgram {
    pub fn new(vertex_shader: &Shader, fragment_shader: &Shader) -> Result<ShaderProgram> {
        let program_id = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(program_id, vertex_shader.get_shader_id());
            gl::AttachShader(program_id, fragment_shader.get_shader_id());
            gl::LinkProgram(program_id);

            // Check for linking erros
            let mut success = gl::FALSE as GLint;
            gl::GetShaderiv(program_id, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                let mut len = 0;
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
                let error = string_utils::create_whitespace_cstring_with_len(len as usize);
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    ptr::null_mut(),
                    error.as_ptr() as *mut GLchar,
                );
            }
        }

        Ok(ShaderProgram { id: program_id })
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
