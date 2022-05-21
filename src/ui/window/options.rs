use arcdps::imgui::Ui;
use serde::{Deserialize, Serialize};

/// Window options.
#[derive(Debug, Clone)]
pub struct WindowOptions {
    pub name: String,
    pub visible: bool,
    pub position: WindowPosition,
    pub width: f32,
    pub height: f32,
    pub title_bar: bool,
    pub background: bool,
    pub resize: bool,
    pub auto_resize: bool,
    pub scroll: bool,
    pub scroll_bar: bool,
}

impl WindowOptions {
    /// Creates new window options.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            visible: true,
            position: WindowPosition::Manual,
            width: 0.0,
            height: 0.0,
            title_bar: true,
            background: true,
            resize: true,
            auto_resize: false,
            scroll: true,
            scroll_bar: true,
        }
    }
}

/// Window position onscreen.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "settings", derive(Serialize, Deserialize))]
pub enum WindowPosition {
    Manual,
    Anchored {
        anchor: WindowAnchor,
        x: f32,
        y: f32,
    },
}

impl WindowPosition {
    /// Calculates the UI position.
    pub fn calc(&self, ui: &Ui, window_size: [f32; 2]) -> [f32; 2] {
        match self {
            Self::Manual => [0.0, 0.0],
            Self::Anchored { anchor, x, y } => {
                let [screen_x, screen_y] = ui.io().display_size;
                let [window_x, window_y] = window_size;
                let rel_x = *x;
                let rel_y = *y;

                match anchor {
                    WindowAnchor::TopLeft => [rel_x, rel_y],
                    WindowAnchor::TopRight => [screen_x - window_x - rel_x, rel_y],
                    WindowAnchor::BottomLeft => [rel_x, screen_y - window_y - rel_y],
                    WindowAnchor::BottomRight => {
                        [screen_x - window_x - rel_x, screen_y - window_y - rel_y]
                    }
                }
            }
        }
    }
}

/// Screen anchor point for window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "settings", derive(Serialize, Deserialize))]
pub enum WindowAnchor {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}
