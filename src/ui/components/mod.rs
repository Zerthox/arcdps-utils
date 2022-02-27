//! Predefined UI components & render helpers.

pub mod window;

#[cfg(feature = "log")]
pub mod log;

use arcdps::imgui::sys;
use std::ffi::CString;

/// Renders a right-click context menu for the last item.
pub fn item_context_menu(str_id: impl AsRef<str>, contents: impl FnOnce()) {
    if let Ok(str_id) = CString::new(str_id.as_ref()) {
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
