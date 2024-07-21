use crate::gl_prelude::VertexAttributeType;
use crate::gl_traits::{Bindable, Deletable};
use crate::gl_types::convert_attributes;
use crate::opengl::vertex_attribute::VertexAttribute;
use crate::{RenderDataState, RenderPrepare};
use anyhow::Result;
use gl::types::{GLboolean, GLint, GLsizei, GLuint, GLvoid};
use sdl2::filesystem::PrefPathError;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::c_void;

/// Represents a Vertex Array Object (VAO) in OpenGL, which stores the format
/// of the vertex data as well as the buffers that provide the vertex data.
///
/// # Fields
/// * `id` - A unique identifier for the Vertex Array Object.
/// * `layout` - An optional `RefCell` containing layout data associated with the VAO.
pub struct VertexArrayObject {
    id: u32,
    layout: Option<RefCell<LayoutData>>,
}

/// Implements the `Default` trait for the `VertexArrayObject` struct.
///
/// This implementation generates a new OpenGL Vertex Array Object (VAO) and
/// initializes the `VertexArrayObject` with the generated VAO ID and no layout data.
///
/// # Safety
/// This function contains an unsafe block to call the OpenGL function `gl::GenVertexArrays`.
///
/// # Returns
/// A new instance of `VertexArrayObject` with a generated VAO ID and `None` layout data.
impl Default for VertexArrayObject {
    fn default() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        Self {
            id,
            layout: None,
        }
    }
}

impl VertexArrayObject {
    /// Creates a new `VertexArrayObject` and initializes it with the given vertex attributes.
    ///
    /// # Type Parameters
    /// * `T` - A type that can be converted to a slice of `VertexAttribute`.
    ///
    /// # Parameters
    /// * `attributes` - A collection of vertex attributes that will define the layout for the VAO.
    ///
    /// # Returns
    /// A new `VertexArrayObject` initialized with the specified vertex attributes.
    ///
    /// # Example
    /// ```ignore
    /// let attributes = vec![VertexAttribute::new(...), VertexAttribute::new(...)];
    /// let vao = VertexArrayObject::new_with_attributes(attributes);
    /// ```
    ///
    /// # Safety
    /// This function internally calls the `set_layout` method which may involve OpenGL operations.
    pub fn new_with_attributes<T: AsRef<[VertexAttribute]>>(attributes: T) -> Self {
        let mut vao = VertexArrayObject::default();
        vao.set_layout(attributes);
        vao
    }

    /// Creates a new `VertexArrayObject` and initializes it with the given vertex attribute types.
    ///
    /// This function converts the provided attribute types into vertex attributes and then uses
    /// them to set up the layout of the `VertexArrayObject`.
    ///
    /// # Type Parameters
    /// * `T` - A type that can be converted to a slice of `VertexAttributeType`.
    ///
    /// # Parameters
    /// * `attribute_types` - A collection of vertex attribute types that will be converted to
    ///   vertex attributes for the VAO layout.
    ///
    /// # Returns
    /// A new `VertexArrayObject` initialized with the vertex attributes derived from the specified
    /// attribute types.
    ///
    /// # Example
    /// ```ignore
    /// let attribute_types = vec![VertexAttributeType::new(...), VertexAttributeType::new(...)];
    /// let vao = VertexArrayObject::new_with_attribute_types(attribute_types);
    /// ```
    ///
    /// # Safety
    /// This function internally calls the `new_with_attributes` method, which involves OpenGL operations.
    pub fn new_with_attribute_types<T: AsRef<[VertexAttributeType]>>(attribute_types: T) -> Self {
        let attribute_type_slice = attribute_types.as_ref();
        let attributes: Vec<VertexAttribute> = attribute_type_slice
            .iter()
            .map(|attribute_type| attribute_type.to_vertex_attribute())
            .collect();
        Self::new_with_attributes(attributes)
    }

