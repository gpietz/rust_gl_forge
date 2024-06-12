use crate::opengl::texture::Texture;
use crate::opengl::texture_builder::TextureBuilder;
use crate::operation_status::OperationStatus;
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

//////////////////////////////////////////////////////////////////////////////
// - TextureManager -
//////////////////////////////////////////////////////////////////////////////

/// Manages textures within the application, handling storage, retrieval,
/// modification, and error tracking of texture data.
///
/// This struct serves as a central hub for managing various aspects of textures,
/// including their actual data, associated paths, specific flags that alter their
/// behavior, and any errors that may occur during texture processing.
///
/// # Fields
/// * `textures`: A hashmap that stores texture data against texture names.
/// * `texture_paths`: A hashmap that associates texture names with their file paths.
/// * `texture_error`: A hashmap that logs any errors related to specific textures.
/// * `texture_flags`: A hashmap that stores flags or properties affecting how textures
///   are rendered or processed.
///
/// # Usage
/// The `TextureManager` is typically used in graphical applications where managing
/// multiple textures is necessary. It provides methods to add, retrieve, and manage
/// textures efficiently.
///
/// # Note
/// The manager uses hashmaps for quick lookup, insertion, and removal of texture
/// related data, which ensures efficient management but requires handling potential
/// issues like hash collisions or memory overhead in applications with a very large
/// number of textures.
#[derive(Default)]
pub struct TextureManager {
    textures: HashMap<String, TextureData>,
    texture_paths: HashMap<String, String>,
    texture_error: HashMap<String, TextureError>,
    texture_flags: HashMap<String, TextureFlags>,
}

impl TextureManager {
    /// Adds a new texture path to the map if it does not already exist.
    ///
    /// This function adds the specified path `texture_path` under the key `name` to
    /// the `texture_paths` map, provided the key does not already exist and the file
    /// path is valid.
    ///
    /// # Arguments
    /// * `name` - A `String` specifying the name of the key for the texture.
    /// * `texture_path` - A `String` indicating the path to the texture file.
    ///
    /// # Returns
    /// An `OperationStatus<TextureError>` indicating the success or failure of the
    /// operation. Returns an error if the key already exists or the file is not found.
    ///
    /// # Examples
    /// ```no-run
    /// let mut texture_manager = TextureManager::new();
    /// let name = "example_texture".to_string();
    /// let path = "path/to/texture.png".to_string();
    /// let result = texture_manager.add_path(name, path);
    /// match result {
    ///     OperationStatus::Success(_) => println!("Texture added successfully!"),
    ///     OperationStatus::Error(e) => println!("Failed to add texture: {:?}", e),
    /// }
    /// ```
    pub fn add_path(&mut self, name: &str, texture_path: &str) -> OperationStatus<TextureError> {
        let name = name.to_string();

        // Check if key is already present
        if self.textures.contains_key(&name) {
            return OperationStatus::new_error(TextureError::KeyExists {
                key_name: name,
            });
        }

        // Check if file is existing
        if let Some(texture_error) = Self::check_file_exists(texture_path) {
            self.texture_error.insert(name, texture_error.clone());
            return OperationStatus::new_error(texture_error);
        }

        // Add texture path into map
        self.texture_paths.insert(name, texture_path.to_string());
        OperationStatus::new_success()
    }

