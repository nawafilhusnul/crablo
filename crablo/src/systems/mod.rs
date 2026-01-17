//! Game systems module.
//!
//! This module contains all game systems:
//! - [`audio`] - Sound effects and music
//! - [`game_renderer`] - High-level game rendering
//! - [`pathfinding`] - A* and BFS pathfinding algorithms
//! - [`rendering`] - Low-level rendering primitives

pub mod audio;
pub mod game_renderer;
pub mod pathfinding;
pub mod rendering;

pub use audio::GameAudio;
