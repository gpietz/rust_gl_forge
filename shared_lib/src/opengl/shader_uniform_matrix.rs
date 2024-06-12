use cgmath::Matrix;
use gl::types::GLboolean;

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
