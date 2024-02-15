use anyhow::Result;

pub mod first_triangle;
pub mod indexed_quad;
pub mod shader_triangle;
pub mod texture_triangle;
pub mod transformation;

//////////////////////////////////////////////////////////////////////////////
// - Renderable -
//////////////////////////////////////////////////////////////////////////////

/// The `Renderable` trait defines a set of behaviors for objects that can be rendered.
///
/// This trait encapsulates the essential functions required for rendering an object,
/// including setup, drawing, and cleanup. It's designed to be implemented by graphical
/// objects in a rendering pipeline.
///
/// # Examples
///
/// Implementing `Renderable` for a custom struct `MyObject`:
///
/// ```
/// struct MyObject {
///     // Object-specific fields
/// }
///
/// impl Renderable for MyObject {
///     fn setup(&mut self) -> Result<()> {
///         // Setup code specific to MyObject
///         Ok(())
///     }
///
///     fn draw(&mut self) {
///         // Drawing code specific to MyObject
///     }
///
///     fn cleanup(&mut self) {
///         // Cleanup code specific to MyObject
///     }
/// }
/// ```
pub trait Renderable {
    /// Sets up necessary resources and state for rendering.
    ///
    /// This method is called before an object is drawn for the first time or
    /// if an object needs to reinitialize its state or resources.
    ///
    /// The default implementation does nothing and returns `Ok(())`. Override
    /// this method in implementations that require specific setup logic.
    ///
    /// # Errors
    ///
    /// Returns an error if the setup fails. The nature of the error depends on
    /// the specifics of the implementation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// # struct MyObject;
    /// # impl Renderable for MyObject {
    /// fn setup(&mut self) -> Result<()> {
    ///     // Setup logic here
    ///     Ok(())
    /// }
    /// # }
    /// # trait Renderable {
    /// #     fn setup(&mut self) -> Result<()> { Ok(()) }
    /// #     fn draw(&mut self) {}
    /// #     fn cleanup(&mut self) {}
    /// # }
    /// ```
    fn setup(&mut self) -> Result<()> {
        Ok(())
    }

    /// Renders the object, considering the elapsed time since the last frame.
    ///
    /// This method should be called for each frame to draw the object, using `delta_time` to adjust for any
    /// animations or time-sensitive changes. Implement this method with the specific drawing logic
    /// for the object, taking into account the time passed to ensure smooth updates.
    ///
    /// # Parameters
    /// - `delta_time`: The time in seconds that has elapsed since the last frame. This parameter
    ///   is essential for creating smooth animations or movements by updating the object's state
    ///   based on the elapsed time.
    ///
    /// # Returns
    /// - `Result<(), Box<dyn std::error::Error>>`: A result indicating the success or failure of the drawing operation.
    ///   Returns `Ok(())` if the drawing operation succeeds, or an error if it fails.
    ///
    /// # Examples
    /// ```
    /// # use std::error::Error;
    /// # struct MyObject;
    /// # impl Renderable for MyObject {
    /// #     fn draw(&mut self, delta_time: f32) -> Result<(), Box<dyn Error>> {
    /// #         // Example drawing logic using delta_time
    /// #         println!("Drawing object with delta_time: {}", delta_time);
    /// #         Ok(())
    /// #     }
    /// # }
    /// # trait Renderable {
    /// #     fn draw(&mut self, delta_time: f32) -> Result<(), Box<dyn Error>>;
    /// # }
    /// ```
    fn draw(&mut self, delta_time: f32) -> Result<()>;

    /// Cleans up resources and state after rendering.
    ///
    /// This method is called when the object is being destroyed or if it needs to
    /// release resources. The default implementation does nothing. Override this
    /// method in implementations that require specific cleanup logic.
    ///
    /// # Examples
    ///
    /// ```
    /// # struct MyObject;
    /// # impl Renderable for MyObject {
    /// fn clean_up(&mut self) {
    ///     // Cleanup logic here
    /// }
    /// #     fn setup(&mut self) -> Result<()> { Ok(()) }
    /// #     fn draw(&mut self) {}
    /// # }
    /// # trait Renderable {
    /// #     fn setup(&mut self) -> Result<()> { Ok(()) }
    /// #     fn draw(&mut self);
    /// #     fn clean_up(&mut self) {}
    /// # }
    /// ```
    fn clean_up(&mut self) -> Result<()> {
        Ok(())
    }

    /// Switches between different rendering modes.
    fn toggle_mode(&mut self) {}

    /// Switches between different shapes.
    fn toggle_shape(&mut self) {}
}
