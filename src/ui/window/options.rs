use arcdps::imgui::Ui;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::ui::Hideable;

/// Window options.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct WindowOptions {
    pub visible: bool,
    pub position: WindowPosition,
    pub width: f32,
    pub height: f32,
    pub title_bar: bool,
    pub background: bool,
    pub title_bar_background: bool,
    pub resize: bool,
    pub auto_resize: bool,
    pub scroll: bool,
    pub scroll_bar: bool,
    pub hotkey: Option<WindowHotkey>,
}

impl WindowOptions {
    /// Creates new window options.
    #[inline]
    pub fn new() -> Self {
        Self {
            visible: true,
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

/// Window position onscreen.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WindowPosition {
    Manual,
    Anchored {
        anchor: WindowAnchor,
        x: f32,
        y: f32,
    },
}

impl Default for WindowPosition {
    fn default() -> Self {
        Self::Manual
    }
}

impl WindowPosition {
    /// Calculates the render position.
    pub fn calc(&self, ui: &Ui, window_size: [f32; 2]) -> Option<[f32; 2]> {
        match self {
            Self::Manual => None,
            Self::Anchored { anchor, x, y } => {
                let [screen_x, screen_y] = ui.io().display_size;
                let [window_x, window_y] = window_size;
                let rel_x = *x;
                let rel_y = *y;

                Some(match anchor {
                    WindowAnchor::TopLeft => [rel_x, rel_y],
                    WindowAnchor::TopRight => [screen_x - window_x - rel_x, rel_y],
                    WindowAnchor::BottomLeft => [rel_x, screen_y - window_y - rel_y],
                    WindowAnchor::BottomRight => {
                        [screen_x - window_x - rel_x, screen_y - window_y - rel_y]
                    }
                })
            }
        }
    }
}

/// Screen anchor point for window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WindowAnchor {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// Hotkey to open window.
pub type WindowHotkey = u32;
