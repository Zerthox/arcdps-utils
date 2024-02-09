use super::*;
use arcdps::imgui::Ui;

pub(crate) const STEP: f32 = 1.0;

pub(crate) const STEP_FAST: f32 = 10.0;

pub(crate) const FORMAT: &str = "%.0f";

/// Renders menus with window options.
pub fn window_options_menus(ui: &Ui, options: &mut WindowOptions, pos: [f32; 2]) {
    options.render_options_menus(ui, pos)
}

/// Renders window style options.
pub fn window_style_options(ui: &Ui, options: &mut WindowOptions) {
    options.render_style_options(ui)
}

/// Renders window position select.
pub fn window_position_select(ui: &Ui, position: &mut WindowPosition, pos: [f32; 2]) {
    position.render_select(ui, pos)
}
