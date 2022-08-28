use std::ptr::{null_mut};
use std::net::TcpStream;
use winapi::um::winnt::DLL_PROCESS_ATTACH;
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

pub mod application;
use application::{ Application, show_message_box, SET_VIEW_EVENT_NS_ADDRESS, ITEM_GET_AREA_INIT_ADDRESS };
static LOG_FILE_NAME: &str = "lamulanamw.log";
static SERVER_URL: &str = "wss://la-mulana.arakiyda.com/cable";
static mut GAME_SERVER_LOOP_COUNTER: u32 = 1;
static mut APPLICATION: Option<Application> = None;

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

    // debug!("Game loop counter is {}", GAME_SERVER_LOOP_COUNTER);
    if GAME_SERVER_LOOP_COUNTER % 2000 == 0 {
        APPLICATION.as_ref().map(|app| {
            app.option_pos(0.0, 0.0);
            app.option_stuck(81);
            app.option_stuck(32);
            app.option_stuck(2);
            app.option_stuck(39);

            debug!("Executing setViewEventNs");
            let set_view_event_ns = app.get_address_from_offset(SET_VIEW_EVENT_NS_ADDRESS).cast::<*const ()>();
            let item_get_area_init = app.get_address_from_offset(ITEM_GET_AREA_INIT_ADDRESS);
            let f: extern "C" fn(u16, *const usize) = std::mem::transmute(set_view_event_ns);
            (f)(16, test_setns as *const usize);
            debug!("Finished executing setViewEventNs");

            let s_data_num = app.get_sdata_num();
            debug!("s_data_num is now {}", s_data_num);
        });
    }
    GAME_SERVER_LOOP_COUNTER = GAME_SERVER_LOOP_COUNTER + 1;

    return timeGetTime();
}

unsafe fn test_setns(ptr: usize) {
    let init_message = format!("Success!");
    show_message_box(&init_message);
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
        let app = Application {
            address: GetModuleHandleW(null_mut()).cast::<*const u8>().sub(0x100000)
        };

        app.write_address(0xdb9060, init as *const usize);
        app.write_address(0xdb9064, game_loop as *const usize);

        APPLICATION = Some(app);
        return true;
    }
}

