use crate::gl_prelude::Vertex;
use crate::gl_shader::ShaderProgram;
use crate::gl_types::VertexAttributeType;
use crate::gl_types::VertexDataType;
use crate::gl_utils::gl_enum_size;
use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use gl::types::{GLboolean, GLsizei, GLvoid};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

//////////////////////////////////////////////////////////////////////////////
// - VertexAttribute -
//////////////////////////////////////////////////////////////////////////////

/// Represents a vertex attribute in a shader program.
///
/// This struct is used to define the properties of a single vertex attribute,
/// such as its name, size, type, and optional specifications like the number of components,
/// data type, and normalization. It also includes stride and offset, which are critical
/// for defining how vertex data is laid out in memory.
///
/// # Fields
///
/// - `name`: An optional `String` that holds the name of the vertex attribute.
/// This name is used to bind the attribute within a shader program. It can be `None`
/// if attributes are bound using layout locations instead of names.
///
/// - `size`: An `i32` indicating the size of the attribute. This size is typically
/// the total number of bytes that the attribute occupies in a vertex.
///
/// - `attribute_type`: A value of the enum `VertexAttributeType` which specifies
/// the high-level semantic type of the attribute (e.g., Position, Color, etc.).
///
/// - `attribute_specs`: An optional `VertexAttributeSpecs` struct that provides
/// detailed specifications for this attribute, including the number of components,
/// the data type of each component, and whether the data should be normalized. This
/// allows for more fine-grained control over how attribute data is interpreted.
///
/// - `stride`: A `usize` that specifies the byte offset between consecutive vertices
/// in a vertex buffer. This is used to stride over attributes in a vertex buffer
/// when iterating through vertex data.
///
/// - `offset`: A `usize` that specifies the byte offset of this attribute from the
/// start of a vertex in a vertex buffer. This is used to locate the beginning of
/// the attribute data in a vertex.
///
/// # Examples
///
/// Creating a basic vertex attribute without custom specifications:
///
/// ```
/// use shared_lib::gl_vertex_attribute::*;
/// use shared_lib::gl_types::*;
///
/// let position_attribute = VertexAttribute::new(VertexAttributeType::Position);
/// ```
///
/// Creating a vertex attribute with custom specifications:
///
/// ```
/// use shared_lib::gl_vertex_attribute::*;
/// use shared_lib::gl_types::*;
///
/// let color_attribute = VertexAttribute::new(VertexAttributeType::Position)
///    .with_name("position")
///    .with_size(3);
/// ```
///
/// This struct is flexible and allows for the definition of vertex attributes
/// to be as simple or detailed as necessary to match the requirements of the
/// shader programs and the layout of the vertex data.
pub struct VertexAttribute {
    /// Optional name of the attribute, useful when querying by name in shader programs.
    pub name: Option<String>,
    /// Enum specifying the semantic meaning (e.g., Position, Color) of the attribute.
    pub attribute_type: VertexAttributeType,
    /// Optional detailed specifications including component count, data type,
    /// and normalization flag. Overrides `attribute_type` when set.
    pub attribute_specs: Option<VertexAttributeSpecs>,
    pub layout_specs: Option<VertexLayoutSpecs>,
}

