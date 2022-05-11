//! Helpers for UI alignment.

use arcdps::imgui::Ui;

/// Render state for left alignment.
#[derive(Debug, Clone, Copy)]
pub struct LeftAlign {
    first: bool,
    spacing: f32,
}

impl LeftAlign {
    /// Starts rendering items in left alignment.
    ///
    /// Items are passed from **left to right**.
    pub fn build() -> Self {
        Self::with_spacing(5.0)
    }

    /// Starts rendering items in left alignment with a custom spacing.
    ///
    /// Items are passed from **left to right**.
    pub fn with_spacing(spacing: f32) -> Self {
        Self {
            first: false,
            spacing,
        }
    }

    /// Renders the next item.
    ///
    /// Items are passed from **left to right**.
    pub fn item(&mut self, ui: &Ui, render: impl FnOnce()) {
        self.item_with_spacing(ui, self.spacing, render);
    }

    /// Renders the next item with a temporary spacing override.
    ///
    /// Items are passed from **left to right**.
    pub fn item_with_spacing(&mut self, ui: &Ui, spacing: f32, render: impl FnOnce()) {
        // prepare
        if self.first {
            self.first = false;
        } else {
            ui.same_line_with_spacing(0.0, spacing);
        }

        // render item
        render();
    }
}

/// Render state for right alignment.
#[derive(Debug, Clone, Copy)]
pub struct RightAlign {
    spacing: f32,
    accumulated: f32,
}

impl RightAlign {
    /// Starts rendering items in right alignment.
    ///
    /// Items are passed from **right to left**.
    pub fn build() -> Self {
        Self::with_spacing(5.0)
    }

    /// Starts rendering items in right alignment with a custom spacing.
    ///
    /// Items are passed from **right to left**.
    pub fn with_spacing(spacing: f32) -> Self {
        Self {
            spacing,
            accumulated: 0.0,
        }
    }

    /// Renders the next item.
    ///
    /// Items are passed from **right to left**.
    ///
    /// The item width will be used for alignment and updated with the correct width after render.
    /// It can be a guessed default on the first render.
    pub fn item(&mut self, ui: &Ui, item_width: &mut f32, render: impl FnOnce()) {
        self.item_with_spacing(ui, self.spacing, item_width, render)
    }

    /// Renders the next item with a temporary spacing override.
    ///
    /// Items are passed from **right to left**.
    ///
    /// The item width will be used for alignment and updated with the correct width after render.
    /// It can be a guessed default on the first render.
    pub fn item_with_spacing(
        &mut self,
        ui: &Ui,
        spacing: f32,
        item_width: &mut f32,
        render: impl FnOnce(),
    ) {
        // prepare alignment
        let [window_x, _] = ui.window_content_region_max();
        ui.same_line_with_pos(window_x - self.accumulated - *item_width);

        // render item
        render();

        // update item width & accumulated with actual size
        *item_width = ui.item_rect_size()[0];
        self.accumulated += *item_width + spacing;
    }
}
