use shared_lib::{
    gl_buffer::BufferObject,
    gl_types::{BufferType, BufferUsage},
    gl_vertex::{TexturedVertex, Vertex},
};

pub(crate) struct VertexData {
    pub vertices: Vec<TexturedVertex>,
    pub indices: Vec<u32>,
}

impl VertexData {
    pub(crate) fn create_vbo(&self) -> BufferObject<TexturedVertex> {
        BufferObject::new_and_bind(
            BufferType::ArrayBuffer,
            BufferUsage::StaticDraw,
            self.vertices.clone(),
        )
    }

    pub(crate) fn create_ibo(&self) -> BufferObject<u32> {
        BufferObject::new_and_bind(
            BufferType::ElementArrayBuffer,
            BufferUsage::StaticDraw,
            self.indices.clone(),
        )
    }

    pub(crate) fn set_vertex_attributes(&self) {
        for attribute in TexturedVertex::attributes() {
            attribute.setup().expect("Failed to set vertex attributes");
        }
    }
}

pub(crate) fn create_textured_vertices(vertices: &Vec<[f32; 8]>) -> Vec<TexturedVertex> {
    let mut textured_vertices = Vec::new();

    for vertex_data in vertices {
        let textured_vertex = TexturedVertex {
            position: [vertex_data[0], vertex_data[1], vertex_data[2]],
            color: [vertex_data[3], vertex_data[4], vertex_data[5], 1.0],
            tex_coords: [vertex_data[6], vertex_data[7]],
        };
        textured_vertices.push(textured_vertex);
    }

    textured_vertices
}

pub(crate) fn create_triangle() -> VertexData {
    let vertices = vec![
        [-0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
        [0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0],
        [0.0, 0.5, 0.0, 0.0, 0.0, 1.0, 0.5, 1.0],
    ];

    VertexData {
        vertices: create_textured_vertices(&vertices),
        indices: vec![0, 1, 2],
    }
}

pub(crate) fn create_quad() -> VertexData {
    let vertices = vec![
        [0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0],
        [0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0],
        [-0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        [-0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0],
    ];

    VertexData {
        vertices: create_textured_vertices(&vertices),
        indices: vec![0, 1, 3, 1, 2, 3],
    }
}
