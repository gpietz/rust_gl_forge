pub mod static_mesh;

use anyhow::Result;
use cgmath::{Matrix4, Quaternion, Vector3};
use std::any::Any;
use thiserror::Error;

/// A trait representing basic functionalities of a mesh,
/// including position, rotation, scaling, and rendering.
pub trait Mesh {
    /// Sets the position of the mesh.
    ///
    /// # Parameters
    /// - `position`: A `Vector3<f32>` representing the new position of the mesh.
    fn set_position(&mut self, position: Vector3<f32>);

    /// Gets the position of the mesh.
    ///
    /// # Returns
    /// A reference to a `Vector3<f32>` representing the position of the mesh.
    fn get_position(&self) -> &Vector3<f32>;

    /// Sets the rotation of the mesh.
    ///
    /// # Parameters
    /// - `rotation`: A `Quaternion<f32>` representing the new rotation of the mesh.
    fn set_rotation(&mut self, rotation: Quaternion<f32>);

    /// Gets the rotation of the mesh.
    ///
    /// # Returns
    /// A reference to a `Quaternion<f32>` representing the rotation of the mesh.
    fn get_rotation(&self) -> &Quaternion<f32>;

    /// Sets the scale of the mesh.
    ///
    /// # Parameters
    /// - `scale`: A `Vector3<f32>` representing the new scale of the mesh.
    fn set_scale(&mut self, scale: Vector3<f32>);

    /// Gets the scale of the mesh.
    ///
    /// # Returns
    /// A reference to a `Vector3<f32>` representing the scale of the mesh.
    fn get_scale(&self) -> &Vector3<f32>;

    /// Renders the mesh.
    ///
    /// # Returns
    /// A `Result` indicating success or failure of the rendering operation.
    fn render(&self) -> Result<()>;
}

/// A trait representing a static mesh, similar to the StaticMesh in Unreal Engine.
/// Provides methods for managing vertices, indices, materials, and transformations.
pub trait StaticMeshTrait: Mesh {
    /// Sets the vertices of the mesh.
    ///
    /// # Parameters
    /// - `vertices`: A vector of `Vector3<f32>` representing the mesh vertices.
    fn set_vertices(&mut self, vertices: Vec<Vector3<f32>>) -> Result<(), MeshError>;

    /// Gets the vertices of the mesh.
    ///
    /// # Returns
    /// A reference to a vector of `Vector3<f32>` representing the mesh vertices.
    fn get_vertices(&self) -> &Vec<Vector3<f32>>;

    /// Sets the indices of the mesh.
    ///
    /// # Parameters
    /// - `indices`: A vector of `u32` representing the mesh indices.
    fn set_indices(&mut self, indices: Vec<u32>);

    /// Gets the indices of the mesh.
    ///
    /// # Returns
    /// A reference to a vector of `u32` representing the mesh indices.
    fn get_indices(&self) -> &Vec<u32>;

    /// Sets the material of the mesh.
    ///
    /// # Parameters
    /// - `material`: A string representing the material name or path.
    fn set_material(&mut self, material: String);

    /// Gets the material of the mesh.
    ///
    /// # Returns
    /// A reference to a string representing the material name or path.
    fn get_material(&self) -> &String;

    /// Calculates the bounding box of the mesh.
    ///
    /// # Returns
    /// A tuple containing two `Vector3<f32>` representing the minimum and maximum points of the bounding box.
    fn calculate_bounding_box(&self) -> (Vector3<f32>, Vector3<f32>);

    /// Applies a transformation to the mesh.
    ///
    /// # Parameters
    /// - `transformation`: A `Matrix4<f32>` representing the transformation matrix.
    fn apply_transformation(&mut self, transformation: Matrix4<f32>);

    /// Performs collision detection with another mesh.
    ///
    /// # Parameters
    /// - `other`: A reference to another object implementing the `StaticMesh` trait.
    ///
    /// # Returns
    /// A boolean indicating whether a collision was detected.
    fn detect_collision(&self, other: &dyn StaticMeshTrait) -> bool;
}

#[derive(Error, Debug)]
pub enum MeshError {
    #[error("Invalid number of mesh vertices; must be divisible by three.")]
    InvalidVertexCount,
}
