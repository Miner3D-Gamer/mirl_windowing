use std::collections::HashMap;

use mirl_buffer::Buffer;

/// # Features/Flag
/// `image` - Grands access to automatic texture lookup -> Define a filepath for a texture and lazy load it
///
/// `texture_manager_cleanup` - Grands accessability to `cleanup_unused`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextureManager<const AUTOMATIC_CLEANUP: bool> {
    /// The raw images in Buffer form
    pub textures: Vec<Option<Buffer>>,
    /// Map texture indexes to String
    pub lookup: HashMap<String, usize>,
    /// A list of empty spaces -> Images cannot be popped when removed as that would move their index
    pub free_list: Vec<usize>,
    #[cfg(feature = "image_support")]
    /// Map textures to files to lazy loading
    pub texture_lookup: HashMap<String, String>,
    /// A list of timestamps for when a texture has last been used
    pub last_used: Vec<u64>,
    /// 'Current' frame time for the textures to compare to
    pub current_frame: u64,
}

#[cfg(feature = "std")]
impl<const AUTOMATIC_CLEANUP: bool> TextureManager<AUTOMATIC_CLEANUP> {
    #[must_use]
    #[allow(clippy::new_without_default)]
    /// Create a texture manager -> Request textures for visual applications
    pub fn new() -> Self {
        Self {
            textures: Vec::new(),
            lookup: HashMap::new(),
            free_list: Vec::new(),
            #[cfg(feature = "image_support")]
            texture_lookup: HashMap::new(),

            last_used: Vec::new(),

            current_frame: 0,
        }
    }
    /// Registering a texture means being able to lazy load it upon request
    #[cfg(feature = "image_support")]
    pub fn register_texture(&mut self, name: String, file_path: String) {
        self.texture_lookup.insert(name, file_path);
    }
    #[must_use]
    /// Returns None if the requested image cannot be found
    pub fn get_from_idx(&self, index: usize) -> Option<&Buffer> {
        //
        // if index < self.last_used.len() {
        //     self.last_used[index] = self.current_frame;
        // }
        self.textures.get(index).and_then(|v| v.as_ref())
    }
    #[must_use]
    /// Get the raw image idx from the given name
    pub fn get_idx(&self, name: &String) -> Option<usize> {
        self.lookup.get(name).copied()
    }
    /// Get a texture
    /// Returns None if the requested image cannot be found
    pub fn get(&mut self, name: &str) -> Option<&Buffer> {
        if let Some(&index) = self.lookup.get(name) {
            if AUTOMATIC_CLEANUP && index < self.last_used.len() {
                self.last_used[index] = self.current_frame;
            }
            return self.textures[index].as_ref();
        }

        // First check if it's already loaded
        if let Some(&index) = self.lookup.get(name) {
            return self.textures[index].as_ref();
        }

        None
    }
    // #[cfg(feature = "image_support")]
    // /// Get a texture -> Enable 'image' feature for lazy loading
    // /// Returns None if the requested image cannot be found
    // ///
    // /// # Errors
    // /// When there was a problem with accessing the file
    // pub fn get_or_load<F: FileSystemTrait>(
    //     &mut self,
    //     name: &str,
    //     file_system: &F,
    //     remove_margins: bool,
    // ) -> Result<Option<&Buffer>, Box<dyn std::error::Error>> {
    //
    //     if let Some(&index) = self.lookup.get(name) {
    //         if index < self.last_used.len() {
    //             self.last_used[index] = self.current_frame;
    //         }
    //         return self.textures[index].as_ref();
    //     }

    //     // First check if it's already loaded
    //     if let Some(&index) = self.lookup.get(name) {
    //         return Ok(self.textures[index].as_ref());
    //     }

    //     #[cfg(feature = "image_support")]
    //     // If not loaded, try to load from file
    //     if let Some(file_path) = self.texture_lookup.get(name) {
    //         match self.load_texture_from_file(file_path, file_system) {
    //             Ok(mut buffer) => {
    //                 if remove_margins {
    //                     buffer.remove_margins();
    //                 }
    //                 self.insert_texture(name.to_string(), buffer);
    //                 if let Some(&index) = self.lookup.get(name) {
    //                     return Ok(self.textures[index].as_ref());
    //                 }
    //             }
    //             Err(e) => {
    //                 return Err(e);
    //             }
    //         }
    //     }

    //     Ok(None)
    // }
    // /// Load texture from file to memory
    // /// # Errors
    // /// When the file was not found
    // #[cfg(feature = "image_support")]
    // pub fn load_texture_from_file<F: FileSystemTrait>(
    //     &self,
    //     file_path: &str,
    //     file_system: &F,
    // ) -> Result<Buffer, Box<dyn core::error::Error>> {
    //     let file = file_system.get_file_contents(file_path)?;
    //     let img = file.to_image()?;
    //     Ok(img.into())
    // }
    /// Manually insert a texture with a corresponding name into cache
    pub fn insert_texture(&mut self, name: String, texture: Buffer) -> usize {
        let index = if let Some(free) = self.free_list.pop() {
            self.textures[free] = Some(texture);
            free
        } else {
            self.textures.push(Some(texture));
            self.textures.len() - 1
        };
        self.lookup.insert(name, index);
        index
    }
    /// Unloads/Deletes the specified image from cache if found
    pub fn unload_texture(&mut self, name: &str) {
        if let Some(&index) = self.lookup.get(name) {
            self.textures[index] = None;
            self.free_list.push(index);
            self.lookup.remove(name);
        }
    }
    /// Checks if a an image is already registered for lazy loading
    #[cfg(feature = "image_support")]
    #[must_use]
    pub fn is_texture_registered(&self, name: &str) -> bool {
        self.texture_lookup.contains_key(name)
    }
    // /// Preload registered image instead of letting it lazy load
    // ///
    // /// # Errors
    // /// When the file cannot be loaded
    // #[cfg(feature = "image_support")]
    // pub fn preload_texture<F: FileSystemTrait>(
    //     &mut self,
    //     name: &str,
    //     file_system: &F,
    // ) -> Result<(), Box<dyn core::error::Error>> {
    //     if !self.lookup.contains_key(name) {
    //         if let Some(file_path) = self.texture_lookup.get(name) {
    //             let buffer =
    //                 self.load_texture_from_file(file_path, file_system)?;
    //             self.insert_texture(name.to_string(), buffer);
    //         }
    //     }
    //     Ok(())
    // }
    /// Remove textures that haven't been used in X ticks -> Call `tick()` every frame for this to work properly
    /// Set to 0 if you only ever want the images you need in memory
    /// Setting it to at least 1 however is recommended
    #[allow(arithmetic_overflow)]
    pub fn cleanup_unused(&mut self, frames_unused: u64) {
        // This calculation is 100% wrong, future me; Fix it.
        let cutoff = self.current_frame.saturating_sub(frames_unused);

        // Collect names to remove (avoid borrowing issues)
        let to_remove: Vec<String> = self
            .lookup
            .iter()
            .filter(|&(_, &index)| index < self.last_used.len() && self.last_used[index] < cutoff)
            .map(|(name, _)| name.clone())
            .collect();

        for name in &to_remove {
            self.unload_texture(name);
        }
    }
    #[allow(arithmetic_overflow)]
    /// Tick the texture manager -> Only thing it does is increment a single value, required for `cleanup_unused()`
    pub const fn tick(&mut self) {
        if AUTOMATIC_CLEANUP {
            self.current_frame += 1;
        }
    }
}
