use anyhow::Result;
use gl::types::GLenum;

use crate::check_gl_panic;
use crate::gl_utils::check_gl_error;

pub struct BlendGuard {
    original_blend: bool,
    blend_src: GLenum,
    blend_dest: GLenum,
    callback: Option<Box<dyn Fn(bool) -> bool>>,
}

impl BlendGuard {
    pub fn new(blend_src: GLenum, blend_dest: GLenum) -> Result<Self> {
        let mut original_blend = false;
        unsafe {
            original_blend = gl::IsEnabled(gl::BLEND) == gl::TRUE;
            gl::Enable(gl::BLEND);
            check_gl_panic!("Error enabling GL blend");
            gl::BlendFunc(blend_src, blend_dest);
            check_gl_panic!("Error calling blend function");
        }
        check_gl_error()?;
        Ok(BlendGuard {
            original_blend,
            blend_src,
            blend_dest,
            callback: None,
        })
    }

    pub fn enable(&self) -> Result<()> {
        if self.call_callback(true) {
            unsafe {
                gl::Enable(gl::BLEND);
                gl::BlendFunc(self.blend_src, self.blend_dest);
            }
            check_gl_error()?;
        }
        Ok(())
    }

    pub fn disable(&self) -> Result<()> {
        if self.call_callback(false) {
            unsafe {
                gl::Disable(gl::BLEND);
            }
            check_gl_error()?;
        }
        Ok(())
    }

    pub fn set_blend_func(&mut self, src: GLenum, dest: GLenum) -> Result<()> {
        unsafe {
            gl::BlendFunc(src, dest);
            check_gl_panic!("Error calling blend function");
        }
        check_gl_error()?;
        self.blend_src = src;
        self.blend_dest = dest;
        Ok(())
    }

    pub fn get_blend_func(&self) -> (GLenum, GLenum) {
        (self.blend_src, self.blend_dest)
    }

    pub fn set_callback<F: 'static + Fn(bool) -> bool>(&mut self, callback: F) {
        self.callback = Some(Box::new(callback));
    }

    pub fn clear_callback(&mut self) {
        self.callback = None;
    }

    fn call_callback(&self, enable: bool) -> bool {
        match &self.callback {
            Some(cb) => cb(enable),
            None => true,
        }
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { 
            gl::IsEnabled(gl::BLEND) == gl::TRUE 
        }
    }
}

impl Default for BlendGuard {
    fn default() -> Self {
        Self::new(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA)
            .expect("Failed to create default BlendGuard")
    }
}

impl Drop for BlendGuard {
    fn drop(&mut self) {
        if !self.original_blend {
            if let Err(e) = self.disable() {
                eprintln!("Error disabling blend in BlendGuard: {}", e);
            }
        }
    }
}

unsafe impl Send for BlendGuard {}
unsafe impl Sync for BlendGuard {}
