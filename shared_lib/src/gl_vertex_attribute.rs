// Ensure you have the `gl` crate added to your `Cargo.toml`
extern crate gl;

use std::collections::HashMap;
use std::fmt::Debug;

use anyhow::anyhow;
use anyhow::Result;
use gl::types::{GLboolean, GLenum, GLint, GLsizei, GLuint, GLvoid};
use thiserror::Error;

use crate::gl_prelude::Vertex;
use crate::gl_types::{VertexAttributeType, VertexDataType};
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

//////////////////////////////////////////////////////////////////////////////
// - VertexLayoutManager -
//////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default)]
pub struct VertexLayoutManager {
    attributes: Vec<VertexAttribute>,
    // Maps attribute index to stride and offset.
    layout_info: HashMap<usize, [u32; 2]>,
    layouts: HashMap<String, VertexLayoutManager>,
    is_setup: bool,
    shader_id: Option<u32>,
}

impl VertexLayoutManager {
    /// Creates a new instance with no attributes and an empty layout.
    pub fn empty() -> Self {
        Self {
            attributes: vec![],
            layout_info: HashMap::new(),
            layouts: HashMap::new(),
            is_setup: false,
            shader_id: None,
        }
    }

    /// Constructs a new instance using attributes from the Vertex trait and initializes layout info.
    pub fn new<T: Vertex>() -> Self {
        let mut manager = Self {
            attributes: T::attributes(),
            layout_info: HashMap::new(),
            layouts: HashMap::new(),
            is_setup: false,
            shader_id: None,
        };
        manager.calculate_layout_info();
        manager
    }

    pub fn from_attributes(attributes: Vec<VertexAttribute>) -> Self {
        let mut layout_manager = Self::empty();
        for attribute in attributes {
            layout_manager.add_attribute(attribute);
        }
        layout_manager
    }

