use std::sync::Mutex;
use std::net::TcpStream;

use log::debug;

use winapi::shared::minwindef::*;
use winapi::um::timeapi::timeGetTime;
use winapi::um::processthreadsapi::ExitProcess;

use tungstenite::{stream::MaybeTlsStream, WebSocket};

use crate::utils::show_message_box;
use crate::network::TestMessagePayload;

pub static INIT_ATTACH_ADDRESS: usize = 0xdb9060;
pub static GAME_LOOP_ATTACH_ADDRESS: usize = 0xdb9064;
pub static OPTION_SDATA_NUM_ADDRESS: usize = 0x00db6fb7;
pub static OPTION_SDATA_ADDRESS: usize = 0x00db7048;
pub static OPTION_POS_CX_ADDRESS: usize = 0x00db7168;
pub static OPTION_POS_CY_ADDRESS: usize = 0x00db714c;
pub static SET_VIEW_EVENT_NS_ADDRESS: usize = 0x00507160;
pub static ITEM_GET_AREA_INIT_ADDRESS: usize = 0x004b8950;
pub static ITEM_GET_AREA_BACK_ADDRESS: usize = 0x004b8a80;
pub static ROOM_DATA_ADDRESS: usize = 0x00db5998;
pub static ITEM_GET_ADDRESS: usize = 0x006d4f80;
pub static ITEM_GET_POS_ADDRESS: usize = 0x006d5804;
pub static ITEM_GET_AREA_HIT_ADDRESS: usize = 0x004b89c0;

static mut GAME_SERVER_LOOP_COUNTER: u32 = 1;
static mut APPLICATION: Option<Application> = None;

pub struct Application {
    pub address: *mut u8,
    pub websocket: Mutex<WebSocket<MaybeTlsStream<TcpStream>>>
}

impl Application {
    pub unsafe fn attach(address: *mut u8, websocket: Mutex<WebSocket<MaybeTlsStream<TcpStream>>>) {
        let app = Application { address, websocket };
        *app.get_address(INIT_ATTACH_ADDRESS) = Self::app_init as *const usize;
        *app.get_address(GAME_LOOP_ATTACH_ADDRESS) = Self::game_loop as *const usize;
        APPLICATION = Some(app);
    }

    unsafe extern "stdcall" fn app_init(patch_version: winapi::shared::ntdef::INT) {
        if patch_version != 1 {
            let init_message = format!("EXE Patch Version does not match DLL. Please re-patch.");
            show_message_box(&init_message);
            ExitProcess(1);
        }
    }

    unsafe extern "stdcall" fn game_loop() -> DWORD {
        APPLICATION.as_ref().map(|app| {
            let _ = app.websocket.lock().unwrap().read_message().map(|message| {
                let data = message.into_data();
                let _ = serde_json::from_slice::<TestMessagePayload>(data.as_ref()).map(|payload| {
                    debug!("{:?}", payload);
                });
            });

            if GAME_SERVER_LOOP_COUNTER % 2000 == 0 {
                app.give_item(81);
            }
            GAME_SERVER_LOOP_COUNTER = GAME_SERVER_LOOP_COUNTER + 1;
        });

        return timeGetTime();
    }

    pub unsafe fn get_address<T>(&self, offset: usize) -> &mut T {
        &mut *self.address.wrapping_add(offset).cast()
    }

    pub unsafe fn give_item(&self, item: u32) {
        self.option_pos(0.0, 0.0);
        self.option_stuck(item);
        self.option_stuck(160);
        self.option_stuck(120);
        self.option_stuck(39);

        let item_get_area_init: *const usize = self.get_address(ITEM_GET_AREA_INIT_ADDRESS);

        let set_view_event_ns: &*const () = self.get_address(SET_VIEW_EVENT_NS_ADDRESS);
        let set_view_event_ns_func: extern "C" fn(u16, *const usize) = std::mem::transmute(set_view_event_ns);
        (set_view_event_ns_func)(16, item_get_area_init);
    }

    unsafe fn option_stuck(&self, option_num: u32) {
        let s_data_num: &mut u8 = self.get_address(OPTION_SDATA_NUM_ADDRESS);
        if *s_data_num < 32 {
            let s_data: &mut [u32;32] = self.get_address(OPTION_SDATA_ADDRESS);
            s_data[*s_data_num as usize] = option_num;
            *s_data_num = *s_data_num + 1
        }
    }

    unsafe fn option_pos(&self, x: f32, y: f32) {
        *self.get_address(OPTION_POS_CX_ADDRESS) = x;
        *self.get_address(OPTION_POS_CY_ADDRESS) = y;
    }
}
