use anyhow::Result;

pub mod first_triangle;
pub mod indexed_quad;

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

    /// Renders the object.
    ///
    /// This method is called for each frame where the object needs to be drawn.
    /// Override this method with the specific drawing logic for the implementation.
    ///
    /// # Examples
    ///
    /// ```
    /// # struct MyObject;
    /// # impl Renderable for MyObject {
    /// fn draw(&mut self) {
    ///     // Drawing logic here
    /// }
    /// #     fn setup(&mut self) -> Result<()> { Ok(()) }
    /// # }
    /// # trait Renderable {
    /// #     fn setup(&mut self) -> Result<()> { Ok(()) }
    /// #     fn draw(&mut self);
    /// #     fn cleanup(&mut self) {}
    /// # }
    /// ```
    fn draw(&mut self);

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
    /// fn cleanup(&mut self) {
    ///     // Cleanup logic here
    /// }
    /// #     fn setup(&mut self) -> Result<()> { Ok(()) }
    /// #     fn draw(&mut self) {}
    /// # }
    /// # trait Renderable {
    /// #     fn setup(&mut self) -> Result<()> { Ok(()) }
    /// #     fn draw(&mut self);
    /// #     fn cleanup(&mut self) {}
    /// # }
    /// ```
    fn cleanup(&mut self) {}
}
