use anyhow::Result;
use shared_lib::{
    gl_buffer::BufferObject,
    gl_shader::ShaderProgram,
    gl_texture::Texture,
    gl_vertex::{TexturedVertex, VertexArrayObject},
};

//////////////////////////////////////////////////////////////////////////////
// - Transformation  -
//////////////////////////////////////////////////////////////////////////////

pub struct Transformation {
    // vao: Option<VertexArrayObject>,
    // vbo: Option<BufferObject<TexturedVertex>>,
    // ibo: Option<BufferObject<u32>>,
    // textures: [Texture; 3],
    // shader: Option<ShaderProgram>,
}

impl Transformation {
    pub fn new() -> Result<Transformation> {
        // let vertices = vec![
        //     [0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0],
        //     [0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0],
        //     [-0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        //     [-0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0],
        // ];

        // let mut vertex_data = Vec::new();
        // for vertex_data in &vertices {
        //     vertex_data.push(TexturedVertex {
        //         position: [vertex_data[0], vertex_data[1], vertex_data[2]],
        //         color: [vertex_data[3], vertex_data[4], vertex_data[5], 1.0],
        //         tex_coords: [vertex_data[6], vertex_data[7]],
        //     });
        // }

        // let vao = VertexArrayObject::new_and_bind()?);
        // vbo = Some(BufferObject::new(
        //     BufferType::ArrayBuffer,
        //     BufferUsage::StaticDraw,
        //     vertex_data,
        // ));

        Ok(Transformation {})
    }
}
