use super::ch_width;
use crate::util::{keycode_to_name, name_to_keycode};
use arcdps::imgui::{sys, InputTextFlags, Ui};
use std::ffi::CString;

/// Renders an input for a [`u32`].
pub fn input_u32(
    ui: &Ui,
    label: impl AsRef<str>,
    value: &mut u32,
    step: u32,
    step_fast: u32,
    flags: InputTextFlags,
) -> bool {
    let mut int = *value as _;
    if ui
        .input_int(label, &mut int)
        .step(step as _)
        .step_fast(step_fast as _)
        .flags(flags)
        .build()
    {
        if let Ok(new) = u32::try_from(int) {
            *value = new;
            return true;
        }
    }
    false
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
    ui.group(|| {
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
    });
}
