use std::ffi::OsStr;
use std::mem::size_of;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use winapi::shared::ntdef::*;
use winapi::um::winnt::DLL_PROCESS_ATTACH;
use winapi::um::winuser::{MB_OK, MessageBoxW};
use winapi::shared::minwindef::*;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::processthreadsapi::ExitProcess;
use reqwest;

fn to_wstring(str : &str) -> Vec<u16> {
    let v : Vec<u16> =
        OsStr::new(str).encode_wide().chain(Some(0).into_iter()).collect();
    v
}

unsafe fn task_init(patch_version: winapi::shared::ntdef::INT) {
    show_message_box("Init task called.");
    ExitProcess(1);
}

#[no_mangle]
extern "stdcall" fn DllMain(hInstDll: HINSTANCE, fdwReason: DWORD, lpvReserved: LPVOID) -> bool {
    if fdwReason != DLL_PROCESS_ATTACH {
        return true;
    }
    unsafe {
        let rva0 = GetModuleHandleW(null_mut())
            .cast::<*const u8>()
            .sub(0x400000);
        write_address(rva0, 0xdb9060, task_init as *const usize);
        return true;
    }
}

unsafe fn write_address(base_address: *mut *const u8, offset: usize, f: *const usize) {
    base_address.add(offset).cast::<*const usize>().write(f);
}

unsafe fn show_message_box(message: &str) {
    let converted_message = to_wstring(message).as_ptr();
    MessageBoxW(null_mut(), converted_message, null_mut(), MB_OK);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
    }
}