    /// Binds the `VertexArrayObject` to the current OpenGL context.
    ///
    /// This function makes the VAO the current VAO so that subsequent OpenGL
    /// operations use this VAO.
    ///
    /// # Safety
    /// This function contains an unsafe block to call the OpenGL function `gl::BindVertexArray`.
    ///
    /// # Example
    /// ```ignore
    /// let vao = VertexArrayObject::new_with_attributes(...);
    /// vao.bind();
    /// ```
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    /// Unbinds the currently bound `VertexArrayObject` from the OpenGL context.
    ///
    /// This function sets the current VAO to zero, effectively unbinding any VAO
    /// that might be bound.
    ///
    /// # Safety
    /// This function contains an unsafe block to call the OpenGL function `gl::BindVertexArray` with zero.
    ///
    /// # Example
    /// ```ignore
    /// VertexArrayObject::unbind();
    /// ```
    pub fn unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    /// Checks if the `VertexArrayObject` is currently bound to the OpenGL context.
    ///
    /// This function queries the current VAO binding and compares it with the VAO's ID.
    ///
    /// # Returns
    /// A `Result` containing `true` if the VAO is currently bound, `false` otherwise.
    ///
    /// # Errors
    /// This function does not currently produce any errors, so it always returns `Ok`.
    ///
    /// # Safety
    /// This function contains an unsafe block to call the OpenGL function `gl::GetIntegerv`.
    ///
    /// # Example
    /// ```ignore
    /// let vao = VertexArrayObject::new_with_attributes(...);
    /// vao.bind();
    /// assert!(vao.is_bound().unwrap());
    /// ```
    fn is_bound(&self) -> Result<bool> {
        let mut current_vao = 0;
        unsafe {
            gl::GetIntegerv(gl::VERTEX_ARRAY_BINDING, &mut current_vao);
        }
        Ok(current_vao == self.id as GLint)
    }

    /// Returns the identifier of the vertex array.
    pub fn array_id(&self) -> u32 {
        self.id
    }

    /// Retrieves the current layout state of the `VertexArrayObject`.
    ///
    /// This function checks if the VAO has layout data. If it does, it returns the state of
    /// the layout data. Otherwise, it returns `RenderDataState::None`.
    ///
    /// # Returns
    /// The current `RenderDataState` of the VAO. If there is no layout data, it returns `RenderDataState::None`.
    ///
    /// # Example
    /// ```ignore
    /// let vao = VertexArrayObject::new_with_attributes(...);
    /// let layout_state = vao.get_layout_state();
    /// ```
    ///
    /// # Panics
    /// This function may panic if the layout data's `RefCell` is already borrowed mutably.
    fn get_layout_state(&self) -> RenderDataState {
        if let Some(layout_data) = &self.layout {
            return layout_data.borrow().layout_data_state;
        }
        RenderDataState::None
    }

    /// Sets the layout for the VertexArrayObject and updates its render data state accordingly.
    ///
    /// This method accepts a collection of `VertexAttribute` elements, determines the current
    /// render data state, and updates the internal `layout_data` with the new layout and state.
    ///
    /// # Type Parameters
    /// - `T`: A type that can be referenced as a slice of `VertexAttribute`.
    ///
    /// # Parameters
    /// - `attributes`: The new vertex attributes to set for the layout. This parameter can be any
    ///   type that implements the `AsRef<[VertexAttribute]>` trait, which allows for flexible input
    ///   types such as `Vec<VertexAttribute>`, `&[VertexAttribute]`, and others.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut vao = VertexArrayObject::new();
    /// let attributes = vec![VertexAttribute { /* fields */ }];
    /// vao.set_layout(attributes);
    /// ```
    ///
    /// This will set the layout of the `VertexArrayObject` to the provided attributes and update
    /// its render data state based on its current state.
    pub fn set_layout<T: AsRef<[VertexAttribute]>>(&mut self, attributes: T) {
        let attributes = attributes.as_ref().to_vec();
        let new_state = match self.get_layout_state() {
            RenderDataState::None | RenderDataState::Provided => RenderDataState::Provided,
            RenderDataState::Uploaded | RenderDataState::NeedsUpdate => {
                RenderDataState::NeedsUpdate
            }
        };

        let layout_data = LayoutData {
            layout: attributes.clone(),
            layout_data_state: new_state,
        };

        self.layout = Some(RefCell::new(layout_data));
    }

    /// Clears the layout data of the `VertexArrayObject`.
    ///
    /// This function binds the VAO, sets its layout data to `None`, and then unbinds the VAO.
    ///
    /// # Example
    /// ```ignore
    /// let mut vao = VertexArrayObject::new_with_attributes(...);
    /// vao.clear_layout();
    /// ```
    ///
    /// # Safety
    /// This function involves binding and unbinding the VAO, which includes OpenGL operations.
    pub fn clear_layout(&mut self) {
        self.bind();
        self.layout = None;
        Self::unbind();
    }

    /// Checks if the `VertexArrayObject` has layout data.
    ///
    /// This function returns `true` if the VAO has layout data, and `false` otherwise.
    ///
    /// # Returns
    /// A boolean indicating whether the VAO has layout data.
    ///
    /// # Example
    /// ```ignore
    /// let vao = VertexArrayObject::new_with_attributes(...);
    /// assert!(vao.has_layout());
    /// ```
    pub fn has_layout(&self) -> bool {
        self.layout.is_some()
    }

