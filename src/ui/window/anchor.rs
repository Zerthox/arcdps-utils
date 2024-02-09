use arcdps::imgui::Ui;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Screen anchor point for window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WindowAnchor {
    /// Anchored to top left of screen, growing down & right.
    TopLeft,

    /// Anchored to top right of screen, growing down & left.
    TopRight,

    /// Anchored to bottom left of screen, growing up & right.
    BottomLeft,

    /// Anchored to bottom right of screen, growing up & left.
    BottomRight,

    /// Anchored to center of screen, growing centered.
    Center,
}

impl WindowAnchor {
    /// Calculates the render position.
    pub fn calc(&self, ui: &Ui, x: f32, y: f32, window_size: [f32; 2]) -> [f32; 2] {
        let [screen_x, screen_y] = ui.io().display_size;
        let [window_x, window_y] = window_size;
        match self {
            Self::TopLeft => [x, y],
            Self::TopRight => [screen_x - window_x - x, y],
            Self::BottomLeft => [x, screen_y - window_y - y],
            Self::BottomRight => [screen_x - window_x - x, screen_y - window_y - y],
            Self::Center => [
                0.5 * screen_x - 0.5 * window_x + x,
                0.5 * screen_y - 0.5 * window_y + y,
            ],
        }
    }

    /// Renders a select for the window anchor.
    pub fn render_select(&mut self, ui: &Ui) {
        ui.group(|| {
            ui.radio_button("Top left", self, WindowAnchor::TopLeft);
            ui.radio_button("Top right", self, WindowAnchor::TopRight);
            ui.radio_button("Bottom left", self, WindowAnchor::BottomLeft);
            ui.radio_button("Bottom right", self, WindowAnchor::BottomRight);
            ui.radio_button("Center", self, WindowAnchor::Center);
        });
    }
}
