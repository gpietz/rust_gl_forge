use crate::renderable::Renderable;
use crate::texture_utils::*;
use anyhow::Result;
use shared_lib::gl_buffer::BufferObject;
use shared_lib::gl_draw;
use shared_lib::gl_shader::{ShaderFactory, ShaderProgram};
use shared_lib::gl_texture::Texture;
use shared_lib::gl_traits::{Bindable, Deletable};
use shared_lib::gl_types::{IndicesValueType, PrimitiveType};
use shared_lib::gl_vertex::{TexturedVertex, VertexArrayObject};

//////////////////////////////////////////////////////////////////////////////
// - TextureTriangle -
//////////////////////////////////////////////////////////////////////////////

pub struct TextureTriangle {
    vao: Option<VertexArrayObject>,
    vbo: Option<BufferObject<TexturedVertex>>,
    ibo: Option<BufferObject<u32>>,
    textures: [Texture; 3],
    shader: Option<ShaderProgram>,
    use_color: bool,
    use_color_location: i32,
    draw_quad: bool,
    vertex_count: u32,
    use_awesomeface: bool,
    use_awesomeface_location: i32,
    setup_called: bool,
}

impl TextureTriangle {
    pub fn new() -> Result<TextureTriangle> {
        let mut texture_triangle = TextureTriangle {
            vao: None,
            vbo: None,
            ibo: None,
            textures: [
                create_texture("assets/textures/m-016-018-bg.jpg", false, false)?,
                create_texture("assets/textures/container.jpg", false, false)?,
                create_texture("assets/textures/awesomeface2.png", true, true)?,
            ],
            shader: None,
            use_color: true,
            use_color_location: 0,
            draw_quad: false,
            vertex_count: 0,
            use_awesomeface: false,
            use_awesomeface_location: 0,
            setup_called: false,
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

    fn print_render_mode(&self) {
        if !self.draw_quad {
            println!("Rendering triangle");
        } else if self.use_awesomeface {
            println!("Rendering quad with awesome face");
        } else {
            println!("Rendering quad");
        }
    }

    fn print_color_mode(&self) {
        let color_mode = if self.use_color { "ON" } else { "OFF" };
        println!("Vertex coloring: {color_mode}");
    }
}

impl Renderable for TextureTriangle {
    fn setup(&mut self) -> Result<()> {
        let vertex_data = if self.draw_quad {
            crate::vertex_data::create_quad()
        } else {
            crate::vertex_data::create_triangle()
        };

        self.vertex_count = vertex_data.indices.len() as u32;
        self.vao = Some(VertexArrayObject::new_and_bind()?);
        self.vbo = Some(vertex_data.create_vbo());
        self.ibo = Some(vertex_data.create_ibo());
        vertex_data.set_vertex_attributes();

        // Create shader program
        let mut shader = ShaderFactory::from_files(
            "assets/shaders/texture_triangle/vertexShader.glsl",
            "assets/shaders/texture_triangle/fragmentShader.glsl",
        )?;

        self.use_color_location = shader.get_uniform_location("useColor")?;
        self.use_awesomeface_location = shader.get_uniform_location("useTexture2")?;
        self.shader = Some(shader);

        self.print_render_mode();

        if !self.setup_called {
            self.setup_called = true;
            self.print_color_mode();
        }

        Ok(())
    }

    fn draw(&mut self, _: f32) -> Result<()> {
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
        } else if !self.use_awesomeface {
            self.textures[1].bind();
        } else {
            self.textures[1].bind_as_unit(0);
            self.textures[2].bind_as_unit(1);
        }

        if let Some(shader) = self.shader.as_mut() {
            shader.bind();
            shader.set_uniform("texture1", 0)?;
            shader.set_uniform("texture2", 1)?;

            shader
                .set_uniform_value(self.use_color_location, self.use_color)
                .unwrap(); // TODO draw() function should return a Result<()> instead of unwrapping!
            shader
                .set_uniform_value(self.use_awesomeface_location, self.use_awesomeface)
                .unwrap(); // TODO draw() function should return a Result<()> instead of unwrapping!
        } else {
            panic!("Shader progam is not available!");
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
        self.print_color_mode();
    }

    fn toggle_shape(&mut self) {
        if self.draw_quad && !self.use_awesomeface {
            self.use_awesomeface = true;
        } else {
            self.draw_quad = !self.draw_quad;
            self.use_awesomeface = false;
        }

        self.clean_up().unwrap(); // Expect should be used here!
        self.setup().unwrap();
    }
}