    /// Adds or updates a texture path in the map.
    ///
    /// This function inserts or updates the specified `texture_path` under the key
    /// `name` in the `texture_paths` map. If the key already exists, it will replace
    /// the existing path with the new one. If the path does not exist, the function
    /// first checks if the file at `texture_path` exists and returns an error if it
    /// does not.
    ///
    /// # Arguments
    /// * `name` - A `String` specifying the key name under which the texture path
    ///   is stored.
    /// * `texture_path` - A `String` indicating the file path of the texture.
    ///
    /// # Returns
    /// An `OperationStatus<TextureError>` indicating the success or failure of the
    /// operation. Returns an error if the file does not exist; otherwise, it returns
    /// success indicating the path was added or updated successfully.
    ///
    /// # Examples
    /// ```no-run
    /// let mut texture_manager = TextureManager::new();
    /// let name = "example_texture".to_string();
    /// let path = "path/to/texture.png".to_string();
    /// let result = texture_manager.add_or_update_path(name, path);
    /// match result {
    ///     OperationStatus::Success(_) => println!("Texture path updated successfully!"),
    ///     OperationStatus::Error(e) => println!("Failed to update texture path: {:?}", e),
    /// }
    /// ```
    pub fn add_or_update_path(
        &mut self,
        name: &str,
        texture_path: &str,
    ) -> OperationStatus<TextureError> {
        let name = name.to_string();

        // Check if file is existing
        if let Some(texture_error) = Self::check_file_exists(texture_path) {
            self.texture_error.insert(name.clone(), texture_error.clone());
            return OperationStatus::new_error(texture_error);
        }

        // Add texture path to map where it'll be replaced if it exists already
        self.texture_paths.insert(name, texture_path.to_string());
        OperationStatus::new_success()
    }

    /// Checks if a file exists at the specified texture path.
    ///
    /// This function verifies the existence of a file at the given `texture_path`.
    /// It returns an `Option<TextureError>` based on the file's presence. If the file
    /// does not exist, it returns `Some(TextureError::FileNotFound)`, otherwise it
    /// returns `None` indicating that the file exists and no error occurred.
    ///
    /// # Arguments
    /// * `texture_path` - A string slice (`&str`) representing the path to the
    ///   texture file.
    ///
    /// # Returns
    /// An `Option<TextureError>` which is `None` if the file exists or contains a
    /// `TextureError::FileNotFound` if the file does not exist.
    fn check_file_exists(texture_path: &str) -> Option<TextureError> {
        let path = Path::new(&texture_path);
        if path.exists() {
            None
        } else {
            Some(TextureError::FileNotFound)
        }
    }

    /// Retrieves the texture error associated with a specific key, if any.
    ///
    /// This method returns a reference to a `TextureError` if there is an error
    /// associated with the given key `name` in the `texture_error` map. It provides
    /// an easy way to check for errors related to specific texture operations that
    /// have previously been attempted and failed.
    ///
    /// # Arguments
    /// * `name` - A string slice (`&str`) that represents the key name associated
    ///   with the texture error.
    ///
    /// # Returns
    /// An `Option<&TextureError>` which is `Some` if an error exists for the given
    /// key, or `None` if there is no error associated.
    pub fn texture_error(&self, name: &str) -> Option<&TextureError> {
        self.texture_error.get(name)
    }

    /// Checks if there is an error associated with a specific texture key.
    ///
    /// This method determines whether there is an error recorded for a given key
    /// `name` in the `texture_error` map. It returns true if an error exists, thus
    /// providing a quick way to verify error presence without retrieving the error
    /// itself.
    ///
    /// # Arguments
    /// * `name` - A string slice (`&str`) representing the key name to check for
    ///   errors.
    ///
    /// # Returns
    /// A `bool` that is `true` if an error is associated with the key, and `false`
    /// otherwise.
    pub fn has_error(&self, name: &str) -> bool {
        self.texture_error.contains_key(name)
    }

    /// Clears any texture error associated with the specified key.
    ///
    /// This method removes an error associated with the given key `name` from the
    /// `texture_error` map. It is useful for resetting the error state of a texture
    /// after the error has been handled or resolved, maintaining clean state management.
    ///
    /// # Arguments
    /// * `name` - A string slice (`&str`) that represents the key name from which
    ///   the error should be removed.
    pub fn clear_error(&mut self, name: &str) {
        self.texture_error.remove(name);
    }

