use std::ptr;
use log::debug;
use std::sync::Mutex;
use lazy_static::lazy_static;

use winapi::shared::minwindef::*;
use winapi::um::timeapi::timeGetTime;
use winapi::um::processthreadsapi::ExitProcess;

use crate::{Application, APPLICATION};
use crate::utils::show_message_box;
use crate::network::{Randomizer, RandomizerMessage};
use crate::lm_structs::taskdata::TaskData;
use crate::lm_structs::taskdata::EventWithBool;
use crate::lm_structs::script_header::{ScriptHeader, ScriptSubHeader};
use crate::{AppConfig, screenplay};

pub static INIT_ATTACH_ADDRESS: usize = 0xdb9060;
pub static GAME_LOOP_ATTACH_ADDRESS: usize = 0xdb9064;
pub static POPUP_DIALOG_DRAW_INTERCEPT: usize = 0xdb9068;
pub static GAME_INIT_ADDRESS: usize = 0x00db753c;
pub static LEMEZA_ADDRESS: usize = 0x00db7538;
pub static GAME_PROCESS_ADDRESS: usize = 0x00db7178;
pub static WARP_MENU_STATUS_ADDRESS: usize = 0x006d59cc;
pub static MOVEMENT_STATUS_ADDRESS: usize = 0x006d59c0;
pub static ITEM_SYMBOL_INIT_INTERCEPT: usize = 0xdb906c;
pub static OPTION_SDATA_NUM_ADDRESS: usize = 0x00db6fb7;
pub static OPTION_SDATA_ADDRESS: usize = 0x00db7048;
pub static OPTION_POS_CX_ADDRESS: usize = 0x00db7168;
pub static OPTION_POS_CY_ADDRESS: usize = 0x00db714c;
pub static SET_VIEW_EVENT_NS_ADDRESS: usize = 0x00507160;
pub static SET_TASK_ADDRESS: usize = 0x00607570;
pub static SET_SE_ADDRESS: usize = 0x00417600;
pub static SE_ADDRESS: usize = 0x006d2708;
pub static ITEM_GET_AREA_INIT_ADDRESS: usize = 0x004b8950;
pub static POPUP_DIALOG_INIT_ADDRESS: usize = 0x00591520;
pub static POPUP_DIALOG_DRAW_ADDRESS: usize = 0x005917b0;
pub static SCRIPT_HEADER_POINTER_ADDRESS: usize = 0x006d296c;
pub static ITEM_SYMBOL_INIT_POINTER_ADDRESS: usize = 0x006d1174;
pub static ITEM_SYMBOL_INIT_ADDRESS: usize = 0x004b8ae0;
pub static ITEM_SYMBOL_BACK_ADDRESS: usize = 0x004b8e70;

static mut GAME_SERVER_LOOP_COUNTER: u32 = 1;

lazy_static! {
    static ref ITEMS_TO_GIVE: Mutex<Vec<GivenItem>> = Mutex::new(vec![]);
    static ref PLAYER_ITEM: Mutex<Option<PlayerItem>> = Mutex::new(None);
    static ref PLAYER_ITEM_POPUP: Mutex<Option<PlayerItemPopup>> = Mutex::new(None);
}

#[derive(Debug)]
pub struct GivenItem {
    pub player_id: u64,
    pub item_id: u32
}

pub struct PlayerItem {
    pub player_id: u64,
    pub for_player: bool
}

pub struct PlayerItemPopup {
    pub popup_id_address: usize,
    pub popup_id: u32,
    pub encoded: Vec<u16>,
    pub line_address: usize,
    pub old_line: ScriptSubHeader,
}

impl Application {
    pub fn attach(&self) {
        *self.get_address(INIT_ATTACH_ADDRESS) = Self::app_init as usize;
        *self.get_address(GAME_LOOP_ATTACH_ADDRESS) = Self::game_loop as usize;
        *self.get_address(POPUP_DIALOG_DRAW_INTERCEPT) = Self::popup_dialog_draw_intercept as usize;
        *self.get_address(ITEM_SYMBOL_INIT_POINTER_ADDRESS) = Self::item_symbol_init_intercept as usize;
        *self.get_address(ITEM_SYMBOL_INIT_INTERCEPT) = Self::item_symbol_init_intercept as usize;
    }

