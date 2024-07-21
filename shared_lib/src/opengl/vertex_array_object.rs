use crate::gl_prelude::VertexAttributeType;
use crate::gl_traits::{Bindable, Deletable};
use crate::gl_types::convert_attributes;
use crate::opengl::vertex_attribute::VertexAttribute;
use crate::{RenderDataState, RenderPrepare};
use anyhow::Result;
use gl::types::{GLboolean, GLint, GLsizei, GLuint, GLvoid};
use sdl2::filesystem::PrefPathError;
use std::cell::RefCell;
use std::ffi::c_void;

pub struct VertexArrayObject {
    id: u32,
    layout: Option<RefCell<LayoutData>>,
}

impl VertexArrayObject {
    /// Create a new Vertex Array Object and bind it.
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        Self {
            id,
            layout: None,
        }
    }

    pub fn new_with_attributes<T: AsRef<[VertexAttribute]>>(attributes: T) -> Self {
        let mut vao = VertexArrayObject::new();
        vao.set_layout(attributes);
        vao
    }

    pub fn new_with_attribute_types<T: AsRef<[VertexAttributeType]>>(attribute_types: T) -> Self {
        let attribute_type_slice = attribute_types.as_ref();
        let attributes: Vec<VertexAttribute> = attribute_type_slice
            .iter()
            .map(|attribute_type| attribute_type.to_vertex_attribute())
            .collect();
        Self::new_with_attributes(attributes)
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

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

    pub fn clear_layout(&mut self) {
        self.bind();
        self.layout = None;
        Self::unbind();
    }

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

    fn upload_layout_data(&self) {
        if let Some(layout) = self.layout.as_ref() {
            self.bind();
            layout.borrow_mut().upload_to_gpu();
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
        for (i, attr) in self.layout.iter().enumerate() {
            let normalized = attr.normalized;
            let stride = attr.stride;
            let offset = attr.offset;
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
