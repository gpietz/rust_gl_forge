use shared_lib::gl_buffer::BufferObject;
use shared_lib::gl_draw;
use shared_lib::gl_shader::{ShaderFactory, ShaderProgram};
use shared_lib::gl_texture::{Texture, TextureBuilder};
use shared_lib::gl_traits::{Bindable, Deletable};
use shared_lib::gl_types::{BufferType, BufferUsage, IndicesValueType, PrimitiveType};
use shared_lib::gl_vertex::{TexturedVertex, Vertex, VertexArrayObject};
use crate::renderable::Renderable;
use anyhow::Result;

fn create_texture(path: &str, has_alpha: bool, flip_vertical: bool) -> Result<Texture> {
    TextureBuilder::default()
        .path(path)
        .has_alpha(has_alpha)
        .flip_vertical(flip_vertical)
        .build()
}

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
            setup_called: false
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

    fn print_render_mode(&self) {
        if !self.draw_quad {
            println!("Rendering triangle");
        } else if self.use_awesomeface {
            println!("Rendering quad with awesome face");
        } else  {
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
        self.use_awesomeface_location = shader.get_uniform_location("useTexture2")?;
        self.shader = Some(shader);
    
        self.print_render_mode();

        if !self.setup_called {
            self.setup_called = true;
            self.print_color_mode();
        }

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
            if !self.use_awesomeface {
                self.textures[1].bind();
            } else {
                self.textures[1].bind_as_unit(0);
                self.textures[2].bind_as_unit(1);
            }
        }

        if let Some(shader) = self.shader.as_mut() {
            shader.bind();
            shader.set_uniform("texture1",  0)?;
            shader.set_uniform("texture2",  1)?;

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
