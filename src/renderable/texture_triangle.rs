use crate::gl_buffer::BufferObject;
use crate::gl_shader::{Shader, ShaderProgram};
use crate::gl_texture::Texture;
use crate::gl_types::{BufferType, BufferUsage, ShaderType};
use crate::gl_vertex::{TexturedVertex, Vertex, VertexArrayObject};
use crate::renderable::Renderable;
use anyhow::Result;

struct MyVertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

//////////////////////////////////////////////////////////////////////////////
// - TextureTriangle -
//////////////////////////////////////////////////////////////////////////////

pub struct TextureTriangle {
    vao: VertexArrayObject,
    vbo: BufferObject<TexturedVertex>,
    texture: Texture,
    shader: ShaderProgram,
}

impl TextureTriangle {
    pub fn new() -> Result<TextureTriangle> {
        // Load the texture
        let texture = Texture::new("assets/textures/m-016-018-bg.jpg")?;

        // Setup vertices
        let vertices = vec![
            TexturedVertex {
                position: [-0.5, -0.5, 0.0], // left vertex position
                tex_coords: [0.0, 0.0],      // left vertex texture coordinates
            },
            TexturedVertex {
                position: [0.5, -0.5, 0.0], // right vertex position
                tex_coords: [1.0, 0.0],     // right vertex texture coordinates
            },
            TexturedVertex {
                position: [0.0, 0.5, 0.0], // top vertex position
                tex_coords: [0.5, 1.0],    // top vertex texture coordinates
            },
        ];

        let vao = VertexArrayObject::new()?;
        vao.bind();

        let vbo = BufferObject::new(BufferType::ArrayBuffer, BufferUsage::StaticDraw, vertices);
        vbo.bind();

        for attribute in TexturedVertex::attributes() {
            attribute.setup()?;
        }

        // Load shaders
        #[rustfmt::skip]
        let mut vertex_shader = Shader::from_file("assets/shaders/texture_triangle/vertexShader.glsl", ShaderType::Vertex)?;
        #[rustfmt::skip]
        let mut fragment_shader = Shader::from_file("assets/shaders/texture_triangle/fragmentShader.glsl", ShaderType::Fragment)?;

        // Create the shader program
        let shader = ShaderProgram::new(&mut vertex_shader, &mut fragment_shader)?;

        Ok(TextureTriangle {
            vao,
            vbo,
            texture,
            shader,
        })
    }
}

impl Renderable for TextureTriangle {
    fn draw(&mut self) {
        unsafe {
            self.vao.bind();
            self.vbo.bind();
            self.texture.bind();
            self.shader.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}
