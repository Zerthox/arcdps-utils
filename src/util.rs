use std::ffi::CString;
use windows::Win32::{
    Foundation::CHAR,
    UI::{
        Input::KeyboardAndMouse::{GetKeyNameTextA, MapVirtualKeyA, VkKeyScanA},
        WindowsAndMessaging::MAPVK_VK_TO_VSC,
    },
};

/// Converts a key's name to its keycode.
pub fn name_to_keycode(name: u8) -> u32 {
    let result = unsafe { VkKeyScanA(CHAR(name)) } as u32;
    result & 0xff
}

/// Converts a keycode to the key's name.
pub fn keycode_to_name(keycode: u32) -> Option<String> {
    let scan_code = unsafe { MapVirtualKeyA(keycode, MAPVK_VK_TO_VSC) };
    let mut buffer = vec![0; 32];

    let result = unsafe { GetKeyNameTextA((scan_code << 16) as i32, &mut buffer) };

    if result > 0 {
        unsafe { buffer.set_len(result as usize) }
        CString::new(buffer)
            .ok()
            .and_then(|string| string.into_string().ok())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_to_code() {
        assert_eq!(65, name_to_keycode('A' as u8));
        assert_eq!(70, name_to_keycode('F' as u8));
    }

    #[test]
    fn code_to_name() {
        assert_eq!(
            Some(String::from("ALT")),
            keycode_to_name(18).map(|name| name.to_uppercase())
        );
        assert_eq!(
            Some(String::from("A")),
            keycode_to_name(65).map(|name| name.to_uppercase())
        );
        assert_eq!(
            Some(String::from("F")),
            keycode_to_name(70).map(|name| name.to_uppercase())
        );
    }
}