    pub fn from_attribute_types(attribute_types: Vec<VertexAttributeType>) -> Self {
        let mut layout_manager = Self::empty();
        for attribute_type in attribute_types {
            layout_manager.add_attribute(attribute_type.to_vertex_attribute());
        }
        layout_manager
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

    pub fn add_attribute_type(&mut self, attribute_type: VertexAttributeType) -> &mut Self {
        self.attributes.push(attribute_type.to_vertex_attribute());
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

    /// Checks if the vertex layout's attributes have already been set up.
    ///
    /// This method verifies whether the layout attributes are already configured,
    /// potentially logging the setup status. If the layout is already set up and
    /// a shader program ID is associated with it, this ID is also logged, indicating
    /// that reinitialization is unnecessary.
    ///
    /// # Returns
    /// - `true`: Indicates that the layout is not set up and needs initialization.
    /// - `false`: Indicates that the layout is already set up and does not require
    ///   further initialization.
    ///
    /// # Usage
    /// This function should be called before attempting to set up layout attributes.
    /// It provides a mechanism to avoid redundant operations by confirming whether
    /// the attribute setup needs to be repeated or can be safely skipped.
    ///
    /// # Example
    /// ```no-run
    /// if layout.check_setup_attributes() {
    ///     layout.setup_attributes()?;
    /// }
    /// ```
    ///
    /// This method ensures that your application performs efficiently by preventing
    /// unnecessary re-setup of vertex layout attributes, especially in scenarios
    /// where multiple rendering passes or shader changes might occur.
    fn check_setup_attributes(&self) -> bool {
        if self.is_setup {
            if let Some(shader_id) = self.shader_id {
                println!(
                    "Layout already setup for shader program id {}, skipping reinitialization.",
                    shader_id
                )
            } else {
                println!("Layout already set up, skipping reinitialization.");
            }
            return false;
        }
        true
    }

    /// Sets up vertex attributes for this layout if they have not been set up already.
    ///
    /// This method processes each vertex attribute defined in the layout, applying
    /// the necessary OpenGL commands to configure them for rendering. It checks if
    /// the layout is already set up using `check_setup_attributes` to avoid redundant
    /// setup. If the layout is already set up, it skips reinitialization.
    ///
    /// # Returns
    /// - `Ok(())`: Successfully sets up the attributes.
    /// - `Err(VertexLayoutError)`: Returns an error if there is a failure during the
    ///   layout finalization or OpenGL configuration process.
    ///
    /// # Errors
    /// This method propagates errors from `finalize_layout`, `check_and_map_gl_error`,
    /// or any issues with attribute configuration. Possible errors include:
    /// - `VertexLayoutError::InvalidShaderId`: If the shader program ID is not valid.
    /// - `VertexLayoutError::OpenGL(String)`: For errors directly from OpenGL calls.
    ///
    /// # Safety
    /// This function contains unsafe blocks for calling OpenGL functions necessary
    /// for setting up vertex attributes. The caller must ensure that the current
    /// OpenGL context is active and that it is safe to call OpenGL functions.
    ///
    /// # Usage
    /// Call this method when initializing a vertex layout or when you need to
    /// reconfigure the vertex attributes (e.g., after changing shaders or attribute
    /// specifications). It ensures that the GPU's vertex attribute pointers are
    /// correctly set up for the current layout.
    ///
    /// # Example
    /// Assuming you have a `VertexLayout` instance `layout`:
    /// ```no-run
    /// layout.setup_attributes()?;
    /// ```
    pub fn setup_attributes(&mut self) -> Result<(), VertexLayoutError> {
        self.finalize_layout()?;

        for (index, attribute) in self.attributes.iter().enumerate() {
            // if !self.check_setup_attributes() {
            //     return Ok(());
            // }

            // Determine the attribute properties from VertexAttributeType
            let (components, data_type, normalized): (u8, GLenum, bool) = (
                attribute.components,
                attribute.data_type.to_gl_enum(),
                attribute.normalized.unwrap_or_default(),
            );

            // Fetch stride and offset from layout_specs or use the calculated layout info
            let (stride, offset) = self.get_stride_and_offset(index, attribute);

            unsafe {
                // Setup the vertex attribute pointer
                let gl_index = index as GLuint;
                gl::EnableVertexAttribArray(gl_index);
                gl::VertexAttribPointer(
                    gl_index as GLuint,
                    components as GLint,
                    data_type,
                    normalized as GLboolean,
                    stride as GLsizei,
                    offset as *const GLvoid,
                );
            }

            // Check for GL errors after setting up the vertex attribute
            check_and_map_gl_error()?;
        }

        self.is_setup = true;
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

        // Iterate over each attribute
        for (index, attribute) in self.attributes.iter().enumerate() {
            println!(
                "Processing attribute {} for shader {}",
                index, shader_program_id
            );

            // Determine the attribute properties from VertexAttributeType
            let (components, data_type, normalized): (u8, GLenum, bool) = (
                attribute.components,
                attribute.data_type.to_gl_enum(),
                attribute.normalized.unwrap_or_default(),
            );

            // Fetch stride and offset from layout_specs or use the calculated layout info
            let (stride, offset) = self.get_stride_and_offset(index, attribute);

            // Retrieve the attribute location by name if available, or use the index from this iteration
            let attr_location = if let Some(name) = &attribute.name {
                let c_str = std::ffi::CString::new(name.as_str()).unwrap();
                unsafe { gl::GetAttribLocation(shader_program_id, c_str.as_ptr()) }
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

            unsafe {
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
            }

            // Check for GL errors after setting up the vertex attribute
            check_and_map_gl_error()?;
        }

        self.is_setup = true;
        self.shader_id = Some(shader_program_id);
        Ok(())
    }

    /// Resets the setup state of this vertex layout, allowing for reinitialization.
    ///
    /// This method clears the current setup state and associated shader program ID,
    /// making the layout ready to be set up again with new settings or after a significant
    /// change that affects the vertex attributes or shader configuration.
    ///
    /// # Effects
    /// - Clears the `is_setup` flag, indicating that the layout needs to be set up again.
    /// - Removes any associated shader program ID, disconnecting the layout from the
    ///   current shader program.
    ///
    /// # Usage
    /// Use this method when changes to the vertex attributes or shader program require
    /// reinitializing the layout. This is common in scenarios where:
    /// - The shader program changes and different attribute configurations are needed.
    /// - The vertex layout itself is altered, requiring a fresh setup.
    ///
    /// # Example
    /// Assuming `layout` is an instance of `VertexLayout` that has been previously set up:
    /// ```no-run
    /// layout.reset_setup();
    /// // Now `layout` can be set up again with new attributes or a different shader.
    /// ```
    pub fn reset_setup(&mut self) {
        self.is_setup = false;
        self.shader_id = None;
    }

    /// Retrieves the stride and offset for a vertex attribute.
    ///
    /// This function returns the stride and offset either directly from the attribute if specified,
    /// or falls back to the pre-calculated layout information based on the attribute index.
    ///
    /// # Parameters
    /// - `index`: The index of the vertex attribute in the layout.
    /// - `attribute`: A reference to the `VertexAttribute` instance.
    ///
    /// # Returns
    /// A tuple containing the stride and offset as `u32`.
    ///
    /// # Panics
    /// Panics if the layout information is not pre-calculated for the given index.
    fn get_stride_and_offset(&self, index: usize, attribute: &VertexAttribute) -> (u32, u32) {
        match (attribute.stride, attribute.offset) {
            (Some(stride), Some(offset)) => (stride, offset),
            _ => {
                // Fallback to calculated layout info if custom specs are not provided
                let layout_info = self
                    .layout_info
                    .get(&index)
                    .expect("Layout info should be calculated for all attributes.");
                (layout_info[0], layout_info[1])
            }
        }
    }

    /// Creates a new vertex layout with specified attributes and associates it
    /// with a given key.
    ///
    /// This method ensures that each layout is uniquely identified by its key. If
    /// a layout with the same key already exists, the method returns an error to
    /// prevent unintended overwrites.
    ///
    /// # Parameters
    /// - `key`: A string slice that serves as the unique identifier for the vertex
    ///          layout. This key is used to retrieve or manage the layout in
    ///          subsequent operations.
    /// - `attributes`: A vector of `VertexAttribute` instances that define the
    ///                 characteristics and behaviors of the vertex layout. These
    ///                 attributes are added to the layout during its creation.
    ///
    /// # Returns
    /// A `Result` type:
    /// - `Ok(())` indicates the successful creation of the layout.
    /// - `Err(anyhow::Error)` is returned if a layout with the provided key
    ///   already exists, detailing the issue with a descriptive message.
    ///
    /// # Errors
    /// The method returns an error if a layout with the specified key already
    /// exists, preventing the duplication of layout identifiers within the manager.
    ///
    /// # Usage
    /// This method is typically used during the setup phase of graphic applications
    /// to define how vertex data should be organized and processed. It ensures that
    /// vertex layouts are uniquely identified and managed within a centralized
    /// system, facilitating better resource management and consistency across
    /// rendering operations.
    pub fn create_layout(&mut self, key: &str, attributes: Vec<VertexAttribute>) -> Result<()> {
        if self.layouts.contains_key(key) {
            return Err(anyhow!("A layout with the key '{}' already exists.", key));
        }

        let mut vlm = VertexLayoutManager::empty();
        for attr in attributes {
            vlm.add_attribute(attr);
        }
        self.layouts.insert(key.to_string(), vlm);
        Ok(())
    }

    /// Creates or updates a vertex layout with specified attributes, associated
    /// with a given key. This function will overwrite any existing layout with
    /// the same key.
    ///
    /// # Parameters
    /// - `key`: A string slice that serves as the unique identifier for the vertex
    ///          layout. This key is used to retrieve or manage the layout in
    ///          subsequent operations.
    /// - `attributes`: A vector of `VertexAttribute` instances that define the
    ///                 characteristics and behaviors of the vertex layout. These
    ///                 attributes are added to the layout.
    ///
    /// # Behavior
    /// If a layout with the specified key already exists, it will be overwritten
    /// with the new attributes, and a warning message will be logged.
    ///
    /// # Usage
    /// This method is used when you want to ensure a vertex layout is up-to-date
    /// with the latest attributes or when replacing an old layout with new
    /// specifications is required. It provides a way to handle dynamic updates
    /// to vertex data structures in graphics applications.
    pub fn create_or_update_layout(&mut self, key: &str, attributes: Vec<VertexAttribute>) {
        let mut vlm = VertexLayoutManager::empty();
        for attr in attributes {
            vlm.add_attribute(attr);
        }
        if self.layouts.insert(key.to_string(), vlm).is_some() {
            eprintln!("Warning: Layout '{}' has been overwritten.", key);
        }
    }

    /// Deletes a vertex layout associated with the specified key. If no layout
    /// is found with the provided key, an error is returned.
    ///
    /// # Parameters
    /// - `key`: A string slice that serves as the unique identifier for the vertex
    ///          layout to be deleted.
    ///
    /// # Returns
    /// A `Result` type:
    /// - `Ok(())` indicates successful deletion of the layout.
    /// - `Err(anyhow::Error)` is returned if no layout is found with the given key,
    ///   providing details of the issue.
    ///
    /// # Errors
    /// An error is returned if no layout associated with the key exists in the
    /// manager, ensuring that attempts to delete non-existent layouts are
    /// appropriately flagged.
    ///
    /// # Usage
    /// This method is typically used when specific vertex layouts are no longer
    /// needed or during cleanup processes where resources need to be released.
    pub fn delete_layout(&mut self, key: &str) -> Result<()> {
        if self.layouts.remove(key).is_none() {
            return Err(anyhow!("No layout found with the key '{}'", key));
        }
        Ok(())
    }

    /// Clears all vertex layouts from the manager, effectively removing all stored
    /// layouts.
    ///
    /// # Behavior
    /// This function removes all entries in the vertex layout manager's internal
    /// storage, leaving it empty. A message is printed to the console to confirm
    /// that all layouts have been removed.
    ///
    /// # Usage
    /// Use this method to reset the layout manager, typically during application
    /// shutdown or when a complete refresh of all layouts is necessary. It helps
    /// ensure that no outdated or unused layouts persist in memory.
    pub fn clear_all_layouts(&mut self) {
        self.layouts.clear();
        print!("All layouts have been removed.");
    }

    /// Checks if there are any vertex layouts currently stored in the manager.
    ///
    /// # Returns
    /// - `true`: Indicates that there is at least one layout present in the manager.
    /// - `false`: Indicates that the manager is empty.
    ///
    /// # Usage
    /// This method is useful for determining whether the layout manager contains
    /// any layouts before performing operations that require existing layouts,
    /// such as drawing operations or layout deletions.
    pub fn has_layouts(&self) -> bool {
        !self.layouts.is_empty()
    }

    /// Returns the number of vertex layouts currently stored in the manager.
    ///
    /// # Returns
    /// - `usize`: The total number of layouts in the manager.
    ///
    /// # Usage
    /// This method can be used to retrieve the count of all stored layouts, which
    /// is helpful for monitoring the resource usage or for debugging purposes to
    /// ensure that the expected number of layouts are present.
    pub fn layout_count(&self) -> usize {
        self.layouts.len()
    }

    /// Activates a vertex layout associated with the specified key, setting up its
    /// attributes. This method ensures that the specified layout is prepared for
    /// rendering by configuring its vertex attributes according to the current
    /// settings.
    ///
    /// # Parameters
    /// - `key`: A string slice that represents the unique identifier for the vertex
    ///   layout to be activated.
    ///
    /// # Returns
    /// - `Ok(())`: Indicates that the layout was successfully activated and set up.
    /// - `Err(VertexLayoutError)`: Returns an error if no layout with the specified
    ///   key exists, specifying the failure reason.
    ///
    /// # Errors
    /// - `VertexLayoutError::InvalidLayoutName`: This error is returned if the layout
    ///   associated with the provided key cannot be found within the manager.
    ///
    /// # Usage
    /// This method is typically called to prepare a specific vertex layout for
    /// rendering operations within a graphics application. It is essential to call
    /// this method when switching between different vertex layouts that require
    /// different attribute configurations, especially when dealing with multiple
    /// shaders or rendering techniques.
    ///
    /// # Example
    /// Assuming you have a `VertexLayoutManager` instance named `manager`, and a
    /// layout identified by the key "basic_layout":
    /// ```no-run
    /// if let Err(e) = manager.activate_layout("basic_layout") {
    ///     println!("Failed to activate layout: {}", e);
    /// }
    /// ```
    ///
    /// This example demonstrates how to attempt to activate a vertex layout and
    /// handle a potential error if the layout does not exist.
    pub fn activate_layout(&mut self, key: &str) -> Result<(), VertexLayoutError> {
        if let Some(layout) = self.layouts.get_mut(key) {
            layout.setup_attributes()?;
            Ok(())
        } else {
            Err(VertexLayoutError::InvalidLayoutName(key.to_string()))
        }
    }

    /// Activates and sets up the vertex layout associated with the given key.
    /// This function will force a setup of the layout even if it has been previously
    /// set up, ensuring that the layout's attributes are correctly initialized.
    ///
    /// # Parameters
    /// - `key`: A string slice representing the unique identifier for the vertex
    ///          layout to be activated and set up.
    ///
    /// # Returns
    /// - `Ok(())`: Successfully activated and set up the layout.
    /// - `Err(VertexLayoutError)`: Returns an error if no layout with the specified
    ///   key exists or if there is a problem during the setup of the layout's attributes.
    ///
    /// # Errors
    /// - `VertexLayoutError::InvalidLayoutName`: Returned if no layout associated
    ///   with the given key can be found within the manager.
    /// - Errors from `setup_attributes()`: Propagates any errors encountered during
    ///   the attribute setup, which could include invalid attribute configurations or
    ///   OpenGL-related errors.
    ///
    /// # Usage
    /// This method is typically called to prepare a vertex layout for rendering
    /// operations. It ensures that all vertex attributes are correctly configured
    /// according to the layout's specifications. This setup is crucial for correct
    /// graphical rendering and should be done whenever a layout is first used or
    /// if the layout's configuration might have changed.
    pub fn force_activate_layout(&mut self, key: &str) -> Result<(), VertexLayoutError> {
        if let Some(layout) = self.layouts.get_mut(key) {
            self.is_setup = false;
            self.shader_id = None;
            layout.setup_attributes()?;
            Ok(())
        } else {
            Err(VertexLayoutError::InvalidLayoutName(key.to_string()))
        }
    }
    
    pub fn attributes_len(&mut self) -> usize {
        self.attributes.len()
    }
}

//////////////////////////////////////////////////////////////////////////////
// - VertexLayoutError -
//////////////////////////////////////////////////////////////////////////////

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
