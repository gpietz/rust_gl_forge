use crate::render_context::RenderContext;
use anyhow::Error as AnyhowError;
use anyhow::Result;
use shared_lib::gl_vertex_attribute::VertexLayoutError;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum SceneError {
    #[error("Resource loading failed")]
    ResourceLoadError,
    #[error("Invalid state transition")]
    InvalidStateTransition,
    #[error("Failed to activate scene due to VAO creation error: {0}")]
    VaoCreationError(#[from] AnyhowError),
    #[error("Failed to")]
    VertexLayoutError(#[from] VertexLayoutError),
    #[error("Failed to load texture: {name}")]
    TextLoadError { name: String },
}

pub type SceneResult = Result<(), SceneError>;

pub trait Scene<T> {
    fn activate(&mut self, _context: &mut T) -> SceneResult {
        Ok(())
    }
    fn deactivate(&mut self, _context: &mut T, _close: bool) -> SceneResult {
        Ok(())
    }
    fn update(&mut self, _context: &mut T) -> SceneResult {
        Ok(())
    }
    
    /// Periodically updates the scene with the given context and delta time.
    /// Indicates activity status with `_is_active`.
    ///
    /// # Arguments
    /// * `_context` - Mutable reference to the context for current state management.
    /// * `_delta_time` - The time elapsed since the last update.
    /// * `_is_active` - Boolean indicating if the scene is currently active.
    ///
    /// # Returns
    /// A `SceneResult` which is typically used to signal success or an error state.
    fn update_tick(&mut self, _context: &mut T, _delta_time: f32, _is_active: bool) -> SceneResult {
        Ok(())
    }

    fn draw(&mut self, context: &mut T) -> SceneResult;
}

pub type RenderScene = dyn Scene<RenderContext>;
