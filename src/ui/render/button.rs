use arcdps::imgui::Ui;

/// Renders a reset button.
pub fn reset_button(ui: &Ui, label: impl AsRef<str>, confirm: &mut bool) -> bool {
    let mut changed = false;

    if !*confirm {
        if ui.button(label) {
            *confirm = true;
        }
    } else {
        ui.group(|| {
            ui.align_text_to_frame_padding();
            ui.text(format!("{}?", label.as_ref()));

            ui.same_line();
            if ui.button("Confirm") {
                changed = true;
                *confirm = false;
            }

            ui.same_line_with_spacing(0.0, 5.0);
            if ui.button("Cancel") {
                *confirm = false;
            }
        });
    }

    changed
}