/// Provides a default implementation for `VertexAttribute`.
///
/// This implementation is useful for creating a `VertexAttribute` instance with default values.
/// It sets up a basic attribute with sensible defaults, suitable as a starting point for further customization.
///
/// # Default Values
///
/// - `name`: Set to `None`, indicating that by default, the attribute does not have a name.
/// This is suitable for scenarios where vertex attributes are accessed by their layout location
/// in the shader, rather than by name.
///
/// - `size`: Set to `0`, implying that the default attribute has no size. This should be updated
/// to reflect the actual size of the attribute in bytes, based on the number and type of components
/// it represents.
///
/// - `attribute_type`: Initialized to `VertexAttributeType::Position`, assuming a positional attribute
/// by default. This can be customized to match the specific type of vertex attribute being defined.
///
/// - `attribute_specs`: Set to `None`, indicating there are no detailed specifications for the default
/// attribute. Custom specifications, such as the number of components, data type, and normalization,
/// can be added as needed.
///
/// - `stride`: Set to `0`, reflecting that, by default, there is no stride between vertices for this attribute.
/// The stride should be defined based on the layout of the vertex buffer to ensure correct parsing of vertex data.
///
/// - `offset`: Also set to `0`, indicating the attribute starts at the beginning of a vertex. This should be
/// adjusted to specify the byte offset of the attribute within the vertex data structure.
///
/// # Example Usage
///
/// ```no-run
/// // Create a default `VertexAttribute` instance
/// use shared_lib::gl_types::VertexDataType;
/// use shared_lib::gl_vertex_attribute::*;
/// use shared_lib::prelude::VertexAttributeType;
/// let default_attribute = VertexAttribute::default();
///
/// // Customizing the default instance
/// let position_attribute = VertexAttribute {
///     name: Some("position".to_string()),
///     attribute_type: VertexAttributeType::Position,
///     attribute_specs: Some(VertexAttributeSpecs::new(3, VertexDataType::Float, false)),
///     ..default_attribute
/// };
/// ```
///
/// This `Default` implementation simplifies the creation of `VertexAttribute` instances
/// with typical or placeholder values, which can then be customized as needed for specific use cases.

impl Default for VertexAttribute {
    fn default() -> Self {
        VertexAttribute {
            name: None,
            attribute_type: VertexAttributeType::Position,
            attribute_specs: None,
            layout_specs: None,
        }
    }
}

impl VertexAttribute {
    /// Creates a new instance of `VertexAttribute` with specified size and attribute type.
    ///
    /// This constructor function initializes a `VertexAttribute` with basic information
    /// required to define a vertex attribute's layout in memory. It is particularly useful
    /// for quickly setting up attributes without custom specifications or names.
    ///
    /// # Parameters
    ///
    /// - `size`: The size of the attribute in bytes. This should reflect the total byte size
    /// of the attribute data per vertex. For example, for an attribute comprising three `f32`
    /// components (e.g., a position vector), the size would be `3 * sizeof(f32)`.
    /// This allows specifying the semantic meaning or usage of the attribute (e.g., position, color).
    ///
    /// # Returns
    ///
    /// Returns a `VertexAttribute` instance with the specified `size` and `attribute_type`,
    /// and with default values for other fields: `name` is set to `None`, indicating no attribute name;
    /// `attribute_specs` is also set to `None`, indicating no additional specifications are provided;
    /// `stride` and `offset` are set to `0`, assuming no specific layout within the vertex buffer.
    ///
    /// # Examples
    ///
    /// Creating a vertex attribute for position with a size of 12 bytes (assuming 3 components of type `f32`):
    ///
    /// ```
    /// use shared_lib::gl_vertex_attribute::*;
    /// use shared_lib::gl_types::*;
    /// let position_attribute = VertexAttribute::new(VertexAttributeType::Position);
    /// ```
    ///
    /// Note: After creating a `VertexAttribute` instance using this method, you may need
    /// to manually set the `stride` and `offset` properties to properly configure the attribute
    /// within your vertex buffer layout.
    pub fn new(attribute_type: VertexAttributeType) -> Self {
        Self {
            attribute_type,
            ..Self::default()
        }
    }

