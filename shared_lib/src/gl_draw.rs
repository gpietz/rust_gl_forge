use crate::gl_types::{IndicesValueType, PrimitiveType};
use gl::types::{GLint, GLsizei, GLuint};
use std::ptr;

/// Draws geometric primitives from array data.
///
/// This function wraps the OpenGL `glDrawArrays` call, providing a safe and Rust-friendly interface.
/// It draws primitives (e.g., points, lines, triangles) based on the vertex data previously defined.
///
/// # Parameters
/// - `primitive_type`: The type of primitives to render. This is specified using the `PrimitiveType` enum,
///   which abstracts over OpenGL's primitive types (e.g., `GL_TRIANGLES`, `GL_LINES`).
/// - `vertex_count`: The number of vertices to be rendered. This count should match the number of vertices
///   available for the specified primitive type to ensure correct rendering. It is converted internally
///   to a `GLsizei` to match OpenGL's expected parameter type.
///
/// # Safety
/// This function contains an `unsafe` block due to the direct call to the OpenGL function `gl::DrawArrays`.
/// It is the caller's responsibility to ensure that all OpenGL state required for the draw call (e.g., the
/// current shader program, bound vertex array object, and vertex buffer objects) has been correctly set up
/// before calling this function. Failure to do so may lead to undefined behavior or a crash.
///
/// # Examples
/// Assuming you have set up a shader program, VAO, and VBO correctly:
/// ```no-run
/// let primitive_type = PrimitiveType::Triangles;
/// let vertex_count = 3; // Drawing one triangle with 3 vertices
/// draw_primitive(primitive_type, vertex_count);
/// ```
///
/// # Note
/// The starting index is fixed at 0, meaning drawing always starts from the first vertex in the buffer.
/// This function does not support specifying an offset into the vertex array. If such functionality is
/// needed, consider using `glDrawArraysInstanced` or `glDrawElements` with an index buffer.
pub fn draw_primitive(primitive_type: PrimitiveType, vertex_count: u32) {
    unsafe {
        gl::DrawArrays(primitive_type.to_gl_enum(), 0, vertex_count as GLsizei);
    }
}

/// Draws elements using OpenGL.
///
/// This function performs an OpenGL draw call using the provided parameters.
///
/// # Parameters
///
/// - `primitive_type`: The type of primitive to be drawn (e.g., Triangles, Lines, etc.).
/// - `elements_count`: The number of elements to be drawn.
/// - `indices_type`: The type of indices used for drawing.
///
/// # Safety
///
/// This function is marked as `unsafe` because it directly calls OpenGL functions,
/// and it is the responsibility of the caller to ensure that the OpenGL context is
/// properly initialized and that the provided parameters are valid.
///
/// # Example
///
/// ```no-run
/// # use your_module_name::draw_elements;
/// # use your_module_name::PrimitiveType;
/// # use your_module_name::IndicesValueType;
///
/// let primitive_type = PrimitiveType::Triangles;
/// let elements_count = 36;
/// let indices_type = IndicesValueType::UnsignedInt;
///
/// unsafe {
///     draw_elements(primitive_type, elements_count, indices_type);
/// }
/// ```
pub fn draw_elements(
    primitive_type: PrimitiveType,
    elements_count: u32, //<< TODO usize
    indices_type: IndicesValueType,
) {
    unsafe {
        gl::DrawElements(
            primitive_type.to_gl_enum(),
            elements_count as GLsizei,
            indices_type.to_gl_enum(),
            ptr::null(),
        );
    }
}

pub fn draw_arrays(primitive_type: PrimitiveType, first: usize, count: usize) {
    unsafe {
        gl::DrawArrays(
            primitive_type.to_gl_enum(),
            first as GLint,
            count as GLsizei,
        )
    }
}
