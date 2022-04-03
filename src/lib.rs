use std::fs;
use std::io::Write;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use std::net::TcpStream;
use winapi::um::winnt::DLL_PROCESS_ATTACH;
use winapi::um::winuser::{MB_OK, MessageBoxW};
use winapi::shared::minwindef::*;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::processthreadsapi::ExitProcess;
use winapi::um::timeapi::timeGetTime;
use tungstenite::{stream::MaybeTlsStream, WebSocket, connect, Message};
use serde_json::json;
use serde_json::Value;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref WEBSOCKET: Mutex<WebSocket<MaybeTlsStream<TcpStream>>> = {
        let url = url::Url::parse("wss://la-mulana.arakiyda.com/cable").unwrap();
        let (ws_connection, _) = connect(url).expect("Failed to connect");
        match ws_connection.get_ref() {
            MaybeTlsStream::NativeTls(ref tls) => {
                tls.get_ref().set_nonblocking(true);
                return;
            },
            _ => ()
        };
        return Mutex::new(ws_connection);
    };
}

unsafe extern "stdcall" fn init(patch_version: winapi::shared::ntdef::INT) {
    if patch_version != 1 {
        let init_message = format!("EXE Patch Version does not match DLL. Please re-patch.");
        show_message_box(&init_message);
        ExitProcess(1);
    }

    let handshake_message = json!({
        "command": "subscribe",
        "identifier": json!({
            "id": "15",
            "channel": "MultiworldSyncChannel"
        }).to_string()
    }).to_string();

    WEBSOCKET.lock().unwrap().write_message(Message::text(handshake_message.to_string())).expect("Unable to Connect To Websocket Channel");
}

unsafe extern "stdcall" fn game_loop() -> DWORD {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("websocket_messages.txt")
        .unwrap();

    WEBSOCKET.lock().unwrap().read_message().map(|message|{
        let data = message.into_data();
        let json_data: Value = serde_json::from_slice(&data).unwrap();
        match &json_data["message"]["body"] {
            Value::String(body) => {
                file.write_all(body.as_ref());
                return;
            },
            _ => ()
        }
    });

    return timeGetTime();
}

#[no_mangle]
extern "stdcall" fn DllMain(_h_inst_dll: HINSTANCE, fdw_reason: DWORD, _lpv_reserved: LPVOID) -> bool {
    if fdw_reason != DLL_PROCESS_ATTACH {
        return true;
    }
    unsafe {
        let rva0 = GetModuleHandleW(null_mut()).cast::<*const u8>().sub(0x100000);
        write_address(rva0, 0xdb9060, init as *const usize);
        write_address(rva0, 0xdb9064, game_loop as *const usize);
        return true;
    }
}

unsafe fn write_address(base_address: *mut *const u8, offset: usize, f: *const usize) {
    base_address.add(offset/4).cast::<*const usize>().write(f);
}

unsafe fn show_message_box(message: &str) {
    let converted_message = to_wstring(message).as_ptr();
    MessageBoxW(null_mut(), converted_message, null_mut(), MB_OK);
}

fn to_wstring(str : &str) -> Vec<u16> {
    return OsStr::new(str).encode_wide().chain(Some(0).into_iter()).collect();
}
