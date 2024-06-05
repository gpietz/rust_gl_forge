use std::ptr;
#[macro_use]
use std::sync::Mutex;
use crate::color::Color;
use crate::gl_blend_guard::BlendGuard;
use crate::gl_draw::{draw_arrays, draw_elements};
use crate::gl_prelude::{
    BufferObject, BufferType, BufferUsage, PrimitiveType, VertexAttributeType, VertexLayoutManager,
};
use crate::gl_shader::{ShaderFactory, ShaderProgram};
use crate::gl_traits::Bindable;
use crate::gl_types::IndicesValueType;
use crate::gl_utils::gl_enum_size;
use crate::gl_vertex_array::VertexArrayObject;
use crate::{Drawable, Position2D, Size2D};
use anyhow::Result;
use cgmath::Matrix4;
use gl::types::{GLfloat, GLsizei};
use lazy_static::lazy_static;

const VERTEX_SHADER_SOURCE: &str = "
    #version 330 core
    layout (location = 0) in vec3 aPos;
    
    uniform mat4 ortho_matrix;
    out vec2 TexCoords;
    
    void main() {
        gl_Position = ortho_matrix * vec4(aPos, 1.0);
        TexCoords = aPos.xy * 0.5 + 0.5;  // Convert from [-1, 1] to [0, 1]
    }";
const FRAGMENT_SHADER_SOURCE: &str = "
    #version 330 core
    out vec4 FragColor;

    in vec2 TexCoords;

    uniform vec4 borderColor;
    uniform vec4 fillColor;
    uniform float opacity;
    uniform float cornerRadius;
    uniform bool isFilled;
    uniform bool hasRoundedCorners;

    float roundedBoxSDF(vec2 p, vec2 b, float r) {
        vec2 q = abs(p) - b + vec2(r);
        return length(max(q, 0.0)) - r;
    }

    void main() {
        vec2 pos = TexCoords - vec2(0.5); // Transform TexCoords to range [-0.5, 0.5]

        // Size of the rectangle
        vec2 halfSize = vec2(0.5) - vec2(cornerRadius);

        // Calculate distance to the edge
        float distance = roundedBoxSDF(pos, halfSize, cornerRadius);

        // Apply smoothstep for antialiasing
        float alpha = 1.0;
        if (hasRoundedCorners) {
            alpha = 1.0 - smoothstep(0.0, 0.01, distance);
        }

        // Determine color
        vec4 color = mix(borderColor, fillColor, float(isFilled));
        FragColor = vec4(color.rgb, color.a * opacity);

        // Debugging output
        //FragColor = vec4(pos, 0.0, 1.0); // Uncomment this line to visualize TexCoords
    }";

pub struct Rectangle {
    position: Position2D,
    size: Size2D<f32>,
    strength: f32,
    color: Color,
    fill_color: Option<Color>,
    opacity: f32,
    corner_radius: Option<f32>,
    projection_matrix: Matrix4<f32>,
}

impl Rectangle {
    pub fn new(
        position: Position2D,
        size: Size2D<f32>,
        color: Color,
        projection_matrix: Matrix4<f32>,
    ) -> Result<Self> {
        let mut rectangle: Rectangle = Rectangle {
            position,
            size,
            strength: 1.0,
            color,
            fill_color: None,
            opacity: 1.0,
            corner_radius: None,
            projection_matrix,
        };

        Ok(rectangle)
    }

    pub fn set_strength(&mut self, strength: f32) {
        self.strength = strength.max(0.0);
    }

    pub fn get_strength(&self) -> f32 {
        self.strength
    }

    pub fn set_fill_color(&mut self, fill_color: Option<Color>) {
        match fill_color {
            Some(color) => self.fill_color = Some(color),
            _ => self.fill_color = None,
        }
    }

    pub fn get_fill_color(&self) -> &Option<Color> {
        &self.fill_color
    }

    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.clamp(0.0, 1.0);
    }

    pub fn get_opacity(&self) -> f32 {
        self.opacity
    }

    pub fn set_corner_radius(&mut self, corner_radius: Option<f32>) {
        self.corner_radius = corner_radius;
    }

    pub fn set_position_xy(&mut self, x: f32, y: f32) {
        self.position.x = x;
        self.position.y = y;
    }
}

impl Drawable for Rectangle {
    fn draw(&mut self) -> Result<()> {
        let mut rectangle_draw = RECTANGLE_DRAW.lock().unwrap();
        rectangle_draw.draw(self)?;
        Ok(())
    }

    fn set_position(&mut self, position2d: Position2D) -> Result<()> {
        self.position = position2d;
        Ok(())
    }

    fn get_position(&self) -> &Position2D {
        &self.position
    }

    fn set_size(&mut self, width: f32, height: f32) -> Result<()> {
        self.size = Size2D::new(width, height);
        Ok(())
    }

    fn get_size(&self) -> &Size2D<f32> {
        &self.size
    }

