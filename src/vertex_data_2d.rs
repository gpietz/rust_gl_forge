use crate::vertex_data::VertexData;
use shared_lib::vertices::textured_vertex::TexturedVertex;

pub(crate) fn create_textured_vertices(vertices: &Vec<[f32; 7]>) -> Vec<TexturedVertex> {
    let mut textured_vertices = Vec::new();

    for vertex_data in vertices {
        let textured_vertex = TexturedVertex {
            position: [vertex_data[0], vertex_data[1], 0.0],
            color: [vertex_data[2], vertex_data[3], vertex_data[4], 1.0],
            tex_coords: [vertex_data[5], vertex_data[6]],
        };
        textured_vertices.push(textured_vertex);
    }

    textured_vertices
}

pub(crate) fn create_triangle() -> VertexData<TexturedVertex> {
    let vertices = vec![
        [-0.5, -0.5, 1.0, 0.0, 0.0, 0.0, 0.0],
        [0.5, -0.5, 0.0, 1.0, 0.0, 1.0, 0.0],
        [0.0, 0.5, 0.0, 0.0, 1.0, 0.5, 1.0],
    ];

    VertexData {
        vertices: create_textured_vertices(&vertices),
        indices: vec![0, 1, 2],
    }
}

pub(crate) fn create_quad() -> VertexData<TexturedVertex> {
    let vertices = vec![
        [0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 1.0],
        [0.5, -0.5, 0.0, 1.0, 0.0, 1.0, 0.0],
        [-0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 0.0],
        [-0.5, 0.5, 1.0, 1.0, 0.0, 0.0, 1.0],
    ];

    VertexData {
        vertices: create_textured_vertices(&vertices),
        indices: vec![0, 1, 3, 1, 2, 3],
    }
}