    extern "stdcall" fn app_init(patch_version: winapi::shared::ntdef::INT) {
        if patch_version != 1 {
            let init_message = format!("EXE Patch Version does not match DLL. Please re-patch.");
            show_message_box(&init_message);
            unsafe {
                ExitProcess(1);
            }
        }
    }

    extern "stdcall" fn game_loop() -> DWORD {
        let game_init: &mut u32 = APPLICATION.get_address(GAME_INIT_ADDRESS);
        if *game_init != 0 {
            let _ = APPLICATION.randomizer.read_messages(|payload| {
                let mut items_to_give = ITEMS_TO_GIVE.lock().unwrap();
                items_to_give.push(GivenItem {
                    player_id: payload.message.player_id,
                    item_id: payload.message.item_id
                });
                debug!("{:?}", payload.message);
            });

            let mut items_to_give = ITEMS_TO_GIVE.lock().unwrap();
            if !items_to_give.is_empty() {
                if let Some(player_item) = PLAYER_ITEM.try_lock().ok().as_mut() {
                    if player_item.is_none() {
                        let next_item = items_to_give.pop().unwrap();
                        **player_item = Some(PlayerItem {
                            player_id: next_item.player_id,
                            for_player: false
                        });
                        APPLICATION.give_item(next_item.item_id);
                    }
                }
            }

            if let Some(popup_option) = PLAYER_ITEM_POPUP.try_lock().ok().as_mut() {
                if let Some(popup) = popup_option.as_ref() {
                    if popup.popup_id != *APPLICATION.get_address::<u32>(popup.popup_id_address) {
                        *APPLICATION.get_address::<ScriptSubHeader>(popup.line_address) = popup.old_line;
                        **popup_option = None;
                    }
                }
            }
        }

        unsafe { timeGetTime() }
    }

    pub fn give_item(&self, item: u32) {
        self.option_pos(0.0, 0.0);
        self.option_stuck(item);
        self.option_stuck(160);
        self.option_stuck(120);
        self.option_stuck(39);

        let item_get_area_init: *const usize = self.get_address(ITEM_GET_AREA_INIT_ADDRESS);
        let set_view_event_ns: &*const () = self.get_address(SET_VIEW_EVENT_NS_ADDRESS);
        let set_view_event_ns_func: extern "C" fn(u16, *const usize) -> *const TaskData = unsafe { std::mem::transmute(set_view_event_ns) };
        (set_view_event_ns_func)(16, item_get_area_init);
    }

    extern "stdcall" fn popup_dialog_draw_intercept(popup_dialog: &TaskData) {
        let mut player_item_option = PLAYER_ITEM.lock().unwrap();
        if let Some(player_item) = player_item_option.as_ref() {
            let script_header: &*const ScriptHeader = APPLICATION.get_address(SCRIPT_HEADER_POINTER_ADDRESS);
            let line_header = unsafe { (*script_header.add(3)).data as *mut ScriptSubHeader};
            let line = unsafe { &mut *line_header.add(2) };

            let item_for_text = if player_item.for_player { "For"} else {"From"};

            let popup = PlayerItemPopup {
                popup_id_address: &popup_dialog.id.uid as *const u32 as usize,
                popup_id: popup_dialog.id.uid,
                encoded: screenplay::encode(format!("  {} Player {}", item_for_text, player_item.player_id)),
                line_address: line as *const ScriptSubHeader as usize,
                old_line: (*line).clone()
            };

            let mut popup_option = PLAYER_ITEM_POPUP.lock().unwrap();
            *popup_option = Some(popup);
            let popup = popup_option.as_ref().unwrap();

            *line = ScriptSubHeader {
                pointer: popup.encoded.as_ptr() as usize,
                data_num: popup.encoded.len() as i32,
                font_num: (popup.encoded.len() - 3) as i32
            };

            APPLICATION.popup_dialog_draw(popup_dialog);

            *player_item_option = None;
        } else {
            APPLICATION.popup_dialog_draw(popup_dialog);
        }
    }

    fn popup_dialog_draw(&self, popup_dialog: &TaskData) {
        let popup_dialog_draw: &*const () = self.get_address(POPUP_DIALOG_DRAW_ADDRESS);
        let popup_dialog_draw_func: extern "C" fn(&TaskData) = unsafe { std::mem::transmute(popup_dialog_draw) };
        (popup_dialog_draw_func)(popup_dialog);
    }

