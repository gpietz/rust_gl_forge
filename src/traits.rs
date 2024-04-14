use anyhow::Result;

/// A trait for objects that can be updated with context-specific data.
///
/// The `Updatable` trait defines a single method, `update`, which should be
/// implemented by any type that needs to adapt or change its state according
/// to some external or internal factors provided at runtime. The generic
/// parameter `T` allows for flexible adaptation to different kinds of context
/// data, from simple numeric values like time deltas to complex structures
/// representing game states or environmental data.
///
/// Implementors of this trait can be used in systems where dynamic changes and
/// adaptations are necessary, such as game logic updaters, real-time data
/// processors, or simulation entities.
///
/// # Type Parameter
///
/// * `T` - The type of the context data used for updating the object. This could
///   include, but is not limited to, time steps, user inputs, or other real-time
///   data feeds.
///
/// # Examples
///
/// Implementing the `Updatable` trait for a game entity that needs to be updated
/// every frame:
///
/// ```no-run
/// struct GameEntity {
///     position: (f32, f32),
///     velocity: (f32, f32),
/// }
///
/// impl Updatable<f32> for GameEntity {
///     fn update(&mut self, delta_time: f32) -> Result<(), String> {
///         // Update the position based on the velocity and the elapsed time
///         self.position.0 += self.velocity.0 * delta_time;
///         self.position.1 += self.velocity.1 * delta_time;
///         Ok(())
///     }
/// }
/// ```
///
/// # Errors
///
/// The `update` method returns a `Result<(), E>`, where `E` is the error type that
/// should describe possible failure modes in the update process. Implementations
/// should define `E` to suit their error handling strategies, possibly providing
/// custom error types that cover specific scenarios like invalid input, out-of-
/// bounds values, or state inconsistencies.
///
/// It is essential that implementors of this trait handle errors robustly, ensuring
/// that errors provide meaningful feedback that can be used to diagnose and rectify
/// issues in the system using the `Updatable` trait.
pub trait Updatable<T> {
    /// Updates the state of the object based on the provided context.
    ///
    /// The `context` parameter typically represents a custom context relevant to the
    /// object's update process, which may include time information or other data necessary
    /// for the update. This method returns a `Result<(), E>` to handle potential errors
    /// during the update process.
    ///
    /// # Parameters
    ///
    /// - `context`: A context of type `T` providing necessary data for the update.
    ///
    /// # Errors
    ///
    /// This method should return an error if the object cannot be updated properly due
    /// to an issue specific to the context or the object's state.
    fn update(&mut self, context: T) -> Result<()>;
}
