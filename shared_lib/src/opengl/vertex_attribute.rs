use crate::gl_prelude::{VertexAttributeType, VertexDataType};

#[derive(Clone, Debug, Default)]
pub struct VertexAttribute {
    /// Optional name of the attribute, useful when querying by name in shader programs.
    pub name: Option<String>,
    pub components: u8,
    pub data_type: VertexDataType,
    pub normalized: bool,
    pub stride: i32,
    pub offset: u32,
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

    pub fn normalized(mut self, normalized: bool) -> Self {
        self.normalized = normalized;
        self
    }

    pub fn stride(mut self, stride: i32) -> Self {
        self.stride = stride;
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = offset;
        self
    }

    /// Calculates the byte size of the attribute based on its specifications or its type.
    pub fn calculate_size(&self) -> usize {
        self.data_type.size() * self.components as usize
    }
}

impl From<VertexAttributeType> for VertexAttribute {
    fn from(value: VertexAttributeType) -> Self {
        match value {
            VertexAttributeType::Position => VertexAttribute {
                components: 3,
                data_type: VertexDataType::Float,
                normalized: false,
                ..Default::default()
            },
            VertexAttributeType::Position2D => VertexAttribute {
                components: 2,
                data_type: VertexDataType::Float,
                normalized: false,
                ..Default::default()
            },
            VertexAttributeType::Color => VertexAttribute {
                components: 4,
                data_type: VertexDataType::Float,
                normalized: false,
                ..Default::default()
            },
            VertexAttributeType::TexCoords => VertexAttribute {
                components: 2,
                data_type: VertexDataType::Float,
                normalized: false,
                ..Default::default()
            },
            VertexAttributeType::Normal => VertexAttribute {
                components: 3,
                data_type: VertexDataType::Float,
                normalized: false,
                ..Default::default()
            },
        }
    }
}
