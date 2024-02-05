use crate::gl_buffer::BufferObject;
use crate::gl_draw;
use crate::gl_shader::{ShaderFactory, ShaderProgram};
use crate::gl_texture::Texture;
use crate::gl_traits::{Bindable, Deletable};
use crate::gl_types::{BufferType, BufferUsage, IndicesValueType, PrimitiveType};
use crate::gl_vertex::{TexturedVertex, Vertex, VertexArrayObject};
use crate::renderable::Renderable;
use anyhow::Result;

//////////////////////////////////////////////////////////////////////////////
// - TextureTriangle -
//////////////////////////////////////////////////////////////////////////////

pub struct TextureTriangle {
    vao: Option<VertexArrayObject>,
    vbo: Option<BufferObject<TexturedVertex>>,
    ibo: Option<BufferObject<u32>>,
    textures: [Texture; 2],
    shader: Option<ShaderProgram>,
    use_color: bool,
    use_color_location: i32,
    draw_quad: bool,
    vertex_count: u32,
}

impl TextureTriangle {
    pub fn new() -> Result<TextureTriangle> {
        let mut texture_triangle = TextureTriangle {
            vao: None,
            vbo: None,
            ibo: None,
            textures: [
                Texture::new("assets/textures/m-016-018-bg.jpg")?,
                Texture::new("assets/textures/container.jpg")?,
            ],
            shader: None,
            use_color: true,
            use_color_location: 0,
            draw_quad: false,
            vertex_count: 0,
        };

        // TODO Replace with something smarter
        texture_triangle
            .setup()
            .expect("Failed to create texture triangle drawable");

        Ok(texture_triangle)
    }

    fn get_texture(&self) -> &Texture {
        if self.draw_quad {
            &self.textures[1]
        } else {
            &self.textures[0]
        }
    }

    fn create_vertex_data(&self) -> Vec<TexturedVertex> {
        let vertices = if !self.draw_quad {
            vec![
                [-0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
                [0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0],
                [0.0, 0.5, 0.0, 0.0, 0.0, 1.0, 0.5, 1.0],
            ]
        } else {
            vec![
                [0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0],
                [0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0],
                [-0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
                [-0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0],
            ]
        };

        let mut vec = Vec::new();

        for vertex_data in &vertices {
            vec.push(TexturedVertex {
                position: [vertex_data[0], vertex_data[1], vertex_data[2]],
                color: [vertex_data[3], vertex_data[4], vertex_data[5], 1.0],
                tex_coords: [vertex_data[6], vertex_data[7]],
            });
        }

        vec
    }

    fn create_index_data(&mut self) -> Vec<u32> {
        if !self.draw_quad {
            vec![0, 1, 2]
        } else {
            vec![0, 1, 3, 1, 2, 3]
        }
    }
}

impl Renderable for TextureTriangle {
    fn setup(&mut self) -> Result<()> {
        let vertex_data = self.create_vertex_data();

        self.vao = Some(VertexArrayObject::new_and_bind()?);
        self.vbo = Some(BufferObject::new(
            BufferType::ArrayBuffer,
            BufferUsage::StaticDraw,
            vertex_data,
        ));

        let index_data = self.create_index_data();
        self.vertex_count = index_data.len() as u32;
        self.ibo = Some(BufferObject::new_and_bind(
            BufferType::ElementArrayBuffer,
            BufferUsage::StaticDraw,
            index_data,
        ));

        for attribute in TexturedVertex::attributes() {
            attribute.setup()?;
        }

        // Create shader program
        let mut shader = ShaderFactory::from_files(
            "assets/shaders/texture_triangle/vertexShader.glsl",
            "assets/shaders/texture_triangle/fragmentShader.glsl",
        )?;
        self.use_color_location = shader.get_uniform_location("useColor")?;
        self.shader = Some(shader);

        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        if let Some(vao) = self.vao.as_mut() {
            vao.bind()?;
        }
        if let Some(vbo) = self.vbo.as_mut() {
            vbo.bind()?;
        }
        if let Some(ibo) = self.ibo.as_mut() {
            ibo.bind()?;
        }
        if !self.draw_quad {
            self.textures[0].bind();
        } else {
            self.textures[1].bind();
        }
        if let Some(shader) = self.shader.as_mut() {
            shader.bind();
            shader
                .set_uniform_value(self.use_color_location, self.use_color)
                .unwrap(); // TODO draw() function should return a Result<()> instead of unwrapping!
        }
        gl_draw::draw_elements(
            PrimitiveType::Triangles,
            self.vertex_count,
            IndicesValueType::Int,
        );
        Ok(())
    }

    fn clean_up(&mut self) -> Result<()> {
        if let Some(shader) = self.shader.as_mut() {
            shader.delete()?;
            self.shader = None;
        }
        if let Some(ibo) = self.ibo.as_mut() {
            ibo.delete()?;
            self.ibo = None;
        }
        if let Some(vbo) = self.vbo.as_mut() {
            vbo.delete()?;
            self.vbo = None;
        }
        if let Some(vao) = self.vao.as_mut() {
            vao.delete()?;
            self.vao = None;
        }
        Ok(())
    }

    fn toggle_mode(&mut self) {
        self.use_color = !self.use_color;
    }

    fn toggle_shape(&mut self) {
        self.draw_quad = !self.draw_quad;
        self.clean_up().unwrap(); // Expect should be used here!
        self.setup().unwrap();
    }
}
