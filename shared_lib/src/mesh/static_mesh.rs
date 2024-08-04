use crate::mesh::{Mesh, MeshError, StaticMeshTrait};
use anyhow::Result;
use cgmath::{Matrix4, Quaternion, SquareMatrix, Vector3};

#[derive(Debug, Clone)]
pub struct StaticMesh {
    position: Vector3<f32>,
    rotation: Quaternion<f32>,
    scale: Vector3<f32>,
    transformation_matrix: Matrix4<f32>,
    vertices: Vec<Vector3<f32>>,
    indices: Vec<u32>,
}

impl Default for StaticMesh {
    fn default() -> Self {
        StaticMesh {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::new(0.0, 0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
            transformation_matrix: Matrix4::from_scale(1.0),
            vertices: Vec::new(),
            indices: Vec::new(),        
        }
    }
}

impl StaticMesh {
    fn update_transformation_matrix(&mut self) {
        self.transformation_matrix = Matrix4::from_translation(self.position)
            * Matrix4::from(self.rotation)
            * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
    }
}

impl Mesh for StaticMesh {
    fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position;
        self.update_transformation_matrix();
    }

    fn get_position(&self) -> &Vector3<f32> {
        &self.position
    }

    fn set_rotation(&mut self, rotation: Quaternion<f32>) {
        self.rotation = rotation;
        self.update_transformation_matrix();
    }

    fn get_rotation(&self) -> &Quaternion<f32> {
        &self.rotation
    }

    fn set_scale(&mut self, scale: Vector3<f32>) {
        self.scale = scale;
        self.update_transformation_matrix();
    }

    fn get_scale(&self) -> &Vector3<f32> {
        &self.scale
    }

    fn render(&self) -> anyhow::Result<()> {
        //TODO Implement the rendering
        Ok(())
    }
}

impl StaticMeshTrait for StaticMesh {
    fn set_vertices(&mut self, vertices: Vec<Vector3<f32>>) -> Result<(), MeshError> {
        if vertices.len() % 3 != 0 {
            return Err(MeshError::InvalidVertexCount);
        }
        self.vertices = vertices;
        // TODO Update VBO!
        Ok(())
    }

    fn get_vertices(&self) -> &Vec<Vector3<f32>> {
        &self.vertices
    }

    fn set_indices(&mut self, indices: Vec<u32>) {
        
    }

    fn get_indices(&self) -> &Vec<u32> {
        todo!()
    }

    fn set_material(&mut self, material: String) {
        todo!()
    }

    fn get_material(&self) -> &String {
        todo!()
    }

    fn calculate_bounding_box(&self) -> (Vector3<f32>, Vector3<f32>) {
        todo!()
    }

    fn apply_transformation(&mut self, transformation: Matrix4<f32>) {
        todo!()
    }

    fn detect_collision(&self, other: &dyn StaticMeshTrait) -> bool {
        todo!()
    }
}