    /// Adds or updates flags associated with a specific texture in the texture
    /// manager. This method allows for setting or modifying the properties and
    /// behavior of textures within the application.
    ///
    /// # Arguments
    /// * `name` - A string slice representing the name of the texture to which the
    ///   flags will be associated.
    /// * `flags` - The `TextureFlags` instance containing the settings or properties
    ///   to be applied to the texture.
    ///
    /// # Behavior
    /// This method inserts new flags for the texture if they do not already exist,
    /// or updates the existing flags if the texture is already present in the map.
    /// There is no return value, and previous flags (if any) are overwritten.
    ///
    /// # Use Cases
    /// Use this method when initializing textures or when texture properties need
    /// to be changed dynamically during runtime. This is particularly useful in
    /// graphics applications where texture behavior needs to be adjusted based on
    /// different rendering contexts or conditions.
    ///
    /// # Note
    /// This method will overwrite any existing flags for the given texture without
    /// warning. If preservation of existing flags is required, consider retrieving,
    /// modifying, and then re-setting the flags.
    pub fn add_texture_flags(&mut self, name: &str, flags: TextureFlags) {
        self.texture_flags.insert(name.to_string(), flags);
    }

    /// Retrieves the flags associated with a specified texture. These flags
    /// might include settings or properties that affect how the texture is
    /// used or rendered in the application.
    ///
    /// # Arguments
    /// * `name` - A string slice representing the name of the texture whose flags
    ///   are being queried.
    ///
    /// # Returns
    /// An `Option<&TextureFlags>`:
    /// - `Some(&TextureFlags)` if flags exist for the named texture.
    /// - `None` if no flags are found for that texture.
    ///
    /// # Usage
    /// This method is primarily used to check and manipulate rendering options or
    /// other properties associated with a texture before it is used in rendering
    /// or other processing tasks. It allows safe, read-only access to the texture's
    /// flags, ensuring that the application can make decisions based on the current
    /// properties without altering them directly.
    ///
    /// # Note
    /// This method provides a non-mutable reference to the texture flags, if they
    /// exist. To modify these flags, other methods in `TextureManager` should be
    /// used that handle mutable access or updates to texture properties.
    pub fn get_texture_flags(&self, name: &str) -> Option<&TextureFlags> {
        self.texture_flags.get(name)
    }

    /// Removes the flags associated with a specific texture from the internal
    /// storage. This is typically used when a texture is no longer needed or
    /// its flags are to be reset.
    ///
    /// # Arguments
    /// * `name` - A string slice representing the name of the texture whose flags
    ///   are to be cleared.
    ///
    /// # Behavior
    /// If the specified texture's name exists in the `texture_flags` map, this
    /// method removes the entry. If no such entry exists, the method does nothing,
    /// performing a no-op.
    ///
    /// This operation does not affect the texture itself, only the flags associated
    /// with it in the `texture_flags` map. This method does not return any value
    /// or provide confirmation of removal. It silently fails if the texture name
    /// is not found.
    ///
    /// # Use Cases
    /// This method is useful in scenarios where texture settings need to be
    /// refreshed or completely removed, such as when textures are reloaded with
    /// different parameters or when cleaning up resources that are no longer used.
    pub fn clear_texture_flags(&mut self, name: &str) {
        self.texture_flags.remove(name);
    }

