use log::debug;

use winapi::shared::minwindef::*;
use winapi::um::timeapi::timeGetTime;
use winapi::um::processthreadsapi::ExitProcess;

use crate::utils::show_message_box;
use crate::network::{Randomizer, RandomizerMessage};
use crate::lm_structs::taskdata::TaskData;
use crate::lm_structs::taskdata::EventWithBool;
use crate::lm_structs::script_header::{ScriptHeader, ScriptSubHeader};
use crate::screenplay;

pub static INIT_ATTACH_ADDRESS: usize = 0xdb9060;
pub static GAME_LOOP_ATTACH_ADDRESS: usize = 0xdb9064;
pub static POPUP_DIALOG_DRAW_INTERCEPT: usize = 0xdb9068;
pub static ITEM_SYMBOL_INIT_INTERCEPT: usize = 0xdb906c;
pub static OPTION_SDATA_NUM_ADDRESS: usize = 0x00db6fb7;
pub static OPTION_SDATA_ADDRESS: usize = 0x00db7048;
pub static OPTION_POS_CX_ADDRESS: usize = 0x00db7168;
pub static OPTION_POS_CY_ADDRESS: usize = 0x00db714c;
pub static SET_VIEW_EVENT_NS_ADDRESS: usize = 0x00507160;
pub static ITEM_GET_AREA_INIT_ADDRESS: usize = 0x004b8950;
pub static POPUP_DIALOG_DRAW_ADDRESS: usize = 0x005917b0;
pub static SCRIPT_HEADER_POINTER_ADDRESS: usize = 0x006d296c;
pub static ITEM_SYMBOL_INIT_POINTER_ADDRESS: usize = 0x006d1174;
pub static ITEM_SYMBOL_INIT_ADDRESS: usize = 0x004b8ae0;
pub static ITEM_SYMBOL_BACK_ADDRESS: usize = 0x004b8e70;

static mut GAME_SERVER_LOOP_COUNTER: u32 = 1;
static mut PLAYER_ITEM: Option<PlayerItem> = None;
static mut PLAYER_ITEM_POPUP: Option<PlayerItemPopup> = None;
static mut APPLICATION: Option<Application> = None;

pub struct PlayerItem {
    pub player_id: i32,
    pub for_player: bool
}

pub struct PlayerItemPopup {
    pub popup_id_address: *const u32,
    pub popup_id: u32,
    pub encoded: Vec<u16>,
    pub line_address: *mut ScriptSubHeader,
    pub old_line: ScriptSubHeader,
}

pub struct Application {
    pub address: *mut u8,
    pub randomizer: Randomizer
}

impl Application {
    pub unsafe fn attach(address: *mut u8, randomizer: Randomizer) {
        let app = Application { address, randomizer };
        *app.get_address(INIT_ATTACH_ADDRESS) = Self::app_init as *const usize;
        *app.get_address(GAME_LOOP_ATTACH_ADDRESS) = Self::game_loop as *const usize;
        *app.get_address(POPUP_DIALOG_DRAW_INTERCEPT) = Self::popup_dialog_draw_intercept as *const usize;
        *app.get_address(ITEM_SYMBOL_INIT_POINTER_ADDRESS) = Self::item_symbol_init_intercept as *const usize;
        *app.get_address(ITEM_SYMBOL_INIT_INTERCEPT) = Self::item_symbol_init_intercept as *const usize;
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
            let _ = app.randomizer.read_messages(|payload| {
                debug!("{:?}", payload);
            });

            PLAYER_ITEM_POPUP.as_ref().map(|popup|{
                if popup.popup_id != *popup.popup_id_address {
                    *popup.line_address = popup.old_line;
                    PLAYER_ITEM_POPUP = None;
                }
            });

            // if GAME_SERVER_LOOP_COUNTER == 2000 {
            //     let player_item = PlayerItem {
            //         player_id: 2,
            //         for_player: true
            //     };
            //     PLAYER_ITEM = Some(player_item);
            //     app.give_item(81);
            // }
            // if GAME_SERVER_LOOP_COUNTER == 3000 {
            //     app.give_item(82);
            // }
            GAME_SERVER_LOOP_COUNTER = GAME_SERVER_LOOP_COUNTER + 1;
        });

        return timeGetTime();
    }

    unsafe extern "stdcall" fn popup_dialog_draw_intercept(popup_dialog: &TaskData) {
        APPLICATION.as_ref().map(|app| {
            let script_header: &*const ScriptHeader = app.get_address(SCRIPT_HEADER_POINTER_ADDRESS);
            let card = (*script_header.add(3)).data;
            let line = card.add(2);

            PLAYER_ITEM.as_ref().map_or_else(|| {app.popup_dialog_draw(popup_dialog)},|player_item| {
                PLAYER_ITEM_POPUP = Some(PlayerItemPopup {
                    popup_id_address: &popup_dialog.id.uid,
                    popup_id: popup_dialog.id.uid,
                    encoded: screenplay::encode("  From Arakida!".to_string()),
                    line_address: line,
                    old_line: (*line).clone()
                });

                let item_popup = PLAYER_ITEM_POPUP.as_ref().unwrap();

                *line = ScriptSubHeader {
                    pointer: item_popup.encoded.as_ptr(),
                    data_num: item_popup.encoded.len() as i32,
                    font_num: (item_popup.encoded.len() - 3) as i32
                };

                app.popup_dialog_draw(popup_dialog);

                PLAYER_ITEM = None;
            });
        });
    }

    unsafe extern "stdcall" fn item_symbol_init_intercept(item: &mut TaskData) {
        APPLICATION.as_ref().map(|app| {
            let item_symbol_init: &*const () = app.get_address(ITEM_SYMBOL_INIT_ADDRESS);
            let item_symbol_init_func: extern "C" fn(&TaskData) = std::mem::transmute(item_symbol_init);
            (item_symbol_init_func)(item);
            item.rfunc = Self::item_symbol_back_intercept as EventWithBool;
        });
    }

    unsafe fn item_symbol_back_intercept(item: &mut TaskData) -> u32 {
        APPLICATION.as_ref().map(|app| {
            if item.hit_data > 0 {
                app.randomizer.send_message(RandomizerMessage {
                    player_id: 1,
                    body: "Test message".to_string()
                });
            }
            let item_symbol_back: &*const () = app.get_address(ITEM_SYMBOL_BACK_ADDRESS);
            let item_symbol_back_func: extern "C" fn(&TaskData) -> u32 = std::mem::transmute(item_symbol_back);
            (item_symbol_back_func)(item)
        }).expect("Application Not Loaded")
    }

    unsafe fn popup_dialog_draw(&self, popup_dialog: &TaskData) {
        let popup_dialog_draw: &*const () = self.get_address(POPUP_DIALOG_DRAW_ADDRESS);
        let popup_dialog_draw_func: extern "C" fn(&TaskData) = std::mem::transmute(popup_dialog_draw);
        (popup_dialog_draw_func)(popup_dialog);
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
