//! Utilities for interfacing with ArcDPS from Rust.

pub mod api;
pub mod exports;
pub mod game;
pub mod ui;

/// Windows bindings.
pub mod win {
    pub use windows::*;
}