    /// Renders the object using the specified number of triangles or EBO entries.
    ///
    /// If `use_ebo` is true, the method uses the EBO to render the specified number of elements.
    /// Otherwise, it uses the VBO to render the specified number of vertices.
    ///
    /// # Parameters
    /// - `use_ebo`: A boolean indicating whether to use the EBO for rendering.
    /// - `count`: The number of triangles or EBO entries to render.
    pub fn render(&self, use_ebo: bool, count: usize) {
        self.prepare_render();
        self.bind();
        let count = count as GLsizei;
        if use_ebo {
            unsafe {
                gl::DrawElements(gl::TRIANGLES, count, gl::UNSIGNED_INT, std::ptr::null());
            }
        } else {
            unsafe {
                gl::DrawArrays(gl::TRIANGLES, 0, count);
            }
        }
        Self::unbind();
    }

    /// Uploads the layout data of the `VertexArrayObject` to the GPU.
    ///
    /// If the VAO has layout data, this function binds the VAO, uploads the layout data
    /// to the GPU, and then unbinds the VAO.
    ///
    /// # Example
    /// ```ignore
    /// let vao = VertexArrayObject::new_with_attributes(...);
    /// vao.upload_layout_data();
    /// ```
    ///
    /// # Safety
    /// This function involves binding and unbinding the VAO, and uploading data to the GPU,
    /// which includes OpenGL operations. The function also temporarily borrows the layout data
    /// mutably.
    ///
    /// # Panics
    /// This function may panic if the layout data's `RefCell` is already borrowed mutably.
    fn upload_layout_data(&self) {
        if let Some(layout) = self.layout.as_ref() {
            self.bind();
            let mut layout = layout.borrow_mut();
            layout.upload_to_gpu();
            layout.layout_data_state = RenderDataState::Uploaded;
            Self::unbind();
        }
    }
}

impl Deletable for VertexArrayObject {
    fn delete(&mut self) -> Result<()> {
        if self.id != 0 {
            unsafe {
                gl::DeleteVertexArrays(1, &self.id);
            }
            self.id = 0;
        }
        Ok(())
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        if let Err(err) = self.delete() {
            eprintln!("Error while dropping VertexArrayObject: {}", err);
            // You might choose to log the error or take other appropriate actions here.
        }
    }
}

impl RenderPrepare for VertexArrayObject {
    fn prepare_render(&self) {
        match self.get_layout_state() {
            RenderDataState::Provided | RenderDataState::NeedsUpdate => self.upload_layout_data(),
            _ => {}
        }
    }
}

struct LayoutData {
    layout: Vec<VertexAttribute>,
    layout_data_state: RenderDataState,
}

impl Drop for LayoutData {
    fn drop(&mut self) {
        if self.layout_data_state > RenderDataState::Provided {
            for (i, _attr) in self.layout.iter().enumerate() {
                unsafe {
                    gl::DisableVertexAttribArray(i as GLuint);
                }
            }
        }
    }
}

impl LayoutData {
    fn upload_to_gpu(&mut self) {
        // Calculate the stride if it is 0
        let mut stride_map = HashMap::<usize, i32>::new();
        for (i, attr) in self.layout.iter().enumerate() {
            if attr.stride == 0 {
                stride_map.insert(
                    i,
                    self.layout
                        .iter()
                        .map(|a| a.components as i32 * a.data_type.size() as i32)
                        .sum(),
                );
            }
        }

        // Calculate the offset if it is not set
        let mut offset_map = HashMap::<usize, u32>::new();
        for (i, attr) in self.layout.iter().enumerate() {
            if attr.offset.is_none() {
                offset_map.insert(
                    i,
                    self.layout
                        .iter()
                        .take(i)
                        .map(|a| a.components as u32 * a.data_type.size() as u32)
                        .sum(),
                );
            }
        }

        // Upload the attribute data to the GPU
        for (i, attr) in self.layout.iter().enumerate() {
            let normalized = attr.normalized;
            let stride = if attr.stride > 0 {
                attr.stride
            } else {
                stride_map[&i]
            };
            let offset = attr.offset.unwrap_or(offset_map[&i]) as usize;
            let type_ = attr.data_type.to_gl_enum();

            unsafe {
                gl::VertexAttribPointer(
                    i as GLuint,
                    attr.components as GLint,
                    type_,
                    normalized as GLboolean,
                    stride,
                    offset as *const GLvoid,
                );
                gl::EnableVertexAttribArray(i as GLuint);
            }
        }
    }
}
