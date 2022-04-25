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
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use log::{debug, error, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use fragile::Fragile;

static LOG_FILE_NAME: &str = "lamulanamw.log";
static SERVER_URL: &str = "wss://la-mulana.arakiyda.com/cable";

lazy_static! {
    static ref WEBSOCKET: Mutex<WebSocket<MaybeTlsStream<TcpStream>>> = {
        let url = url::Url::parse(SERVER_URL).unwrap();
        let (ws_connection, _) = connect(url).expect("Failed to connect");
        match ws_connection.get_ref() {
            MaybeTlsStream::NativeTls(ref tls) => {
                tls.get_ref().set_nonblocking(true).map_err(|err| {
                    error!("Could not set socket as nonblocking: {}", err);
                }).unwrap();
            },
            _ => ()
        };
        Mutex::new(ws_connection)
    };
    static ref APPLICATION_ADDRESS: Mutex<Fragile<*mut *const u8>> = {
        let t = unsafe { GetModuleHandleW(null_mut()).cast::<*const u8>().sub(0x100000) };
        Mutex::new(
            Fragile::new(
                t
            )
        )
    };
}

#[derive(Serialize, Deserialize)]
struct InitialPayload {
    command: String,
    identifier: String
}

#[derive(Serialize, Deserialize)]
struct Identifier {
    id: u64,
    channel: String
}

#[derive(Serialize, Deserialize, Debug)]
struct TestMessagePayload {
    identifier: String,
    message: TestMessage
}

#[derive(Serialize, Deserialize, Debug)]
struct TestMessage {
    body: String
}

unsafe extern "stdcall" fn init(patch_version: winapi::shared::ntdef::INT) {
    if patch_version != 1 {
        let init_message = format!("EXE Patch Version does not match DLL. Please re-patch.");
        show_message_box(&init_message);
        ExitProcess(1);
    }

    let ident = Identifier {
        id: 15,
        channel: "MultiworldSyncChannel".to_string()
    };
    let initial_payload = InitialPayload {
        command: "subscribe".to_string(),
        identifier: serde_json::to_string(&ident).unwrap()
    };

    WEBSOCKET.lock().unwrap().write_message(Message::Text(serde_json::to_string(&initial_payload).unwrap())).expect("Unable to Connect To Websocket Channel");
}

unsafe extern "stdcall" fn game_loop() -> DWORD {
    let _ = WEBSOCKET.lock().unwrap().read_message().map(|message| {
        let data = message.into_data();
        let _ = serde_json::from_slice::<TestMessagePayload>(data.as_ref()).map(|payload| {
            debug!("{:?}", payload);
        });
    });

    let fragile_application_address = APPLICATION_ADDRESS.lock().unwrap();
    let application_address = fragile_application_address.get();
    let ptr = application_address.add(0x4d9690/4).cast::<*const ()>();
    let f: extern "C" fn() = std::mem::transmute(ptr);
    (f)();

    return timeGetTime();
}

#[no_mangle]
extern "stdcall" fn DllMain(_h_inst_dll: HINSTANCE, fdw_reason: DWORD, _lpv_reserved: LPVOID) -> bool {
    if fdw_reason != DLL_PROCESS_ATTACH {
        return true;
    }
    let file_appender = FileAppender::builder()
        .build(LOG_FILE_NAME)
        .unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("lamulanamw", Box::new(file_appender)))
        .build(Root::builder().appender("lamulanamw").build(LevelFilter::Debug))
        .unwrap();
    log4rs::init_config(config).unwrap();

    unsafe {
        let fragile_application_address = APPLICATION_ADDRESS.lock().unwrap();
        let application_address = fragile_application_address.get();
        write_address( application_address, 0xdb9060, init as *const usize);
        write_address(application_address, 0xdb9064, game_loop as *const usize);
        return true;
    }
}

unsafe fn write_address(base_address: &*mut *const u8, offset: usize, f: *const usize) {
    base_address.add(offset/4).cast::<*const usize>().write(f);
}

unsafe fn show_message_box(message: &str) {
    let converted_message = to_wstring(message).as_ptr();
    MessageBoxW(null_mut(), converted_message, null_mut(), MB_OK);
}

fn to_wstring(str : &str) -> Vec<u16> {
    return OsStr::new(str).encode_wide().chain(Some(0).into_iter()).collect();
}
