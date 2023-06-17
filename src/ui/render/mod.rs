//! Predefined primitive UI elements & render helpers.

mod button;
mod context_menu;
mod input;
mod table;

pub use self::button::*;
pub use self::context_menu::*;
pub use self::input::*;
pub use self::table::*;
pub use crate::colors::with_alpha;

use arcdps::imgui::{StyleStackToken, StyleVar, Ui};

/// Returns the width of the given number of "0" characters.
pub fn ch_width(ui: &Ui, count: usize) -> f32 {
    ui.calc_text_size("0".repeat(count))[0]
}

/// Enable small padding similar to ArcDPS and other plugins.
pub fn small_padding<'ui>(ui: &'ui Ui) -> StyleStackToken<'ui> {
    ui.push_style_var(StyleVar::FramePadding([1.0, 1.0]))
}
