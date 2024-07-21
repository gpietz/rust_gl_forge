use crate::opengl::vertex_attribute::VertexAttribute;
use thiserror::Error;

pub trait VertexLayout {
    fn attributes() -> Vec<VertexAttribute>;
    fn layout_size() -> usize {
        Self::attributes().iter().map(|attr| attr.calculate_size()).sum()
    }
}

#[derive(Error, Debug)]
pub enum VertexLayoutError {
    #[error("Invalid VAO id")]
    InvalidVAOId,
    #[error("Invalid shader program id")]
    InvalidShaderId,
    #[error("Invalid number of components: each attribute must have between 1 and 4 components.")]
    InvalidNumberOfComponents,
    #[error("Invalid attribute name: {0}")]
    InvalidAttributeName(String),
    #[error("Invalid index for attribute location: {0}")]
    InvalidAttributeLocation(usize),
    #[error("Invalid layout name: {0}")]
    InvalidLayoutName(String),
    #[error("Datatype not present for attribute in vertex layout")]
    DataTypeNotPresent,
    #[error("OpenGL error: {0}")]
    OpenGL(String),
}
