use anyhow::Result;

/// The `Drawable` trait defines the interface for renderable objects.
///
/// This trait requires implementing the `draw` method, which handles
/// the rendering logic for the object. It returns a `Result<(), E>` to
/// allow for error handling in cases where drawing might fail due to
/// various reasons, such as OpenGL context issues or shader compilation errors.
///
/// # Examples
///
/// Implementing the `Drawable` trait for a custom `Square` struct:
///
/// ```no-run
/// struct Square {
///     // Square-specific fields
/// }
///
/// impl Drawable for Square {
///     fn draw(&self) -> Result<(), std::io::Error> {
///         // Implementation for drawing the square
///         Ok(())
///     }
/// }
/// ```
pub trait Drawable {
    /// Draws the object.
    ///
    /// This method should contain all the logic necessary to render the object
    /// on the screen. It returns a `Result<(), E>` where `E` is the error type.
    ///
    /// # Errors
    ///
    /// This method should return an error if any issues occur during the drawing
    /// process.
    fn draw(&self) -> Result<()>;
}

/// The `Updatable` trait defines the interface for objects that can be updated
/// over time. This is commonly used for objects whose state changes, such as
/// animations, game entities, or dynamic UI elements.
///
/// Implementors must define the `update` method, which progresses the state of
/// the object based on the elapsed time since the last update. This method also
/// returns a `Result<(), E>` for error handling.
///
/// # Examples
///
/// Implementing the `Updatable` trait for a `Particle` struct:
///
/// ```no-run
/// struct Particle {
///     // Particle-specific fields
/// }
///
/// impl Updatable for Particle {
///     fn update(&mut self, delta_time: f32) -> Result<(), std::io::Error> {
///         // Update particle position or other properties
///         Ok(())
///     }
/// }
/// ```
pub trait Updatable {
    /// Updates the state of the object.
    ///
    /// The `delta_time` parameter represents the amount of time (usually in seconds)
    /// that has passed since the last update. This method returns a `Result<(), E>`
    /// to handle potential errors during the update process.
    ///
    /// # Parameters
    ///
    /// - `delta_time`: The time elapsed since the last update, in seconds.
    ///
    /// # Errors
    ///
    /// This method should return an error if the object cannot be updated properly.
    fn update(&mut self, delta_time: f32) -> Result<()>;
}