    /// Retrieves a texture by name, cloning it for safe independent usage.
    ///
    /// This function checks if a texture already exists in the cache; if so, it
    /// returns a cloned version. If not found, it attempts to load the texture
    /// from a predefined path, cache it, and then returns a clone.
    ///
    /// # Arguments
    /// * `name` - The name of the texture to retrieve.
    ///
    /// # Returns
    /// A `Result` containing either:
    /// - A cloned `Texture` instance if successful.
    /// - A `TextureError` if the texture cannot be found, loaded, or cloned.
    ///
    /// # Errors
    /// This function can return `TextureError::KeyNotExisting` if no texture
    /// or path is registered under the provided name.
    /// It may also return `TextureError::CloneFailure` if the cloning process fails,
    /// or `TextureError::FindFailed` if the texture could not be retrieved post-insertion.
    ///
    /// # Examples
    /// ```no-run
    /// use shared_lib::gl_texture::TextureManager;
    ///
    /// let mut texture_manager = TextureManager::new();
    /// match texture_manager.get_texture("example_texture") {
    ///     Ok(texture) => println!("Texture cloned successfully."),
    ///     Err(e) => eprintln!("Failed to retrieve or clone the texture: {}", e),
    /// }
    /// ```
    ///
    /// # Implementation Details
    /// The function first checks for the existence of the texture in the internal
    /// cache. If found, it clones this texture to ensure that modifications to
    /// the returned texture do not affect the cached version. If the texture is
    /// not found, it checks for a registered path and attempts to load and cache
    /// the texture. A freshly loaded texture is then cloned before being returned.
    /// If insertion of a new texture succeeds but retrieval fails, it handles this
    /// edge case by returning a `FindFailed` error.
    pub fn get_texture(&mut self, name: &str) -> anyhow::Result<Texture, TextureError> {
        // Attempt for retrieve and clone an existing texture
        if let Some(texture_data) = self.textures.get(name) {
            return get_cloned_texture(texture_data);
        }

        // If the texture isn't loaded, and no path is registered, return an error
        if !self.texture_paths.contains_key(name) {
            return Err(TextureError::KeyNotExisting {
                key_name: name.to_string(),
            });
        }

        // Create, insert, and directly clone the new texture
        let texture = self.create_texture(name)?;
        self.textures.insert(name.to_string(), TextureData::new(texture));

        // Assuming insertion is successful and the texture is now available
        return self
            .textures
            .get(name)
            .map(get_cloned_texture)
            .unwrap_or_else(|| Err(TextureError::FindFailed));

        // Helper function to clone a texture
        fn get_cloned_texture(texture_data: &TextureData) -> anyhow::Result<Texture, TextureError> {
            match texture_data.texture.clone_as_non_owner() {
                Ok(cloned_texture) => Ok(cloned_texture),
                Err(e) => Err(TextureError::CloneFailure {
                    message: e.to_string(),
                }),
            }
        }
    }

    /// Retrieves a list of textures based on provided names and attempts to clone
    /// each texture as a non-owner. This function is part of the `TextureManager`
    /// which handles the retrieval and cloning of texture resources.
    ///
    /// # Arguments
    /// * `texture_names` - A slice of string slices that represent the names of
    ///   the textures to be retrieved and cloned.
    ///
    /// # Returns
    /// A `Result` containing either:
    /// - A vector of `TextureResult` objects, each representing the outcome of
    ///   the texture retrieval and cloning process.
    /// - An error, encapsulated within a `TextureError`, that occurred during
    ///   texture retrieval or cloning.
    ///
    /// # Detailed Behavior
    /// The function iterates over each name provided in `texture_names`. For each
    /// name, it attempts to:
    /// 1. Retrieve the texture from a managed store.
    /// 2. Clone the texture as a non-owner.
    ///
    /// If both retrieval and cloning are successful, a `TextureResult::success` is
    /// pushed to the result vector. If an error occurs during the cloning process,
    /// a `TextureResult::failure` with a `CloneFailure` error is pushed instead.
    /// Similarly, if the initial retrieval fails, a `TextureResult::failure` with
    /// the corresponding error is added to the results.
    ///
    /// # Example
    /// ```no-run
    /// let mut texture_manager = TextureManager::new();
    /// let texture_names = ["texture1", "texture2"];
    /// let results = texture_manager.get_textures(&texture_names);
    /// match results {
    ///     Ok(texture_results) => {
    ///         for texture_result in texture_results {
    ///             println!("{:?}", texture_result);
    ///         }
    ///     },
    ///     Err(e) => eprintln!("Failed to retrieve or clone textures: {}", e),
    /// }
    /// ```
    pub fn get_textures(&mut self, texture_names: &[&str]) -> anyhow::Result<Vec<TextureResult>> {
        let mut texture_results = Vec::new();
        for &texture_name in texture_names {
            match self.get_texture(texture_name) {
                Ok(texture) => match texture.clone_as_non_owner() {
                    Ok(cloned_texture) => {
                        texture_results
                            .push(TextureResult::success(texture_name.to_string(), cloned_texture));
                    }
                    Err(clone_error) => {
                        texture_results.push(TextureResult::failure(
                            texture_name.to_string(),
                            TextureError::CloneFailure {
                                message: clone_error.to_string(),
                            },
                        ));
                    }
                },
                Err(texture_error) => {
                    texture_results
                        .push(TextureResult::failure(texture_name.to_string(), texture_error));
                }
            }
        }

        Ok(texture_results)
    }

