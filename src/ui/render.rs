//! Predefined primitive UI elements & render helpers.

use crate::util::{keycode_to_name, name_to_keycode};
use arcdps::imgui::{sys, InputTextFlags, StyleStackToken, StyleVar, Ui};
use std::ffi::CString;

/// Returns the width of the given number of "0" characters.
pub fn ch_width(ui: &Ui, count: usize) -> f32 {
    ui.calc_text_size("0".repeat(count))[0]
}

/// Adjusts the alpha value of a color.
pub fn with_alpha(mut color: [f32; 4], alpha: f32) -> [f32; 4] {
    color[3] = alpha;
    color
}

/// Enable small padding similar to ArcDPS and other plugins.
pub fn small_padding<'ui>(ui: &'ui Ui) -> StyleStackToken<'ui> {
    ui.push_style_var(StyleVar::FramePadding([1.0, 1.0]))
}

/// Renders a reset button.
pub fn reset_button(ui: &Ui, label: impl AsRef<str>, confirm: &mut bool) -> bool {
    let mut changed = false;

    if !*confirm {
        if ui.button(label) {
            *confirm = true;
        }
    } else {
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
    }

    changed
}

/// Renders a right-click context menu for the last item.
pub fn item_context_menu(str_id: impl Into<String>, contents: impl FnOnce()) {
    if let Ok(str_id) = CString::new(str_id.into()) {
        if unsafe {
            sys::igBeginPopupContextItem(
                str_id.as_ptr(),
                sys::ImGuiPopupFlags_MouseButtonRight as i32,
            )
        } {
            contents();
            unsafe { sys::igEndPopup() };
        }
    }
}

/// Renders a right-click context menu for the window.
pub fn window_context_menu(str_id: impl Into<String>, contents: impl FnOnce()) {
    if let Ok(str_id) = CString::new(str_id.into()) {
        if unsafe {
            sys::igBeginPopupContextWindow(
                str_id.as_ptr(),
                sys::ImGuiPopupFlags_MouseButtonRight as i32,
            )
        } {
            contents();
            unsafe { sys::igEndPopup() };
        }
    }
}

/// Renders a float input with a custom format.
pub fn input_float_with_format(
    label: impl Into<String>,
    value: &mut f32,
    step: f32,
    step_fast: f32,
    format: impl Into<String>,
    flags: InputTextFlags,
) -> bool {
    if let (Ok(label), Ok(format)) = (CString::new(label.into()), CString::new(format.into())) {
        unsafe {
            sys::igInputFloat(
                label.as_ptr(),
                value as *mut f32,
                step,
                step_fast,
                format.as_ptr(),
                flags.bits() as i32,
            )
        }
    } else {
        false
    }
}

/// Renders a custom key input.
pub fn input_key(ui: &Ui, id: impl AsRef<str>, label: impl AsRef<str>, keycode: &mut Option<u32>) {
    const SPACING: f32 = 5.0;

    ui.align_text_to_frame_padding();
    ui.text(label);

    let mut buffer = String::with_capacity(3);
    if let Some(keycode) = keycode {
        buffer.push_str(&keycode.to_string());
    }
    ui.same_line_with_spacing(0.0, SPACING);
    ui.set_next_item_width(ch_width(ui, 4));
    if ui
        .input_text(id, &mut buffer)
        .chars_uppercase(true)
        .chars_noblank(true)
        .build()
    {
        match buffer.len() {
            1 => {
                // read entered key name
                *keycode = Some(name_to_keycode(buffer.as_bytes()[0]));
            }
            2 | 3 => {
                // read entered keycode
                *keycode = buffer.parse().ok();
            }
            _ => {
                // reset to none
                *keycode = None;
            }
        }
    }

    // display key name
    let name = keycode.and_then(keycode_to_name).unwrap_or_default();
    ui.same_line_with_spacing(0.0, SPACING);
    ui.text(name);
}
