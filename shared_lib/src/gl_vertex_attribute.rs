// Ensure you have the `gl` crate added to your `Cargo.toml`
extern crate gl;

use std::collections::HashMap;
use std::fmt::Debug;

use anyhow::Context;
use anyhow::Result;
use gl::types::{GLboolean, GLint, GLsizei, GLuint, GLvoid};
use thiserror::Error;

use crate::gl_prelude::Vertex;
use crate::gl_shader::ShaderProgram;
use crate::gl_types::VertexDataType;
use crate::gl_utils::check_gl_error;

//////////////////////////////////////////////////////////////////////////////
// - VertexAttribute -
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct VertexAttribute {
    /// Optional name of the attribute, useful when querying by name in shader programs.
    pub name: Option<String>,
    pub components: u8,
    pub data_type: VertexDataType,
    pub normalized: Option<bool>,
    pub stride: Option<u32>,
    pub offset: Option<u32>,
}

impl VertexAttribute {
    pub fn new(components: u8, data_type: impl Into<VertexDataType>) -> Self {
        Self {
            components,
            data_type: data_type.into(),
            ..Self::default()
        }
    }

    /// Sets the name field of the instance, consuming and returning self for method chaining.
    pub fn name(mut self, name: impl Into<Option<String>>) -> Self {
        self.name = name.into();
        self
    }

    /// Sets the number of components for the vertex attribute and returns the modified object.
    /// # Arguments
    /// * `components` - A `u8` specifying the number of components (1 to 4 are typical values).
    pub fn components(mut self, components: u8) -> Self {
        self.components = components;
        self
    }

    pub fn data_type(mut self, data_type: VertexDataType) -> Self {
        self.data_type = data_type;
        self
    }

    pub fn normalized(mut self, normalized: impl Into<Option<bool>>) -> Self {
        self.normalized = normalized.into();
        self
    }

    pub fn stride(mut self, stride: impl Into<Option<u32>>) -> Self {
        self.stride = stride.into();
        self
    }

    pub fn offset(mut self, offset: impl Into<Option<u32>>) -> Self {
        self.offset = offset.into();
        self
    }

    /// Calculates the byte size of the attribute based on its specifications or its type.
    pub fn calculate_size(&self) -> usize {
        self.data_type.size() * self.components as usize
    }
}

impl Default for VertexAttribute {
    fn default() -> Self {
        todo!()
    }
}

//////////////////////////////////////////////////////////////////////////////
// - VertexLayoutManager -
//////////////////////////////////////////////////////////////////////////////

pub struct VertexLayoutManager {
    attributes: Vec<VertexAttribute>,
    // Maps attribute index to stride and offset.
    layout_info: HashMap<usize, [u32; 2]>,
}

impl VertexLayoutManager {
    /// Creates a new instance with no attributes and an empty layout.
    pub fn empty() -> Self {
        Self {
            attributes: vec![],
            layout_info: HashMap::new(),
        }
    }

    /// Constructs a new instance using attributes from the Vertex trait and initializes layout info.
    pub fn new<T: Vertex>() -> Self {
        let mut manager = Self {
            attributes: T::attributes(),
            layout_info: HashMap::new(),
        };
        manager.calculate_layout_info();
        manager
    }

    /// Creates a new instance and sets up vertex attributes for a given shader, handling errors.
    pub fn new_and_setup<T: Vertex>(shader: &ShaderProgram) -> Result<Self> {
        let mut manager = Self::new::<T>();
        let program_id = shader.program_id();
        manager.setup_attributes_for_shader(program_id).with_context(|| {
            format!(
                "Failed to set up vertex attributes for shader program ID: {}",
                shader.program_id()
            )
        })?;
        Ok(manager)
    }

    /// Adds a vertex attribute and updates layout info, returning a mutable reference to self.
    pub fn add_attribute(&mut self, attribute: VertexAttribute) -> &mut Self {
        self.attributes.push(attribute);
        self.calculate_layout_info();
        self
    }

    /// Adds multiple vertex attributes and updates layout info, returning a mutable
    /// reference to self.
    pub fn add_attributes(&mut self, attributes: &[VertexAttribute]) -> &mut Self {
        for attribute in attributes {
            self.attributes.push(attribute.clone());
        }
        self.calculate_layout_info();
        self
    }

