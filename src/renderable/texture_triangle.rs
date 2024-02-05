use crate::gl_buffer::BufferObject;
use crate::gl_draw;
use crate::gl_shader::{ShaderFactory, ShaderProgram};
use crate::gl_texture::Texture;
use crate::gl_types::{BufferType, BufferUsage, PrimitiveType};
use crate::gl_vertex::{TexturedVertex, Vertex, VertexArrayObject};
use crate::renderable::Renderable;
use anyhow::Result;

//////////////////////////////////////////////////////////////////////////////
// - TextureTriangle -
//////////////////////////////////////////////////////////////////////////////

pub struct TextureTriangle {
    vao: VertexArrayObject,
    vbo: BufferObject<TexturedVertex>,
    texture: Texture,
    shader: ShaderProgram,
    use_color: bool,
    use_color_location: i32,
}

impl TextureTriangle {
    pub fn new() -> Result<TextureTriangle> {
        // Load the texture
        let texture = Texture::new("assets/textures/m-016-018-bg.jpg")?;

        // Setup vertices
        let vertices = vec![
            TexturedVertex {
                position: [-0.5, -0.5, 0.0], // left vertex position
                color: [1.0, 0.0, 0.0, 1.0], // left vertex color
                tex_coords: [0.0, 0.0],      // left vertex texture coordinates
            },
            TexturedVertex {
                position: [0.5, -0.5, 0.0],  // right vertex position
                color: [0.0, 1.0, 0.0, 1.0], // right vertex color
                tex_coords: [1.0, 0.0],      // right vertex texture coordinates
            },
            TexturedVertex {
                position: [0.0, 0.5, 0.0],   // top vertex position
                color: [0.0, 0.0, 1.0, 1.0], // top vertex color
                tex_coords: [0.5, 1.0],      // top vertex texture coordinates
            },
        ];

        let vao = VertexArrayObject::new()?;
        vao.bind();

        let vbo = BufferObject::new(BufferType::ArrayBuffer, BufferUsage::StaticDraw, vertices);
        vbo.bind();

        for attribute in TexturedVertex::attributes() {
            attribute.setup()?;
        }

        // Create shader program
        let mut shader = ShaderFactory::from_files(
            "assets/shaders/texture_triangle/vertexShader.glsl",
            "assets/shaders/texture_triangle/fragmentShader.glsl",
        )?;
        let use_color_location = shader.get_uniform_location("useColor")?;

        Ok(TextureTriangle {
            vao,
            vbo,
            texture,
            shader,
            use_color: true,
            use_color_location,
        })
    }
}

impl Renderable for TextureTriangle {
    fn draw(&mut self) {
        self.vao.bind();
        self.vbo.bind();
        self.texture.bind();
        self.shader.bind();
        self.shader
            .set_uniform_value(self.use_color_location, self.use_color)
            .unwrap(); // TODO draw() function should return a Result<()> instead of unwrapping!
        gl_draw::draw_primitive(PrimitiveType::Triangles, 3);
    }

    fn toggle_mode(&mut self) {
        self.use_color = !self.use_color;
    }
}
