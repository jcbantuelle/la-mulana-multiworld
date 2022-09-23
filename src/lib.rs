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
pub mod taskdata;
pub mod roomcache;
pub mod hitbox;
use application::{ Application, show_message_box, SET_VIEW_EVENT_NS_ADDRESS, ITEM_GET_AREA_INIT_ADDRESS, ITEM_GET_AREA_BACK_ADDRESS, ROOM_DATA_ADDRESS, ITEM_GET_ADDRESS, ITEM_GET_POS_ADDRESS, ITEM_GET_AREA_HIT_ADDRESS };
use crate::taskdata::TaskData;
use crate::roomcache::RoomCache;
use crate::hitbox::HitBox;

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

    if GAME_SERVER_LOOP_COUNTER % 2000 == 0 {
        APPLICATION.as_ref().map(|app| {
            app.give_item(81);
        });
    }
    GAME_LOOP_COUNTER += 1;

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
        let app = Application {
            address: GetModuleHandleW(null_mut()).cast::<u8>().wrapping_sub(0x400000)
        };

        *app.get_address(0xdb9060) = init as *const usize;
        *app.get_address(0xdb9064) = game_loop as *const usize;

        APPLICATION = Some(app);
        return true;
    }
}