    pub fn add_attributes_iter<I>(&mut self, attributes: I) -> &mut Self
    where
        I: IntoIterator<Item = VertexAttribute>,
    {
        for attribute in attributes {
            self.attributes.push(attribute);
        }
        self.calculate_layout_info();
        self
    }

    /// Adds vertex attributes from an iterable and updates layout info, returning a mutable
    /// reference to self.
    fn calculate_layout_info(&mut self) {
        let mut current_offset: usize = 0;
        let mut total_size = 0;

        // Calculate the total size for the stride in a tightly packed layout
        for attribute in self.attributes.iter() {
            let size = attribute.calculate_size();
            total_size += size;
        }

        // Set stride (now correctly representing total_size) and offset for each attribute
        for (index, attribute) in self.attributes.iter().enumerate() {
            let size = attribute.calculate_size();
            self.layout_info.insert(index, [total_size as u32, current_offset as u32]);
            current_offset += size;
        }
    }

    /// Removes an attribute by name and updates layout info, returning a mutable reference to self.
    pub fn remove_attribute_by_name(&mut self, name: &str) -> &mut Self {
        self.attributes.retain(|attr| attr.name.as_deref() != Some(name));
        self.calculate_layout_info();
        self
    }

    /// Removes an attribute by index and updates layout info if the index is valid, returning
    /// a mutable reference to self.
    pub fn remove_attribute_by_index(&mut self, index: usize) -> &mut Self {
        if index < self.attributes.len() {
            self.attributes.remove(index);
            self.calculate_layout_info();
        }
        self
    }

    /// Clears all attributes without updating layout info, returning a mutable reference to self.
    pub fn clear_attributes(&mut self) -> &mut Self {
        self.attributes.clear();
        self
    }

    /// Returns a reference to a vertex attribute by name if it exists.
    pub fn get_attribute_by_name(&self, name: &str) -> Option<&VertexAttribute> {
        self.attributes.iter().find(|attr| attr.name.as_deref() == Some(name))
    }

    /// Returns a mutable reference to a vertex attribute by name if it exists.
    pub fn get_attribute_by_name_mut(&mut self, name: &str) -> Option<&mut VertexAttribute> {
        self.attributes.iter_mut().find(|attr| attr.name.as_deref() == Some(name))
    }

    /// Returns a reference to a vertex attribute by index if it exists.
    pub fn get_attribute_by_index(&self, index: usize) -> Option<&VertexAttribute> {
        self.attributes.get(index)
    }

    /// Returns a mutable reference to a vertex attribute by index if it exists.
    pub fn get_attribute_by_index_mut(&mut self, index: usize) -> Option<&mut VertexAttribute> {
        self.attributes.get_mut(index)
    }

    /// Replaces or adds a vertex attribute by name. If an attribute with the given name exists,
    /// it's updated; otherwise, the attribute is added.
    pub fn add_or_replace(&mut self, attribute: VertexAttribute) -> &mut Self {
        match self.attributes.iter_mut().find(|attr| attr.name == attribute.name) {
            Some(existing_attribute) => *existing_attribute = attribute,
            None => self.attributes.push(attribute),
        }
        self.calculate_layout_info();
        self
    }

    /// Returns the number of attributes.
    pub fn attribute_count(&self) -> usize {
        self.attributes.len()
    }

    /// Finalizes the layout of vertex attributes, ensuring all constraints are met.
    /// Returns an error if any attribute has an invalid number of components.
    ///
    /// # Errors
    /// Returns a `VertexLayoutError` if any issues are found in the vertex attribute layout.
    /// This may include but is not limited to invalid numbers of components per attribute,
    /// misaligned data structures, or incompatible attribute configurations.
    ///
    /// # Examples
    /// ```no-run
    /// let mut layout_manager = VertexLayoutManager::new();
    /// layout_manager.add_attribute(VertexAttribute { components: 3, ... });
    /// assert!(layout_manager.finalize_layout().is_ok());
    /// ```
    pub fn finalize_layout(&mut self) -> Result<(), VertexLayoutError> {
        for attribute in self.attributes.iter() {
            // Check number of components
            if attribute.components < 1 || attribute.components > 4 {
                return Err(VertexLayoutError::InvalidNumberOfComponents);
            }
        }
        Ok(())
    }

