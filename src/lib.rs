//! Utilities for interfacing with ArcDPS from Rust.

pub mod api;
pub mod exports;
pub mod game;

/// Windows bindings.
pub mod win {
    pub use windows::*;
}