    extern "stdcall" fn item_symbol_init_intercept(item: &mut TaskData) {
        let item_symbol_init: &*const () = APPLICATION.get_address(ITEM_SYMBOL_INIT_ADDRESS);
        let item_symbol_init_func: extern "C" fn(&TaskData) = unsafe { std::mem::transmute(item_symbol_init) };
        (item_symbol_init_func)(item);
        item.rfunc = Self::item_symbol_back_intercept as EventWithBool;
    }

    fn item_symbol_back_intercept(item: &mut TaskData) -> u32 {
        let acquired = item.hit_data > 0;
        let item_id = item.buff[1];

        if acquired {
            // Hardcoded to assume item is for other player for now
            item.sbuff[2] = 0;
        }

        let item_symbol_back: &*const () = APPLICATION.get_address(ITEM_SYMBOL_BACK_ADDRESS);
        let item_symbol_back_func: extern "C" fn(&TaskData) -> u32 = unsafe { std::mem::transmute(item_symbol_back) };
        let result = (item_symbol_back_func)(item);

        if acquired {
            let player_item = PlayerItem {
                player_id: APPLICATION.app_config.buddy_id,
                for_player: true
            };

            {
                let mut player_item_option = PLAYER_ITEM.lock().unwrap();
                *player_item_option = Some(player_item);
            }

            APPLICATION.create_dialog_popup(item_id as u32);

            APPLICATION.randomizer.send_message(RandomizerMessage {
                player_id: APPLICATION.app_config.buddy_id,
                item_id
            });
        }

        result
    }

    fn create_dialog_popup(&self, item_id: u32) {
        self.option_stuck(item_id);

        let popup_dialog_init: *const usize = self.get_address(POPUP_DIALOG_INIT_ADDRESS);
        let set_task: &*const () = self.get_address(SET_VIEW_EVENT_NS_ADDRESS);
        let set_task_func: extern "C" fn(u16, *const usize) -> *const TaskData = unsafe { std::mem::transmute(set_task) };
        (set_task_func)(16, popup_dialog_init);

        self.pause_game_process();
        self.set_lemeza_item_pose();
        self.disable_warp_menu();
        self.disable_movement();
        self.play_sound_effect(0x618);
    }

    fn pause_game_process(&self) {
        let val: &mut u32 = self.get_address(GAME_PROCESS_ADDRESS);
        *val |= 2;
    }

    fn disable_movement(&self) {
        let val: &mut u32 = self.get_address(MOVEMENT_STATUS_ADDRESS);
        *val |= 1;
    }

    fn disable_warp_menu(&self) {
        let val: &mut u32 = self.get_address(WARP_MENU_STATUS_ADDRESS);
        *val |= 0x100000;
    }

    fn set_lemeza_item_pose(&self) {
        let lemeza_address: &mut usize = self.get_address(LEMEZA_ADDRESS);
        let lemeza: &mut TaskData = self.get_address(*lemeza_address);
        (*lemeza).sbuff[6] = 0xf;
    }

    fn play_sound_effect(&self, effect_id: u32) {
        let se_address: &mut u32 = self.get_address(SE_ADDRESS);
        let set_se: &*const () = self.get_address(SET_SE_ADDRESS);
        let set_se_func: extern "C" fn(u32, u32, u32, u32, u32, u32) = unsafe { std::mem::transmute(set_se) };
        (set_se_func)(*se_address + effect_id,0x27,0xf,0x3f499326,0,0x3f000000);
    }

    fn option_stuck(&self, option_num: u32) {
        let s_data_num: &mut u8 = self.get_address(OPTION_SDATA_NUM_ADDRESS);
        if *s_data_num < 32 {
            let s_data: &mut [u32;32] = self.get_address(OPTION_SDATA_ADDRESS);
            s_data[*s_data_num as usize] = option_num;
            *s_data_num = *s_data_num + 1
        }
    }

    fn option_pos(&self, x: f32, y: f32) {
        *self.get_address(OPTION_POS_CX_ADDRESS) = x;
        *self.get_address(OPTION_POS_CY_ADDRESS) = y;
    }

    pub fn get_address<T>(&self, offset: usize) -> &mut T {
        unsafe {
            let addr: usize = std::mem::transmute(self.address.wrapping_add(offset));
            &mut*(addr as *mut T)
        }
    }
}
