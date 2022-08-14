use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr::{null_mut, write};
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
use std::sync::Mutex;
use log::{debug, error, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use fragile::Fragile;

static LOG_FILE_NAME: &str = "lamulanamw.log";
static SERVER_URL: &str = "wss://la-mulana.arakiyda.com/cable";
static mut GAME_SERVER_LOOP_COUNTER: u32 = 1;
static mut APPLICATION_ADDRESS: *mut *const u8 = null_mut();
static OPTION_SDATA_NUM_ADDRESS: usize = 0x00db6fb7;
static OPTION_SDATA_ADDRESS: usize = 0x00db7048;
static OPTION_POS_CX: usize = 0x00db7168;
static OPTION_POS_CY: usize = 0x00db714c;
static SET_NS_EVENT_PTR: usize = 0x00507160;
static SET_ITEM_GET_AREA_INIT_PTR: usize = 0x004b8950;

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
    debug!("DLL Start.");
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

    debug!("Game loop counter is {}", GAME_SERVER_LOOP_COUNTER);
    if GAME_SERVER_LOOP_COUNTER % 2000 == 0 {
        debug!("About to spawn item...");
        option_pos(640.0, 480.0);
        option_stuck(81);
        option_stuck(32);
        option_stuck(24);
        option_stuck(39);

        debug!("Executing setViewEventNs");
        let ptr = get_address_from_offset(0x4d9690).cast::<*const ()>();
        let f: extern "C" fn(u16, usize) = std::mem::transmute(ptr);
        (f)(16, SET_ITEM_GET_AREA_INIT_PTR);
        debug!("Finished executing setViewEventNs");
    }
    GAME_SERVER_LOOP_COUNTER = GAME_SERVER_LOOP_COUNTER + 1;

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
        APPLICATION_ADDRESS = GetModuleHandleW(null_mut()).cast::<*const u8>().sub(0x100000);
        write_address(0xdb9060, init as *const usize);
        write_address(0xdb9064, game_loop as *const usize);
        return true;
    }
}

unsafe fn write_address<T>(offset: usize, f: T) {
    write_address_with_offset(offset, 0, f);
}

unsafe fn write_address_with_offset<T>(offset: usize, additional_offset: usize, f: T) {
    get_address_from_offset(offset).add(additional_offset).cast::<T>().write(f);
}

unsafe fn read_address<T>(offset: usize) -> T {
    get_address_from_offset(offset).cast::<T>().read()
}

unsafe fn get_address_from_offset(offset: usize) -> *mut *const u8 {
    APPLICATION_ADDRESS.get().add(offset/4)
}

unsafe fn show_message_box(message: &str) {
    let converted_message = to_wstring(message).as_ptr();
    MessageBoxW(null_mut(), converted_message, null_mut(), MB_OK);
}

unsafe fn option_stuck(option_num: u16) {
    let s_data_num: u16 = read_address(OPTION_SDATA_NUM_ADDRESS);
    if s_data_num & 0xe0 == 0 {
        let updated_s_data_num = s_data_num + 1;
        write_address_with_offset(OPTION_SDATA_NUM_ADDRESS, updated_s_data_num as usize, option_num);
    }
}

unsafe fn option_pos(x: f32, y: f32) {
    write_address(OPTION_POS_CX, x);
    write_address(OPTION_POS_CY, y);
}

fn to_wstring(str : &str) -> Vec<u16> {
    return OsStr::new(str).encode_wide().chain(Some(0).into_iter()).collect();
}
