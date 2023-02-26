//! Predefined primitive UI elements & render helpers.

use crate::util::{keycode_to_name, name_to_keycode};
use arcdps::imgui::{
    sys, Id, InputTextFlags, StyleStackToken, StyleVar, TableColumnFlags, TableColumnSetup,
    TableFlags, TableToken, Ui,
};
use std::{
    ffi::{c_void, CString},
    mem,
};
use windows::Win32::Graphics::Direct3D11::ID3D11ShaderResourceView;

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

/// Renders a table with (optional) icon headers.
pub fn table_with_icons<'ui, N>(
    ui: &Ui<'ui>,
    label: impl AsRef<str>,
    columns: &[TableIconColumn<N>],
    flags: TableFlags,
    show_icons: bool,
) -> Option<TableToken<'ui>>
where
    N: AsRef<str>,
{
    if let Some(token) = ui.begin_table_with_flags(label, columns.len(), flags) {
        for column in columns {
            ui.table_setup_column_with(column.as_setup());
        }

        if show_icons {
            for column in columns {
                table_header_icon(ui, column.name.as_ref(), column.icon);
            }
        } else {
            ui.table_headers_row();
        }

        Some(token)
    } else {
        None
    }
}

/// Icon type.
pub type Icon = ID3D11ShaderResourceView;

/// A table column setup with icon.
#[derive(Debug, Clone)]
pub struct TableIconColumn<'i, 'id, Name> {
    pub name: Name,
    pub icon: Option<&'i Icon>,
    pub flags: TableColumnFlags,
    pub init_width_or_weight: f32,
    pub user_id: Id<'id>,
}

impl<'i, 'id, Name> TableIconColumn<'i, 'id, Name> {
    /// Creates a new icon column.
    pub fn new(name: Name, icon: Option<&'i Icon>) -> Self {
        Self::with_flags(name, icon, Default::default())
    }

    /// Creates a new icon column with given flags.
    pub fn with_flags(name: Name, icon: Option<&'i Icon>, flags: TableColumnFlags) -> Self {
        Self::with_id(name, icon, flags, 0.0, Default::default())
    }

    /// Creates a new icon column with given width/weight and id.
    pub fn with_id(
        name: Name,
        icon: Option<&'i Icon>,
        flags: TableColumnFlags,
        init_width_or_weight: f32,
        user_id: Id<'id>,
    ) -> Self {
        Self {
            name,
            icon,
            flags,
            init_width_or_weight,
            user_id,
        }
    }

    /// Generates the equivalent [`TableColumnSetup`].
    pub fn as_setup(&self) -> TableColumnSetup<'id, &str>
    where
        Name: AsRef<str>,
    {
        TableColumnSetup {
            name: self.name.as_ref(),
            flags: self.flags,
            init_width_or_weight: self.init_width_or_weight,
            user_id: self.user_id,
        }
    }
}

impl<N> Default for TableIconColumn<'_, '_, N>
where
    N: Default,
{
    fn default() -> Self {
        Self::new(Default::default(), None)
    }
}

/// Renders a table header with icon.
pub fn table_header_icon(ui: &Ui, label: impl AsRef<str>, icon: Option<&Icon>) {
    ui.table_next_column();
    if let Some(icon) = icon {
        // avoid dropping by transmuting the reference
        let ptr = *unsafe { mem::transmute::<_, &*const c_void>(icon) };

        let size = ui.text_line_height_with_spacing();
        let [pos_x, pos_y] = ui.cursor_screen_pos();
        let top = pos_y + (ui.text_line_height() - size) / 2.0;

        ui.set_cursor_screen_pos([pos_x + size, pos_y]);
        ui.table_header("");

        ui.get_window_draw_list()
            .add_image(ptr.into(), [pos_x, top], [pos_x + size, top + size])
            .build();
    } else {
        ui.table_header(label);
    }
}