    /// Creates a texture based on a specified name by using associated settings.
    ///
    /// This method attempts to create a texture for a given `name` using the path
    /// and flags stored in the `texture_paths` and `texture_flags` maps, respectively.
    /// If the texture path exists, it builds the texture with optional settings like
    /// alpha transparency and vertical flipping according to the flags. If any part
    /// of the texture creation process fails, it returns an error.
    ///
    /// # Arguments
    /// * `name` - A string slice (`&str`) representing the name of the texture to be
    ///   created.
    ///
    /// # Returns
    /// A `Result` that either contains the created `Texture` or a `TextureError` if
    /// an error occurs during texture creation or if the key does not exist.
    ///
    /// # Examples
    /// ```no-run
    /// use shared_lib::gl_texture::TextureManager;
    ///
    /// let texture_manager = TextureManager::new();
    /// match texture_manager.create_texture("example_texture") {
    ///     Ok(texture) => println!("Texture created successfully."),
    ///     Err(e) => println!("Failed to create texture: {:?}", e),
    /// }
    /// ```
    fn create_texture(&self, name: &str) -> anyhow::Result<Texture, TextureError> {
        if let Some(texture_path) = self.texture_paths.get(name) {
            let texture_flags = match self.texture_flags.get(name) {
                Some(flags) => flags.clone(),
                None => TextureFlags::default(),
            };
            TextureBuilder::default()
                .path(texture_path)
                .has_alpha(texture_flags.has_alpha)
                .flip_vertical(texture_flags.flip_vertically)
                .build()
                .map_err(|e| {
                    eprintln!("Failed creating texture: {:?}", e);
                    TextureError::CreateTextureFailure {
                        message: e.to_string(),
                    }
                })
        } else {
            Err(TextureError::KeyNotExisting {
                key_name: name.to_string(),
            })
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// - TextureData -
//////////////////////////////////////////////////////////////////////////////

struct TextureData {
    pub(crate) texture: Texture,
    pub(crate) description: Option<TextureDescriptor>,
}

impl TextureData {
    pub fn new(texture: Texture) -> Self {
        Self {
            texture,
            description: None,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// - TextureError -
//////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Error)]
pub enum TextureError {
    #[error("A texture with that key exists already: {key_name}")]
    KeyExists {
        key_name: String,
    },
    #[error("No key with the name found: {key_name}")]
    KeyNotExisting {
        key_name: String,
    },
    #[error("File has not been found")]
    FileNotFound,
    #[error("Failed to create texture: {message}")]
    CreateTextureFailure {
        message: String,
    },
    #[error("Failed to find a previously created texture")]
    FindFailed,
    #[error("Failed to clone a texture: {message}")]
    CloneFailure {
        message: String,
    },
}

//////////////////////////////////////////////////////////////////////////////
// - TextureFlags -
//////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug, Clone)]
pub struct TextureFlags {
    pub has_alpha: bool,
    pub flip_vertically: bool,
}

//////////////////////////////////////////////////////////////////////////////
// - TextureResult -
//////////////////////////////////////////////////////////////////////////////
pub struct TextureResult {
    name: String,
    success: bool,
    texture: Option<Texture>,
    error: Option<TextureError>,
}

impl TextureResult {
    pub(crate) fn success(name: String, texture: Texture) -> Self {
        Self {
            name,
            success: true,
            texture: Some(texture),
            error: None,
        }
    }

    pub(crate) fn failure(name: String, error: TextureError) -> Self {
        Self {
            name,
            success: false,
            texture: None,
            error: Some(error),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn texture(&self) -> &Option<Texture> {
        &self.texture
    }

    pub fn error(&self) -> &Option<TextureError> {
        &self.error
    }

    pub fn is_success(&self) -> bool {
        self.success
    }

    pub fn is_failure(&self) -> bool {
        !self.success
    }
}

//////////////////////////////////////////////////////////////////////////////
// - TextureDescriptor -
//////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TextureDescriptor {
    pub path: String,
    pub flip_vertically: bool,
    pub flip_horizontally: bool,
}

impl TextureDescriptor {
    fn new(path: String) -> Self {
        Self {
            path,
            ..Default::default()
        }
    }
}
