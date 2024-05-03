//! Predefined primitive UI elements & render helpers.

mod button;
mod combo;
mod context_menu;
mod input;
mod table;
mod window;

pub use self::{button::*, combo::*, context_menu::*, input::*, table::*, window::*};
pub use crate::colors::with_alpha;

use arcdps::imgui::{StyleStackToken, StyleVar, Ui};

/// Returns the width of the given number of "0" characters.
#[inline]
pub fn ch_width(ui: &Ui, count: usize) -> f32 {
    count as f32 * ui.calc_text_size("0")[0]
}

/// Enable small padding similar to ArcDPS and other plugins.
#[inline]
pub fn small_padding<'ui>(ui: &'ui Ui) -> StyleStackToken<'ui> {
    ui.push_style_var(StyleVar::FramePadding([1.0, 1.0]))
}
