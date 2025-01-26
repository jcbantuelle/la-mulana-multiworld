use std::ffi::OsStr;
use std::ptr::null_mut;
use std::os::windows::ffi::OsStrExt;
use winapi::um::winuser::{MB_OK, MessageBoxW};

pub fn show_message_box(message: &str) {
    let converted_message = create_wstring(message);
    unsafe {
        MessageBoxW(null_mut(), converted_message.as_ptr(), null_mut(), MB_OK);
    }
}

fn create_wstring(str : &str) -> Vec<u16> {
    return OsStr::new(str).encode_wide().chain(Some(0).into_iter()).collect();
}
