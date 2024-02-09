use super::*;
use crate::ui::{
    render::{ch_width, input_float_with_format},
    Hideable,
};
use arcdps::imgui::Ui;
use arcdps::{
    exports::{self, CoreColor},
    imgui::{InputTextFlags, StyleVar},
};

/// Hotkey to open window.
pub type WindowHotkey = Option<u32>;

/// Window options.
#[derive(Debug, Clone)]
pub struct WindowOptions {
    /// Whether the window is (currently) visible.
    pub visible: bool,

    /// Position of the window.
    pub position: WindowPosition,

    /// Width of the window in pixels.
    pub width: f32,

    /// Height of the window in pixels.
    pub height: f32,

    /// Whether to show the window titlebar.
    pub title_bar: bool,

    /// Whether to show the window body background.
    pub background: bool,

    /// Whether to show the window titlebar background.
    pub title_bar_background: bool,

    /// Whether to allow manually resizing the window.
    pub resize: bool,

    /// Whether to automatically resize the window according to contents.
    pub auto_resize: bool,

    /// Whether to allow scrolling.
    pub scroll: bool,

    /// Whether to show the scrollbar.
    pub scroll_bar: bool,

    /// Hotkey to open the window with.
    pub hotkey: WindowHotkey,
}

impl WindowOptions {
    /// Creates new window options.
    #[inline]
    pub fn new() -> Self {
        Self {
            visible: false,
            position: WindowPosition::default(),
            width: 0.0,
            height: 0.0,
            title_bar: true,
            background: true,
            title_bar_background: true,
            resize: true,
            auto_resize: false,
            scroll: true,
            scroll_bar: true,
            hotkey: None,
        }
    }

    /// Handles a key press event.
    ///
    /// Toggles visibility and returns `true` if the key matches the hotkey.
    #[inline]
    pub fn key_press(&mut self, key: usize) -> bool {
        if matches!(self.hotkey, Some(hotkey) if hotkey == key as u32) {
            self.toggle_visibility();
            true
        } else {
            false
        }
    }

    /// Updates the options with values from the [`Ui`]
    #[inline]
    pub fn update(&mut self, ui: &Ui) {
        // update window size values
        [self.width, self.height] = ui.window_size();
    }

    /// Renders the options menus for window style & position.
    pub fn render_options_menus(&mut self, ui: &Ui, pos: [f32; 2]) {
        let colors = exports::colors();
        let grey = colors
            .core(CoreColor::MediumGrey)
            .unwrap_or([0.5, 0.5, 0.5, 1.0]);

        ui.menu("Style", || {
            ui.text_colored(grey, "Window style");
            self.render_style_options(ui);
        });

        ui.menu("Position", || {
            ui.text_colored(grey, "Window position");
            self.position.render_select(ui, pos);
        });
    }

    /// Renders the window style options.
    pub fn render_style_options(&mut self, ui: &Ui) {
        let input_width = ch_width(ui, 12);

        ui.checkbox("Titlebar", &mut self.title_bar);
        ui.checkbox("Background", &mut self.background);
        ui.checkbox("Titlebar background", &mut self.title_bar_background);
        ui.checkbox("Scrollbar", &mut self.scroll_bar);
        ui.checkbox("Auto resize", &mut self.auto_resize);

        ui.set_next_item_width(input_width);

        let current = ui.clone_style().alpha;
        let _style = ui.push_style_var(StyleVar::Alpha(if self.auto_resize {
            0.3
        } else {
            current
        }));

        let flags = if self.auto_resize {
            InputTextFlags::READ_ONLY
        } else {
            InputTextFlags::empty()
        };

        input_float_with_format("Width", &mut self.width, STEP, STEP_FAST, FORMAT, flags);

        ui.set_next_item_width(input_width);
        input_float_with_format("Height", &mut self.height, STEP, STEP_FAST, FORMAT, flags);
    }
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl Hideable for WindowOptions {
    fn is_visible(&self) -> bool {
        self.visible
    }

    fn visible_mut(&mut self) -> &mut bool {
        &mut self.visible
    }
}
