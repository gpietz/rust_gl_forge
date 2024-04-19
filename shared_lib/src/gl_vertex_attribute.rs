use crate::gl_prelude::Vertex;
use crate::gl_shader::ShaderProgram;
use crate::gl_types::VertexDataType;
use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use gl::types::{GLboolean, GLsizei, GLvoid};
use std::collections::HashMap;
use std::fmt::{Debug};

//////////////////////////////////////////////////////////////////////////////
// - VertexAttribute -
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub struct VertexAttribute {
    /// Optional name of the attribute, useful when querying by name in shader programs.
    pub name: Option<String>,
    pub components: u32,
    pub data_type: VertexDataType,
    pub normalized: Option<bool>,
    pub stride: Option<u32>,
    pub offset: Option<u32>,
}

impl VertexAttribute {
    pub fn new(components: u32, data_type: impl Into<VertexDataType>) -> Self {
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

    pub fn components(mut self, components: u32) -> Self {
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
    pub fn empty() -> Self {
        Self {
            attributes: vec![],
            layout_info: HashMap::new(),
        }
    }

    pub fn new<T: Vertex>() -> Self {
        let mut manager = Self {
            attributes: T::attributes(),
            layout_info: HashMap::new(),
        };
        manager.calculate_layout_info();
        manager
    }

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

    pub fn add_attribute(&mut self, attribute: VertexAttribute) -> &mut Self {
        self.attributes.push(attribute);
        self.calculate_layout_info();
        self
    }

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

    pub fn remove_attribute_by_name(&mut self, name: &str) -> &mut Self {
        self.attributes.retain(|attr| attr.name.as_deref() != Some(name));
        self.calculate_layout_info();
        self
    }

    pub fn remove_attribute_by_index(&mut self, index: usize) -> &mut Self {
        if index < self.attributes.len() {
            self.attributes.remove(index);
            self.calculate_layout_info();
        }
        self
    }

    pub fn clear_attributes(&mut self) -> &mut Self {
        self.attributes.clear();
        self
    }

    pub fn get_attribute_by_name(&self, name: &str) -> Option<&VertexAttribute> {
        self.attributes.iter().find(|attr| attr.name.as_deref() == Some(name))
    }

    pub fn get_attribute_by_name_mut(&mut self, name: &str) -> Option<&mut VertexAttribute> {
        self.attributes.iter_mut().find(|attr| attr.name.as_deref() == Some(name))
    }

    pub fn get_attribute_by_index(&self, index: usize) -> Option<&VertexAttribute> {
        self.attributes.get(index)
    }

    pub fn get_attribute_by_index_mut(&mut self, index: usize) -> Option<&mut VertexAttribute> {
        self.attributes.get_mut(index)
    }

    /// Replaces or adds a vertex attribute by name. If an attribute with the given name exists,
    /// it's updated; otherwise, the attribute is added.
    pub fn set_attribute(&mut self, attribute: VertexAttribute) -> &mut Self {
        match self.attributes.iter_mut().find(|attr| attr.name == attribute.name) {
            Some(existing_attribute) => *existing_attribute = attribute,
            None => self.attributes.push(attribute),
        }
        self.calculate_layout_info();
        self
    }

    pub fn attribute_count(&self) -> usize {
        self.attributes.len()
    }

    pub fn finalize_layout(&mut self) -> &mut Self {
        // TODO: Perform validation and optimization here
        self
    }

    pub fn setup_attributes_for_shader(&mut self, shader_program_id: u32) -> Result<()> {
        if shader_program_id == 0 {
            return Err(anyhow!("Invalid shader program id"));
        }

        unsafe {
            // Iterate over each attribute
            for (index, attribute) in self.attributes.iter().enumerate() {
                println!(
                    "Processing attribute {} for shader {}",
                    index, shader_program_id
                );
                // Determine the default attribute properties from VertexAttributeType
                let (mut components, default_data_type, default_normalized) =
                    attribute.attribute_type.to_gl_data();

                // Override with VertexAttributeSpecs if available
                if let Some(specs) = &attribute.attribute_specs {
                    if specs.components > 0 {
                        components = specs.components as i32;
                    }
                }

                // Determine the data type and normalization
                let data_type = attribute
                    .attribute_specs
                    .as_ref()
                    .map_or(default_data_type, |specs| specs.data_type.to_gl_enum());
                let normalized = attribute
                    .attribute_specs
                    .as_ref()
                    .map_or(default_normalized, |specs| specs.normalized as GLboolean);

                // Fetch stride and offset from layout_specs or use the calculated layout info
                let (stride, offset) = if let Some(layout_specs) = &attribute.layout_specs {
                    (layout_specs.stride, layout_specs.offset)
                } else {
                    // Fallback to calculated layout info if custom specs are not provided
                    let layout_info = self
                        .layout_info
                        .get(&index)
                        .expect("Layout info should be calculated for all attributes.");
                    (layout_info[0], layout_info[1])
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
                    return Err(anyhow!("Invalid attribute name: {:?}", attribute.name));
                }

                // Setup the vertex attribute pointer
                gl::EnableVertexAttribArray(attr_location as u32);
                gl::VertexAttribPointer(
                    attr_location as u32,
                    components,
                    data_type,
                    normalized,
                    stride as GLsizei,
                    offset as *const GLvoid,
                );
            }
        }
        Ok(())
    }

    pub fn save_layout(&self) -> Result<()> {
        Ok(())
    }
}

// Ensure you have the `gl` crate added to your `Cargo.toml`
extern crate gl;

pub fn setup_vertex_attributes(components: &[u8]) {
    // Assuming each component is a float, calculate the total stride
    let float_size = std::mem::size_of::<f32>() as GLsizei;
    let mut stride = 0;
    for &comp in components {
        stride += comp as GLsizei * float_size;
    }

    // Offset tracks the byte offset of each attribute within the vertex
    let mut offset = 0;
    for (index, &comp) in components.iter().enumerate() {
        unsafe {
            // Enable the vertex attribute array
            gl::EnableVertexAttribArray(index as u32);
            // Set up the vertex attribute pointer
            gl::VertexAttribPointer(
                index as u32,                       // Attribute index
                comp as i32,                        // Number of components per attribute
                gl::FLOAT,                          // Data type of each component
                gl::FALSE,                          // Normalized
                stride,                             // Stride
                offset as *const gl::types::GLvoid, // Offset
            );
        }

        // Increment offset by the size of this attribute
        offset += comp as GLsizei * float_size;
    }
}
