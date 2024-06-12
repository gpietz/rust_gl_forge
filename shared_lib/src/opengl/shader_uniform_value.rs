use gl::types::GLint;

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