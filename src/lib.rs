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

unsafe fn task_init(patch_version: winapi::shared::ntdef::INT) {
    let init_message = format!("Init task called. Patch Version: {}", patch_version);
    show_message_box(&init_message);
    ExitProcess(1);
}

#[no_mangle]
extern "stdcall" fn DllMain(_h_inst_dll: HINSTANCE, fdw_reason: DWORD, _lpv_reserved: LPVOID) -> bool {
    if fdw_reason != DLL_PROCESS_ATTACH {
        return true;
    }
    unsafe {
        let rva0 = GetModuleHandleW(null_mut())
            .cast::<*const u8>()
            .sub(0x400000);
        let rva0_address = format!("Address: {:p}", rva0);
        show_message_box(&rva0_address);
        ExitProcess(1);
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

fn to_wstring(str : &str) -> Vec<u16> {
    return OsStr::new(str).encode_wide().chain(Some(0).into_iter()).collect();
}
