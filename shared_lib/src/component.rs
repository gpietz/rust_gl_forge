use std::rc::Rc;

//////////////////////////////////////////////////////////////////////////////
// - Component -
//////////////////////////////////////////////////////////////////////////////

pub enum Component {
    Mesh(Rc<Mesh>),
    Transform(Transform),
    Material(Material),
}

//////////////////////////////////////////////////////////////////////////////
// - Mesh -
//////////////////////////////////////////////////////////////////////////////

pub struct Mesh {
    pub vertices: Vec<MeshVertex>,
    pub vertex_layout: Vec<(String, u32)>,
    pub sub_meshes: Vec<Rc<Mesh>>,
}

//////////////////////////////////////////////////////////////////////////////
// - Mesh -
//////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, Clone, Copy)]
pub struct MeshVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
    pub color: [f32; 4],
}

//////////////////////////////////////////////////////////////////////////////
// - Transform -
//////////////////////////////////////////////////////////////////////////////

pub struct Transform {
    position: [f32; 3],
    rotation: [f32; 3],
    scale: [f32; 3],
}

//////////////////////////////////////////////////////////////////////////////
// - Transform -
//////////////////////////////////////////////////////////////////////////////

pub struct Material {
    pub shader_name: String,
    pub texture_name: String,
}

//////////////////////////////////////////////////////////////////////////////
// - Mesh -
//////////////////////////////////////////////////////////////////////////////

// type AnyMesh = Box<dyn Any>;
//
// pub struct Mesh {
//     vertex_data: AnyMesh,
// }
//
// impl Mesh {
//     pub fn new<T: 'static + VertexData>(vertex_data: T) -> Self {
//         Mesh{
//             vertex_data: Box::new(vertex_data)
//         }
//     }
//
//     pub fn get_layout(&self) -> Vec<(String, usize)> {
//
//     }
// }

//////////////////////////////////////////////////////////////////////////////
// - AnyMesh -
//////////////////////////////////////////////////////////////////////////////

// pub trait VertexData {
//     fn get_layout() -> Vec<(String, usize)>;
//     fn get_data(&self) -> Vec<f32>;
//     fn get_id(&self) -> u32;
// }
//
// pub struct Transform {
//     position: [f32; 3],
//     rotation: [f32; 3],
//     scale: [f32; 3],
// }
//
// // pub struct Mesh {
// //     vertices: Vec<V>,
// //     indices: Vec<u32>,
// // }
