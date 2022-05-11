//! Predefined UI components & render helpers.

pub mod window;

#[cfg(feature = "log")]
pub mod log;

use crate::util::{keycode_to_name, name_to_keycode};
use arcdps::imgui::{sys, Ui};
use std::ffi::CString;

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

/// Renders a custom key input.
pub fn key_input(ui: &Ui, id: impl AsRef<str>, label: impl AsRef<str>, keycode: &mut Option<u32>) {
    const SPACING: f32 = 5.0;

    ui.text(label);

    let mut buffer = String::with_capacity(3);
    if let Some(keycode) = keycode {
        buffer.push_str(&keycode.to_string());
    }
    ui.same_line_with_spacing(0.0, SPACING);
    ui.push_item_width(ui.calc_text_size("0000")[0]);
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
