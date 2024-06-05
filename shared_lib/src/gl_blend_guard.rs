pub struct BlendGuard {
    original_blend: bool,
}

impl BlendGuard {
    fn new() -> Self {
        let original_blend: bool = unsafe { gl::IsEnabled(gl::BLEND) == gl::TRUE };
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
        BlendGuard { original_blend }
    }

    pub fn enable(&self) {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }

    pub fn disable(&self) {
        unsafe {
            gl::Disable(gl::BLEND);
        }
    }
}

impl Default for BlendGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for BlendGuard {
    fn drop(&mut self) {
        if !self.original_blend {
            self.disable();
        }
    }
}
