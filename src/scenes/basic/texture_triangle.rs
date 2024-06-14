use std::fmt::{Display, Formatter};

use sdl2::keyboard::Keycode;

use shared_lib::gl_draw;
use shared_lib::gl_types::{IndicesValueType, PrimitiveType};
use shared_lib::opengl::buffer_object::BufferObject;
use shared_lib::opengl::texture::Texture;
use shared_lib::opengl::vertex_array_object::VertexArrayObject;
use shared_lib::opengl::vertex_layout_manager::VertexLayoutManager;
use shared_lib::sdl_window::SdlKeyboardState;
use shared_lib::vertices::textured_vertex::TexturedVertex;

use crate::render_context::RenderContext;
use crate::resources::{shaders, textures};
use crate::scene::{Scene, SceneResult};
use crate::scene_utils::query_texture;
use crate::vertex_data_2d;

//////////////////////////////////////////////////////////////////////////////
// - TextureTriangle -
//////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct TextureTriangle {
    vao: Option<VertexArrayObject>,
    vbo: Option<BufferObject<TexturedVertex>>,
    ibo: Option<BufferObject<u32>>,
    use_vertex_color: bool,
    render_mode: RenderMode,
    textures: Vec<Texture>,
    vertex_count: u32,
}

impl TextureTriangle {
    fn is_draw_quad(&self) -> bool {
        self.render_mode == RenderMode::Quad || self.render_mode == RenderMode::Quad2
    }

    fn update_data(&mut self) -> SceneResult {
        let vertex_data = if self.is_draw_quad() {
            vertex_data_2d::create_quad()
        } else {
            vertex_data_2d::create_triangle()
        };

        let had_vao_data = self.vao.is_some();
        self.vao = Some(VertexArrayObject::new()?);
        self.vbo = Some(vertex_data.create_vbo());
        self.ibo = Some(vertex_data.create_ibo());
        self.vertex_count = self.get_vertex_count();

        // Setup vertex layout
        VertexLayoutManager::new::<TexturedVertex>().setup_attributes()?;

        self.print_render_mode();

        if !had_vao_data {
            self.print_color_mode();
        }

        Ok(())
    }

    fn get_vertex_count(&self) -> u32 {
        self.ibo.as_ref().map(|ibo| ibo.data_len()).unwrap_or(0) as u32
    }

    fn print_render_mode(&self) {
        println!("{}", self.render_mode);
    }

    fn print_color_mode(&self) {
        let color_mode = if self.use_vertex_color { "ON" } else { "OFF" };
        println!("Vertex coloring: {color_mode}");
    }

    fn bind_textures(&mut self) {
        match self.render_mode {
            RenderMode::Triangle => {
                self.textures[0].bind();
            }
            RenderMode::Quad => {
                self.textures[1].bind();
            }
            RenderMode::Quad2 => {
                self.textures[1].bind_as_unit(0);
                self.textures[2].bind_as_unit(1);
            }
        }
    }

    fn process_keyboard_input(&mut self, keyboard_state: &SdlKeyboardState) -> SceneResult {
        if keyboard_state.is_key_pressed(Keycode::F3) {
            self.use_vertex_color = !self.use_vertex_color;
            self.print_color_mode();
        }
        if keyboard_state.is_key_pressed(Keycode::F4) {
            self.render_mode = self.render_mode.next();
            self.update_data()?;
        }
        Ok(())
    }
}

impl Scene<RenderContext> for TextureTriangle {
    fn activate(&mut self, context: &mut RenderContext) -> SceneResult {
        if self.vao.is_none() {
            self.update_data()?;

            // Preload textures
            let required_textures = [
                textures::M016018BG,
                textures::CRATE8512,
                textures::AWESOMEFACE2,
            ];

            for &texture_name in &required_textures {
                self.textures.push(query_texture(context, texture_name)?);
            }

            // Preload shader
            context
                .shader_manager()
                .get_shader(shaders::SIMPLE_TEXTURED_TRIANGLE)?;
        }
        Ok(())
    }

    fn update(&mut self, context: &mut RenderContext) -> SceneResult {
        self.process_keyboard_input(context.keyboard_state())
    }

    fn draw(&mut self, context: &mut RenderContext) -> SceneResult {
        if let Some(vao) = self.vao.as_mut() {
            vao.bind()?;
            self.bind_textures();

            match context
                .shader_manager()
                .get_shader_mut(shaders::SIMPLE_TEXTURED_TRIANGLE)
            {
                Ok(shader) => {
                    shader.activate();
                    shader.set_uniform("texture1", 0)?;
                    shader.set_uniform("texture2", 1)?;

                    shader.set_uniform("useColor", self.use_vertex_color)?;
                    shader.set_uniform("useTexture2", self.render_mode == RenderMode::Quad2)?;
                }
                _ => {
                    panic!("Shader program is not available!");
                }
            }

            gl_draw::draw_elements(
                PrimitiveType::Triangles,
                self.vertex_count,
                IndicesValueType::Int,
            );
        }

        Ok(())
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
enum RenderMode {
    #[default]
    Triangle,
    Quad,
    Quad2,
}

impl RenderMode {
    fn next(self) -> Self {
        match self {
            RenderMode::Triangle => RenderMode::Quad,
            RenderMode::Quad => RenderMode::Quad2,
            RenderMode::Quad2 => RenderMode::Triangle,
        }
    }
}

impl Display for RenderMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderMode::Triangle => write!(f, "Rendering triangle"),
            RenderMode::Quad => write!(f, "Rendering quad"),
            RenderMode::Quad2 => write!(f, "Rendering quad with awesome face"),
        }
    }
}
