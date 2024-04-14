use shared_lib::gl_buffer::BufferObject;
use shared_lib::gl_prelude::{BufferType, BufferUsage};
use shared_lib::vertices::textured_vertex::TexturedVertex2D;

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

//////////////////////////////////////////////////////////////////////////////
// - VertexCreator -
//////////////////////////////////////////////////////////////////////////////

trait VertexCreator<T> {
    fn create_vertex(data: &[f32]) -> TexturedVertex2D;
}

// struct FullVertex;
// struct ColorlessVertex;
//
// impl VertexCreator<TexturedVertex3D> for FullVertex {
//     fn create_vertex(data: &[f32]) -> TexturedVertex2D {
//         TexturedVertex2D {
//             position: [data[0], data[1]],
//             color: [data[2], data[3], data[4], 1.0],
//             tex_coords: [data[5], data[6]],
//         }
//     }
// }
//
// impl VertexCreator for ColorlessVertex {
//     fn create_vertex(data: &[f32]) -> TexturedVertex2D {
//         TexturedVertex2D {
//             position: [data[0], data[1]],
//             color: [data[2], data[3], data[4], 1.0],
//             tex_coords: [data[2], data[3]],
//         }
//     }
// }