    fn set_color(&mut self, color: Color) -> Result<()> {
        self.color = color;
        Ok(())
    }

    fn get_color(&self) -> &Color {
        &self.color
    }

    fn set_projection_matrix(&mut self, projection_matrix: &Matrix4<f32>) -> Result<()> {
        self.projection_matrix = *projection_matrix;
        Ok(())
    }

    fn get_projection_matrix(&self) -> &Matrix4<f32> {
        &self.projection_matrix
    }
}

struct RectangleDraw {
    vao: VertexArrayObject,
    vbo: BufferObject<f32>,
    ebo: BufferObject<u32>,
    vertices: Vec<f32>,
    shader: ShaderProgram,
    last_position: Option<Position2D>,
    last_size: Option<Size2D<f32>>,
}

impl RectangleDraw {
    pub fn new() -> Self {
        let vao = VertexArrayObject::new().expect("Failed to create vertex array object");
        let vbo = BufferObject::empty(BufferType::ArrayBuffer, BufferUsage::DynamicDraw);
        let ebo = BufferObject::new(
            BufferType::ElementArrayBuffer,
            BufferUsage::StaticDraw,
            vec![0, 1, 2, 2, 3, 0],
        );

        // Set vertex Attributes
        VertexLayoutManager::from_attribute_types(vec![VertexAttributeType::Position])
            .setup_attributes()
            .expect("Failed to setup vertex attribute layout");

        // Create shader program
        let vertex_shader = VERTEX_SHADER_SOURCE;
        let fragment_shader = FRAGMENT_SHADER_SOURCE;
        let shader_program = ShaderFactory::from_source(vertex_shader, fragment_shader)
            .expect("Failed to create shader program");

        Self {
            vao,
            vbo,
            ebo,
            vertices: Vec::new(),
            shader: shader_program,
            last_position: None,
            last_size: None,
        }
    }

    pub fn draw(&mut self, rect: &Rectangle) -> Result<()> {
        self.vao.bind()?;
        self.vbo.bind()?;
        self.ebo.bind()?;
        self.update_shader_uniforms(rect);
        self.update_vertices(rect);

        assert_eq!(self.ebo.data_len(), 6);
        assert_eq!(self.vbo.data_len(), 12);

        let _blend_guard = BlendGuard::default();

        if rect.fill_color.is_some() {
            draw_elements(
                PrimitiveType::Triangles,
                self.ebo.data_len() as u32,
                IndicesValueType::Int,
            );
        } else {
            unsafe {
                gl::LineWidth(rect.strength);
            }
            draw_arrays(PrimitiveType::LineLoop, 0, 4);
        }

        Ok(())
    }

    fn update_shader_uniforms(&mut self, rect: &Rectangle) -> Result<()> {
        self.shader.activate();

        // Projection matrix
        self.shader.set_uniform_matrix("ortho_matrix", false, &rect.projection_matrix)?;
        // Color
        let border_color: [f32; 4] = rect.color.into();
        self.shader.set_uniform("borderColor", border_color)?;
        // Fill color
        let fill_color: [f32; 4] = rect.fill_color.unwrap_or(Color::TRANSPARENT).into();
        self.shader.set_uniform("fillColor", fill_color)?;
        // Opacity
        self.shader.set_uniform("opacity", rect.opacity.clamp(0.0, 1.0));
        // Corner radius
        self.shader.set_uniform("cornerRadius", rect.corner_radius.unwrap_or(0.0))?;
        // Flags (fill rectangle/round corners)
        self.shader.set_uniform("isFilled", rect.fill_color.is_some())?;
        self.shader.set_uniform("hasRoundedCorners", rect.corner_radius.is_some())?;

        Ok(())
    }

    fn update_vertices(&mut self, rect: &Rectangle) -> Result<()> {
        if !self.position_changed(rect.position) && !self.size_changed(rect.size) {
            return Ok(());
        }

        let Position2D { x, y } = rect.position;
        let Size2D { width, height } = rect.size;

        #[rustfmt::skip]
        let vertices: [f32; 12] = [
            x,y,0.0,                    // Bottom-left
            x + width,y,0.0,            // Bottom-right
            x + width,y + height, 0.0,  // Top-right
            x, y + height, 0.0,         // Top-left
        ];
        let vec = vertices.to_vec();
        self.vbo.update_data(vec, None)?;

        self.last_position = Some(rect.position);
        self.last_size = Some(rect.size);

        Ok(())
    }

    fn position_changed(&self, position: Position2D) -> bool {
        self.last_position.map_or(true, |last_pos| last_pos != position)
    }

    fn size_changed(&self, size: Size2D<f32>) -> bool {
        self.last_size.map_or(true, |last_size| last_size != size)
    }
}

lazy_static! {
    static ref RECTANGLE_DRAW: Mutex<RectangleDraw> = { Mutex::new(RectangleDraw::new()) };
}
