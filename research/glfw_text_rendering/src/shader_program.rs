use anyhow::{anyhow, Result};
use gl::types::{GLchar, GLenum, GLint, GLuint};
use std::collections::HashMap;
use std::ffi::CString;
use std::fmt::{Display, Formatter};
use std::{fs, ptr};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ShaderType {
    Vertex,
    Fragment,
}

impl Display for ShaderType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ShaderType::Vertex => write!(f, "Vertex"),
            ShaderType::Fragment => write!(f, "Fragment"),
        }
    }
}

impl Into<GLenum> for ShaderType {
    fn into(self) -> GLenum {
        match self {
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
        }
    }
}

pub struct ShaderProgram {
    pub id: i32,
    shader_sources: HashMap<ShaderType, String>,
    shader_files: HashMap<ShaderType, String>,
}

impl ShaderProgram {
    pub fn new() -> Self {
        ShaderProgram {
            id: 0,
            shader_sources: HashMap::new(),
            shader_files: HashMap::new(),
        }
    }

    pub fn with_file(&mut self, r#type: ShaderType, file: &str) -> Result<&mut Self> {
        if self.is_type_defined(&r#type) {
            return Err(anyhow!("ShaderType already defined: {}", r#type));
        }
        self.shader_files.insert(r#type, file.to_string());
        Ok(self)
    }

    pub fn with_source(&mut self, r#type: ShaderType, source: &str) -> Result<&mut Self> {
        if self.is_type_defined(&r#type) {
            return Err(anyhow!("ShaderType already defined: {}", r#type));
        }
        self.shader_sources.insert(r#type, source.to_string());
        Ok(self)
    }

    pub fn is_type_defined(&self, r#type: &ShaderType) -> bool {
        self.shader_sources.contains_key(r#type) || self.shader_files.contains_key(r#type)
    }

    pub fn compile(&mut self) -> Result<&mut Self> {
        let mut shader_sources: HashMap<ShaderType, CString> = HashMap::new();
        for shader_file in &self.shader_files {
            let source = fs::read_to_string(shader_file.1)?;
            let source = CString::new(source.as_bytes())?;
            shader_sources.insert(shader_file.0.clone(), source);
        }
        for shader_source in &self.shader_sources {
            let source = CString::new(shader_source.1.as_bytes())?;
            shader_sources.insert(shader_source.0.clone(), source);
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

            self.id = shader_program as i32;
        }

        Ok(self)
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
                    let error_message =
                        std::str::from_utf8(&info_log).unwrap_or_else(|_| "Failed to read log");
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
                    let error_message =
                        std::str::from_utf8(&info_log).unwrap_or_else(|_| "Failed to read log");
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
