use super::*;
use crate::ui::render::{ch_width, input_float_with_format};
use arcdps::imgui::{InputTextFlags, Ui};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
            Self::Anchored { anchor, x, y } => Some(anchor.calc(ui, *x, *y, window_size)),
        }
    }

    /// Renders a select for the window position.
    pub fn render_select(&mut self, ui: &Ui, pos: [f32; 2]) {
        if ui.radio_button_bool("Manual", matches!(self, WindowPosition::Manual)) {
            *self = WindowPosition::Manual;
        }

        if ui.radio_button_bool(
            "Screen relative",
            matches!(self, WindowPosition::Anchored { .. }),
        ) {
            let [x, y] = pos;
            *self = WindowPosition::Anchored {
                anchor: WindowAnchor::TopLeft,
                x,
                y,
            }
        }

        // details for anchored
        if let WindowPosition::Anchored { anchor, x, y } = self {
            let input_width = ch_width(ui, 12);
            ui.indent();

            anchor.render_select(ui);

            ui.set_next_item_width(input_width);
            input_float_with_format("x", x, STEP, STEP_FAST, FORMAT, InputTextFlags::empty());

            ui.set_next_item_width(input_width);
            input_float_with_format("y", y, STEP, STEP_FAST, FORMAT, InputTextFlags::empty());

            ui.unindent();
        }
    }
}
