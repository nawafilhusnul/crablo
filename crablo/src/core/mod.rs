//! Core game module containing state management and configuration.
//!
//! This module provides:
//! - [`Game`] - Main game state and logic
//! - [`Player`] - Player state and abilities
//! - [`Database`] - Persistence layer
//! - [`constants`] - Game configuration constants
//! - [`traits`] - Common behavior traits

pub mod constants;
pub mod database;
pub mod game;
pub mod player;
pub mod shop;
pub mod traits;

pub use database::Database;
pub use game::Game;
