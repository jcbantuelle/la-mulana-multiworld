pub mod entrypoints;
pub mod live;

use std::sync::Mutex;

use crate::AppConfig;
use crate::network::Randomizer;
use crate::lm_structs::taskdata::TaskData;

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
pub static GLOBAL_FLAGS_ADDRESS: usize = 0x006d5a70;
pub static INVENTORY_ADDRESS: usize = 0x006d4db4;

pub trait Application {
    fn attach(&self);
    fn get_address(&self) -> usize;
    fn get_randomizer(&self) -> &dyn Randomizer;
    fn get_app_config(&self) -> &AppConfig;
    fn give_item(&self, item: u32);
    fn create_dialog_popup(&self, item_id: u32);
    fn popup_dialog_draw(&self, popup_dialog: &TaskData);
    fn pause_game_process(&self);
    fn disable_movement(&self);
    fn disable_warp_menu(&self);
    fn set_lemeza_item_pose(&self);
    fn play_sound_effect(&self, effect_id: u32);
    fn option_stuck(&self, option_num: u32);
    fn option_pos(&self, x: f32, y: f32);
}

pub trait ApplicationMemoryOps {
    fn read_address<V>(&self, offset: usize) -> &mut V;
}

impl ApplicationMemoryOps for Box<dyn Application + Sync> {
    fn read_address<T>(&self, offset: usize) -> &mut T {
        unsafe {
            let addr: usize = std::mem::transmute(self.get_address().wrapping_add(offset));
            &mut*(addr as *mut T)
        }
    }
}