    /// Creates a new instance of `VertexAttribute` with a specified name, size, and attribute type.
    ///
    /// This constructor function allows for the specification of an attribute's name in addition
    /// to its size and type. The name is useful for shader programs that reference vertex attributes
    /// by name rather than by layout position. This method is ideal for attributes that need to be
    /// identified within shader code, providing a clear linkage between the vertex buffer layout
    /// and the shader attributes.
    ///
    /// # Generics
    ///
    /// - `N`: A generic parameter that implements the `AsRef<str>` trait, allowing for flexible
    /// string types to be used as the name parameter (e.g., `&str`, `String`).
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the vertex attribute as it will be used in shader programs.
    /// - `attribute_type`: The type of the vertex attribute, defined by the `VertexAttributeType` enum.
    /// This parameter specifies the semantic meaning or intended use of the attribute (e.g., position, color).
    ///
    /// # Returns
    ///
    /// Returns a `VertexAttribute` instance with the specified `name`, `size`, and `attribute_type`.
    /// The `name` is wrapped in a `Some` indicating it's explicitly provided. The `attribute_specs`
    /// is set to `None`, indicating no additional specifications are given; `stride` and `offset` are
    /// initialized to `0`, assuming a default layout within the vertex buffer.
    ///
    /// # Examples
    ///
    /// Creating a named vertex attribute for position with a size of 12 bytes (3 components of type `f32`):
    ///
    /// ```
    /// use shared_lib::gl_vertex_attribute::*;
    /// use shared_lib::gl_types::*;
    /// let position_attribute = VertexAttribute::new_with_name(VertexAttributeType::Position, "position");
    /// ```
    ///
    /// This instance can then be used to configure vertex buffers and shader attribute bindings,
    /// ensuring that attributes are correctly identified and linked within your rendering pipeline.
    pub fn new_with_name<N: AsRef<str>>(attribute_type: VertexAttributeType, name: N) -> Self {
        Self {
            name: Some(name.as_ref().to_string()),
            attribute_type,
            ..Self::default()
        }
    }

    /// Sets the name field of the instance, consuming and returning self for method chaining.
    pub fn with_name<N: AsRef<str>>(mut self, name: N) -> Self {
        self.name = Some(name.as_ref().to_string());
        self
    }

    pub fn with_attribute_type(mut self, attribute_type: VertexAttributeType) -> Self {
        self.attribute_type = attribute_type;
        self
    }

    pub fn with_attribute_specs(mut self, attribute_specs: VertexAttributeSpecs) -> Self {
        self.attribute_specs = Some(attribute_specs);
        self
    }

    pub fn with_layout_specs(mut self, layout_specs: VertexLayoutSpecs) -> Self {
        self.layout_specs = Some(layout_specs);
        self
    }

    /// Sets both the stride and offset for the vertex attribute and returns the modified instance.
    ///
    /// # Parameters
    ///
    /// - `stride`: The byte offset between consecutive vertices in a vertex buffer.
    /// - `offset`: The byte offset of the attribute from the start of a vertex in a vertex buffer.
    ///
    /// # Returns
    ///
    /// Returns the `VertexAttribute` instance with the updated stride and offset.
    ///
    /// # Example
    ///
    /// ```
    /// use shared_lib::gl_vertex_attribute::*;
    /// use shared_lib::gl_types::*;
    /// let attribute = VertexAttribute::new(VertexAttributeType::Position)
    ///     .with_stride_and_offset(24, 0);
    /// ```
    pub fn with_stride_and_offset(mut self, stride: u32, offset: u32) -> Self {
        self.layout_specs = Some(VertexLayoutSpecs::new(stride, offset));
        self
    }

    /// Calculates the byte size of the attribute based on its specifications or its type.
    pub fn calculate_size(&self) -> usize {
        if let Some(specs) = &self.attribute_specs {
            specs.components as usize * specs.data_type.size()
        } else {
            let (components, data_type, _) = self.attribute_type.to_gl_data();
            components as usize * gl_enum_size(data_type)
        }
    }
}

