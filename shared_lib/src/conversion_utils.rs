use cgmath::Vector3;

pub fn convert_to_vector3_vec(vertices: &[f32]) -> Vec<Vector3<f32>> {
    vertices
        .chunks(3)
        .map(|chunk| Vector3::new(chunk[0], chunk[1], chunk[2]))
        .collect()
}
