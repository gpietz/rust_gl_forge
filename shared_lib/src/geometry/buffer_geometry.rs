use crate::gl_types::BufferType;
use crate::opengl::buffer_object::BufferObject;
use crate::opengl::vertex_array_object::VertexArrayObject;
use crate::opengl::vertex_layout::VertexLayout;

pub struct BufferGeometry<V: VertexLayout> {
    vertices: Vec<V>,
    indices: Option<Vec<u32>>,
    vao: Option<VertexArrayObject>,
    vbo: Option<BufferObject<V>>,
    ebo: Option<BufferObject<u32>>,
}

impl<V: VertexLayout> BufferGeometry<V> {
    pub fn new(vertices: Vec<V>, indices: Option<Vec<u32>>) -> Self {
        BufferGeometry {
            vertices,
            indices,
            vao: None,
            vbo: None,
            ebo: None,
        }
    }

    //TODO Add error handling to this function
    pub fn init(&mut self) {
        let vao = VertexArrayObject::default();
        vao.bind();

        //let vbo = BufferObject::new(BufferType::ArrayBuffer, ) 
    }

    pub fn render(&self) {

    }
}