impl Debug for VertexAttribute {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Clone for VertexAttribute {
    fn clone(&self) -> Self {
        VertexAttribute {
            name: self.name.clone(),
            attribute_type: self.attribute_type,
            attribute_specs: self.attribute_specs,
            layout_specs: self.layout_specs.clone(),
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// - VertexAttributeSpecs  -
//////////////////////////////////////////////////////////////////////////////

/// Specifies detailed configuration for a vertex attribute.
///
/// This struct provides additional details necessary for describing how each vertex attribute
/// should be interpreted by the GPU. It includes the number of components in the attribute,
/// the data type of those components, and whether the attribute data should be normalized.
///
/// # Fields
///
/// - `components`: A `u32` specifying the number of components per attribute. This defines
/// how many individual pieces of data make up a single attribute. For example, a position
/// attribute might have 3 components (x, y, z), while a color attribute might have 4 components
/// (r, g, b, a).
///
/// - `data_type`: Specifies the type of data each component of the attribute is, using the
/// `VertexDataType` enum. This type informs the GPU about how to interpret the raw bytes of
/// each component. Common types include `Float` for floating-point data and `Int` for integer data.
///
/// - `normalized`: A `bool` indicating whether the attribute data should be normalized.
/// Normalization is relevant for integer types, converting their values to a floating-point range
/// (e.g., [0, 1] for unsigned types or [-1, 1] for signed types) when accessed by the GPU. This is
/// typically used for attributes like color data, where the integer values represent a fraction of
/// a whole (like color intensity).
///
/// # Example
///
/// ```
/// use shared_lib::gl_vertex_attribute::*;
/// use shared_lib::gl_types::*;
/// let specs = VertexAttributeSpecs {
///     components: 3,
///     data_type: VertexDataType::Float,
///     normalized: false,
/// };
/// ```
///
/// This example creates a `VertexAttributeSpecs` instance for a vertex attribute with 3 components
/// per attribute (such as a position vector), specifies that each component is a floating-point
/// value (`Float`), and indicates that the data does not need normalization.
#[derive(Debug, Copy, Clone)]
pub struct VertexAttributeSpecs {
    /// The number of components per attribute
    /// (e.g., 2 for texture coordinates, 3 for position vectors, etc.)
    pub components: u32,
    pub data_type: VertexDataType,
    /// Should the data be normalized?
    pub normalized: bool,
}

/// Constructs a new `VertexAttributeSpecs` instance.
///
/// This method provides a straightforward way to create a `VertexAttributeSpecs` object,
/// encapsulating the details required to fully describe a vertex attribute's specifications,
/// including the number of components, data type, and normalization flag.
///
/// # Parameters
///
/// - `components`: The number of individual data components per vertex attribute. This value
///   determines how many values are associated with each instance of the attribute. For example,
///   a texture coordinate might have 2 components, while a position vector might have 3 components.
///
/// - `data_type`: The type of data these components represent, specified by the `VertexDataType` enum.
///   This defines how the GPU should interpret the bytes that make up each component of the attribute.
///
/// - `normalized`: A boolean indicating whether the attribute data should undergo normalization.
///   This is typically relevant for integer data types, where it specifies whether the values should be
///   normalized to a specific range (e.g., [0, 1] or [-1, 1]) when accessed in a shader.
///
/// # Returns
///
/// A `VertexAttributeSpecs` instance with the specified configuration.
///
/// # Example
///
/// Creating specifications for a 3-component position attribute with floating-point data that does not
/// require normalization:
///
/// ```
/// use shared_lib::gl_vertex_attribute::*;
/// use shared_lib::gl_types::*;
/// let position_specs = VertexAttributeSpecs::new(3, VertexDataType::Float, false);
/// ```
///
/// This `VertexAttributeSpecs` instance can then be used to define a vertex attribute for a shader,
/// providing precise control over how vertex data is interpreted and used during rendering.
impl VertexAttributeSpecs {
    pub fn new(components: u32, data_type: VertexDataType, normalized: bool) -> Self {
        Self {
            components,
            data_type,
            normalized,
        }
    }
}

/// Enables creating a `VertexAttributeSpecs` instance from a tuple of `(u32, VertexDataType, bool)`.
///
/// This implementation facilitates a convenient way to instantiate `VertexAttributeSpecs` using
/// a tuple, directly mapping tuple elements to the structs fields. It simplifies scenarios where
/// vertex attribute specifications are dynamically generated or passed around in tuple form.
///
/// # Parameters
///
/// The tuple consists of three values:
/// - The first element (`u32`) specifies the number of components per vertex attribute.
/// - The second element (`VertexDataType`) defines the data type of the attribute's components.
/// - The third element (`bool`) indicates whether the attribute data should be normalized.
///
/// # Returns
///
/// Returns a new instance of `VertexAttributeSpecs` initialized with the values from the tuple.
///
/// # Example
///
/// Converting a tuple into a `VertexAttributeSpecs` instance:
///
/// ```
/// use shared_lib::gl_vertex_attribute::*;
/// use shared_lib::gl_types::*;
/// let specs_tuple = (3, VertexDataType::Float, false); // Example tuple for position attributes
/// let specs: VertexAttributeSpecs = specs_tuple.into(); // Using `into()` for conversion
/// ```
///
/// This use of the `From` trait provides an idiomatic and flexible way to work with vertex attribute
/// specifications in Rust, leveraging Rust's type conversion features for cleaner and more concise code.
impl From<(u32, VertexDataType, bool)> for VertexAttributeSpecs {
    fn from(value: (u32, VertexDataType, bool)) -> VertexAttributeSpecs {
        VertexAttributeSpecs {
            components: value.0,
            data_type: value.1,
            normalized: value.2,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// - VertexLayoutSpecs -
//////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct VertexLayoutSpecs {
    /// Byte stride between consecutive vertices within a vertex buffer. Determines how many bytes
    /// to skip to advance to the next vertex. This value is crucial for interleaved vertex
    /// formats where multiple attributes are stored in a single array.
    pub stride: u32,
    /// Byte offset from the start of a vertex to this attribute's data. Specifies the start
    /// position of the attribute data in each vertex, allowing for precise attribute location
    /// within an interleaved data format.
    pub offset: u32,
}

impl VertexLayoutSpecs {
    pub fn new(stride: u32, offset: u32) -> Self {
        Self { stride, offset }
    }

    pub fn from(layout_specs: &VertexLayoutSpecs) -> VertexLayoutSpecs {
        VertexLayoutSpecs {
            stride: layout_specs.stride,
            offset: layout_specs.offset,
        }
    }
}

impl From<[u32; 2]> for VertexLayoutSpecs {
    fn from(value: [u32; 2]) -> Self {
        Self {
            stride: value[0],
            offset: value[1],
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// - VertexLayoutManager -
//////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct VertexLayoutManager {
    attributes: Vec<VertexAttribute>,
    // Maps attribute index to stride and offset.
    layout_info: HashMap<usize, [u32; 2]>,
}

impl VertexLayoutManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_type<T: Vertex>() -> Self {
        let mut manager = Self {
            attributes: T::attributes(),
            layout_info: HashMap::new(),
        };
        manager.calculate_layout_info();
        manager
    }

    pub fn new_with_attributes(attributes: &[VertexAttribute]) -> Self {
        let mut manager = Self {
            attributes: attributes.to_vec(),
            layout_info: HashMap::new(),
        };
        manager.calculate_layout_info();
        manager
    }

    pub fn new_and_setup<T: Vertex>(shader: &ShaderProgram) -> Result<Self> {
        let mut manager = Self::new_with_type::<T>();
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
                index as u32,                // Attribute index
                comp as i32,                 // Number of components per attribute
                gl::FLOAT,                   // Data type of each component
                gl::FALSE,                   // Normalized
                stride,                      // Stride
                offset as *const gl::types::GLvoid, // Offset
            );
        }

        // Increment offset by the size of this attribute
        offset += comp as GLsizei * float_size;
    }
}
