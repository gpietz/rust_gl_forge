use shared_lib::gl_prelude::{BufferType, BufferUsage};
use shared_lib::opengl::buffer_object::BufferObject;
use shared_lib::opengl::vertex_array_object::VertexArrayObject;

pub(crate) struct VertexData<T>
where
    T: Clone,
{
    pub vertices: Vec<T>,
    pub indices: Vec<u32>,
}

impl<T> VertexData<T>
where
    T: Clone,
{
    pub(crate) fn create_vbo(&self, vao: &VertexArrayObject) -> BufferObject<T> {
        BufferObject::new_with_vao(
            vao,
            BufferType::ArrayBuffer,
            BufferUsage::StaticDraw,
            self.vertices.clone(),
        )
    }

    pub(crate) fn create_ibo(&self, vao: &VertexArrayObject) -> BufferObject<u32> {
        BufferObject::new_with_vao(
            vao,
            BufferType::ElementArrayBuffer,
            BufferUsage::StaticDraw,
            self.indices.clone(),
        )
    }
    
    pub(crate) fn has_indices(&self) -> bool {
        !self.indices.is_empty()
    }
}
