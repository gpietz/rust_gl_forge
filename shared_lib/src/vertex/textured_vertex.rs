// use crate::component::VertexData;
// 
// pub struct TexturedVertex {
//     pub position: [f32; 3],
//     pub normal: [f32; 3],
//     pub tex_coords: [f32; 3],
// }
// 
// impl VertexData for TexturedVertex {
//     fn get_layout() -> Vec<(String, usize)> {
//         vec![
//             ("position".to_string(), 3),
//             ("normal".to_string(), 3),
//             ("tex_coords".to_string(), 3),
//         ]
//     }
// 
//     fn get_data(&self) -> Vec<f32> {
//         [self.position.to_vec(), self.normal.to_vec(), self.tex_coords.to_vec()].concat()
//     }
// }
