use shared_lib::gl_prelude::{BufferType, BufferUsage};
use shared_lib::opengl::buffer_object::BufferObject;

//////////////////////////////////////////////////////////////////////////////
// - VertexData -
//////////////////////////////////////////////////////////////////////////////

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
    pub(crate) fn create_vbo(&self) -> BufferObject<T> {
        BufferObject::new(
            BufferType::ArrayBuffer,
            BufferUsage::StaticDraw,
            self.vertices.clone(),
        )
    }

    pub(crate) fn create_ibo(&self) -> BufferObject<u32> {
        BufferObject::new(
            BufferType::ElementArrayBuffer,
            BufferUsage::StaticDraw,
            self.indices.clone(),
        )
    }
}
