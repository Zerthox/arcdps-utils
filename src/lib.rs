//! Utilities for interfacing with ArcDPS from Rust.

pub mod colors;
pub mod time;
pub mod tracking;
pub mod ui;
pub mod util;

#[cfg(feature = "settings")]
pub mod settings;

#[cfg(feature = "update")]
pub mod update;

#[deprecated]
pub mod api {
    // backwards comp
    pub use super::time::since_event as delta_time;
}
