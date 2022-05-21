use super::*;
use crate::{
    api::CoreColor,
    exports,
    ui::{
        render::{ch_width, input_float_with_format},
        Ui,
    },
};
use arcdps::imgui::{InputTextFlags, StyleVar};

/// Renders menus with window options.
pub fn window_options_menus(ui: &Ui, options: &mut WindowOptions) {
    let colors = exports::colors();
    let grey = colors
        .core(CoreColor::MediumGrey)
        .unwrap_or([0.5, 0.5, 0.5, 1.0]);

    let input_width = ch_width(ui, 12);
    const STEP: f32 = 1.0;
    const STEP_FAST: f32 = 10.0;
    const FORMAT: &str = "%.0f";

    ui.menu("Style", || {
        ui.text_colored(grey, "Window style");

        ui.checkbox("Titlebar", &mut options.title_bar);
        ui.checkbox("Background", &mut options.background);
        ui.checkbox("Scrollbar", &mut options.scroll_bar);
        ui.checkbox("Auto resize", &mut options.auto_resize);

        ui.set_next_item_width(input_width);

        let current = ui.clone_style().alpha;
        let _style = ui.push_style_var(StyleVar::Alpha(if options.auto_resize {
            0.3
        } else {
            current
        }));

        let flags = if options.auto_resize {
            InputTextFlags::READ_ONLY
        } else {
            InputTextFlags::empty()
        };

        input_float_with_format("Width", &mut options.width, STEP, STEP_FAST, FORMAT, flags);

        ui.set_next_item_width(input_width);
        input_float_with_format(
            "Height",
            &mut options.height,
            STEP,
            STEP_FAST,
            FORMAT,
            flags,
        );
    });

    ui.menu("Position", || {
        ui.text_colored(grey, "Window position");

        if ui.radio_button_bool("Manual", options.position == WindowPosition::Manual) {
            options.position = WindowPosition::Manual;
        }

        if ui.radio_button_bool(
            "Screen relative",
            matches!(options.position, WindowPosition::Anchored { .. }),
        ) {
            options.position = WindowPosition::Anchored {
                anchor: WindowAnchor::TopLeft,
                x: 0.0,
                y: 0.0,
            }
        }

        if let WindowPosition::Anchored { anchor, x, y } = &mut options.position {
            ui.indent();

            ui.radio_button("Top left", anchor, WindowAnchor::TopLeft);
            ui.radio_button("Top right", anchor, WindowAnchor::TopRight);
            ui.radio_button("Bottom left", anchor, WindowAnchor::BottomLeft);
            ui.radio_button("Bottom right", anchor, WindowAnchor::BottomRight);

            ui.set_next_item_width(input_width);
            input_float_with_format("x", x, STEP, STEP_FAST, FORMAT, InputTextFlags::empty());

            ui.set_next_item_width(input_width);
            input_float_with_format("y", y, STEP, STEP_FAST, FORMAT, InputTextFlags::empty());

            ui.unindent();
        }
    });
}
