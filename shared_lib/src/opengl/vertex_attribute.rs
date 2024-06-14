use crate::gl_prelude::VertexDataType;

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
    pub fn new(components: u8, data_type: VertexDataType) -> Self {
        Self {
            components,
            data_type,
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
        VertexAttribute {
            name: None,
            components: 3,
            data_type: VertexDataType::Float,
            normalized: None,
            stride: None,
            offset: None,
        }
    }
}
