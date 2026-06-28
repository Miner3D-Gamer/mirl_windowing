//! A lib for creating(/managing) windows
//!
//! To get started, use either the `minifb`, `glfw`, or experimentally `console` flags
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(const_clone)]

/// Most 2d games have 60 tps while 3d games often have 20 tps
pub mod ticker;

/// Window related stuff
pub mod windowing;

/// All the stuff you'd need from the lib usually
pub mod prelude;

/// A simple texture manager to dynamically load/unload images
pub mod texture_manager;

// Windows
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "glfw")]
/// The glfw version of the backend
pub mod glfw;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "minifb")]
/// The minifb version of the backend
pub mod minifb;

#[cfg(feature = "console")]
/// The console version of the backend
pub mod console;