    /// Configures shader program vertex attributes based on current VertexAttribute settings.
    /// Validates the shader program ID and ensures attribute specifications meet OpenGL standards.
    ///
    /// # Parameters
    /// - `shader_program_id`: The identifier for the shader program to configure.
    ///
    /// # Returns
    /// A result indicating success or containing a `VertexLayoutError` detailing any issues such as:
    /// - Invalid shader program IDs.
    /// - Issues with attribute names or indices.
    /// - OpenGL specific errors during attribute setup.
    ///
    /// # Errors
    /// - `InvalidShaderId` if the shader program ID is zero.
    /// - `InvalidNumberOfComponents`, `InvalidAttributeName`, or `InvalidAttributeLocation` for
    ///   configuration errors.
    /// - `OpenGL` for errors returned from OpenGL commands.
    pub fn setup_attributes_for_shader(
        &mut self,
        shader_program_id: u32,
    ) -> Result<(), VertexLayoutError> {
        if shader_program_id == 0 {
            return Err(VertexLayoutError::InvalidShaderId);
        }

        self.finalize_layout()?;

        unsafe {
            // Iterate over each attribute
            for (index, attribute) in self.attributes.iter().enumerate() {
                println!(
                    "Processing attribute {} for shader {}",
                    index, shader_program_id
                );

                // Determine the attribute properties from VertexAttributeType
                let (mut components, data_type, data_size, normalized) = (
                    attribute.components,
                    attribute.data_type.to_gl_enum(),
                    attribute.data_type.size(),
                    attribute.normalized.unwrap_or_default(),
                );

                // Fetch stride and offset from layout_specs or use the calculated layout info
                let (stride, offset) = match (attribute.stride, attribute.offset) {
                    (Some(stride), Some(offset)) => (stride, offset),
                    _ => {
                        // Fallback to calculated layout info if custom specs are not provided
                        let layout_info = self
                            .layout_info
                            .get(&index)
                            .expect("Layout info should be calculated for all attributes.");
                        (layout_info[0], layout_info[1])
                    }
                };

                // Retrieve the attribute location by name if available, or use the index from this iteration
                let attr_location = if let Some(name) = &attribute.name {
                    let c_str = std::ffi::CString::new(name.as_str()).unwrap();
                    gl::GetAttribLocation(shader_program_id, c_str.as_ptr())
                } else {
                    index as i32
                };

                // Check if the attribute location is valid
                if attr_location < 0 {
                    return if let Some(attribute_name) = &attribute.name {
                        Err(VertexLayoutError::InvalidAttributeName(
                            attribute_name.to_string(),
                        ))
                    } else {
                        Err(VertexLayoutError::InvalidAttributeLocation(index))
                    };
                }

                // Setup the vertex attribute pointer
                gl::EnableVertexAttribArray(attr_location as u32);
                gl::VertexAttribPointer(
                    attr_location as GLuint,
                    components as GLint,
                    data_type,
                    normalized as GLboolean,
                    stride as GLsizei,
                    offset as *const GLvoid,
                );

                // Check for GL errors after setting up the vertex attribute
                check_and_map_gl_error()?;
            }
        }
        Ok(())
    }

    pub fn save_layout(&self) -> Result<()> {
        Ok(())
    }
}

//////////////////////////////////////////////////////////////////////////////
// - VertexLayoutError -
//////////////////////////////////////////////////////////////////////////////

#[derive(Error, Debug)]
pub enum VertexLayoutError {
    #[error("Invalid shader program id")]
    InvalidShaderId,
    #[error("Invalid number of components: each attribute must have between 1 and 4 components.")]
    InvalidNumberOfComponents,
    #[error("Invalid attribute name: {0}")]
    InvalidAttributeName(String),
    #[error("Invalid index for attribute location: {0}")]
    InvalidAttributeLocation(usize),
    #[error("Datatype not present for attribute in vertex layout")]
    DataTypeNotPresent,
    #[error("OpenGL error: {0}")]
    OpenGL(String),
}

//////////////////////////////////////////////////////////////////////////////
// - Misc. Functions -
//////////////////////////////////////////////////////////////////////////////

/// Checks for OpenGL errors and maps any encountered errors to `VertexLayoutError`.
///
/// # Returns
/// A `Result<(), VertexLayoutError>`, returning `Ok(())` if no OpenGL errors were detected,
/// or `Err(VertexLayoutError::OpenGL)` with an error description if errors are present.
fn check_and_map_gl_error() -> Result<(), VertexLayoutError> {
    check_gl_error().map_err(|e| VertexLayoutError::OpenGL(e.to_string()))
}
