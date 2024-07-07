use shared_lib::gl_types::{BufferType, BufferUsage};
use shared_lib::opengl::buffer_object::BufferObject;
use shared_lib::vertices::textured_vertex::TexturedVertex;

pub(crate) const CUBE_VERTEX_DATA: [(f32, f32, f32); 36] = [
    (-0.5, -0.5, -0.5),
    (0.5, -0.5, -0.5),
    (0.5, 0.5, -0.5),
    (0.5, 0.5, -0.5),
    (-0.5, 0.5, -0.5),
    (-0.5, -0.5, -0.5),
    (-0.5, -0.5, 0.5),
    (0.5, -0.5, 0.5),
    (0.5, 0.5, 0.5),
    (0.5, 0.5, 0.5),
    (-0.5, 0.5, 0.5),
    (-0.5, -0.5, 0.5),
    (-0.5, 0.5, 0.5),
    (-0.5, 0.5, -0.5),
    (-0.5, -0.5, -0.5),
    (-0.5, -0.5, -0.5),
    (-0.5, -0.5, 0.5),
    (-0.5, 0.5, 0.5),
    (0.5, 0.5, 0.5),
    (0.5, 0.5, -0.5),
    (0.5, -0.5, -0.5),
    (0.5, -0.5, -0.5),
    (0.5, -0.5, 0.5),
    (0.5, 0.5, 0.5),
    (-0.5, -0.5, -0.5),
    (0.5, -0.5, -0.5),
    (0.5, -0.5, 0.5),
    (0.5, -0.5, 0.5),
    (-0.5, -0.5, 0.5),
    (-0.5, -0.5, -0.5),
    (-0.5, 0.5, -0.5),
    (0.5, 0.5, -0.5),
    (0.5, 0.5, 0.5),
    (0.5, 0.5, 0.5),
    (-0.5, 0.5, 0.5),
    (-0.5, 0.5, -0.5),
];

pub(crate) const CUBE_TEXTURE_DATA: [(f32, f32); 36] = [
    (0.0, 0.0),
    (1.0, 0.0),
    (1.0, 1.0),
    (1.0, 1.0),
    (0.0, 1.0),
    (0.0, 0.0),
    (0.0, 0.0),
    (1.0, 0.0),
    (1.0, 1.0),
    (1.0, 1.0),
    (0.0, 1.0),
    (0.0, 0.0),
    (1.0, 0.0),
    (1.0, 1.0),
    (0.0, 1.0),
    (0.0, 1.0),
    (0.0, 0.0),
    (1.0, 0.0),
    (1.0, 0.0),
    (1.0, 1.0),
    (0.0, 1.0),
    (0.0, 1.0),
    (0.0, 0.0),
    (1.0, 0.0),
    (0.0, 1.0),
    (1.0, 1.0),
    (1.0, 0.0),
    (1.0, 0.0),
    (0.0, 0.0),
    (0.0, 1.0),
    (0.0, 1.0),
    (1.0, 1.0),
    (1.0, 0.0),
    (1.0, 0.0),
    (0.0, 0.0),
    (0.0, 1.0),
];

pub(crate) fn create_textured_vertices(vertices: &Vec<[f32; 5]>) -> Vec<TexturedVertex> {
    let mut textured_vertices = Vec::new();

    for vertex_data in vertices {
        let textured_vertex = TexturedVertex {
            position: [vertex_data[0], vertex_data[1], vertex_data[2]],
            tex_coords: [vertex_data[3], vertex_data[4]],
            ..Default::default()
        };
        textured_vertices.push(textured_vertex);
    }

    textured_vertices
}

pub(crate) fn create_cube() -> Vec<TexturedVertex> {
    let vertices = CUBE_VERTEX_DATA
        .iter()
        .zip(CUBE_TEXTURE_DATA.iter())
        .map(|(&(x, y, z), &(u, v))| [x, y, z, u, v])
        .collect();
    create_textured_vertices(&vertices)
}

pub(crate) fn create_vbo(vertices: Vec<TexturedVertex>) -> BufferObject<TexturedVertex> {
    BufferObject::new(
        BufferType::ArrayBuffer,
        BufferUsage::StaticDraw,
        vertices.clone(),
    )
}
