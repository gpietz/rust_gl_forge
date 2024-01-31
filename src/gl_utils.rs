use anyhow::Result;

pub fn check_gl_error() -> Result<()> {
    let mut errors = Vec::new();

    loop {
        let error_code = unsafe { gl::GetError() };
        if error_code == gl::NO_ERROR {
            break;
        }

        let error_msg = match error_code {
            gl::INVALID_ENUM => "INVALID_ENUM",
            gl::INVALID_VALUE => "INVALID_VALUE",
            gl::INVALID_OPERATION => "INVALID_OPERATION",
            gl::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
            gl::STACK_OVERFLOW => "STACK_OVERFLOW",
            gl::STACK_UNDERFLOW => "STACK_UNDERFLOW",
            gl::OUT_OF_MEMORY => "OUT_OF_MEMORY",
            _ => "UNKNOWN_ERROR",
        };

        errors.push(format!("OpenGL error [{}]: {}", error_code, error_msg));
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(errors.join(", ")))
    }
}
