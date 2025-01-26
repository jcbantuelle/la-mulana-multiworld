pub mod entrypoints;
pub mod live;

use std::sync::Mutex;
use crate::AppConfig;
use crate::archipelago::client::{ArchipelagoClient, ArchipelagoError};
use crate::lm_structs::taskdata::TaskData;
use phf::phf_map;

pub struct AppAddresses {
    pub game_loop_address: usize,
    pub popup_dialog_draw_address: usize,
    pub game_init_address: usize,
    pub lemeza_address: usize,
    pub game_process_address: usize,
    pub warp_menu_status_address: usize,
    pub movement_status_address: usize,
    pub option_sdata_num_address: usize,
    pub option_sdata_address: usize,
    pub option_pos_cx_address: usize,
    pub option_pos_cy_address: usize,
    pub set_view_event_ns_address: usize,
    pub set_se_address: usize,
    pub se_address: usize,
    pub item_get_area_init_address: usize,
    pub popup_dialog_init_address: usize,
    pub script_header_pointer_address: usize,
    pub item_symbol_init_address: usize,
    pub item_symbol_back_address: usize,
    pub global_flags_address: usize,
    pub inventory_words: usize
}

const ADDRESS_LOOKUP: phf::Map<&'static str, AppAddresses> = phf_map! {
    "1.0.0.1" => AppAddresses {
        game_loop_address: 0x0066f1c0,
        popup_dialog_draw_address: 0x005917b0,
        game_init_address: 0x00db753c,
        lemeza_address: 0x00db7538,
        game_process_address: 0x00db7178,
        warp_menu_status_address: 0x006d59cc,
        movement_status_address: 0x006d59c0,
        option_sdata_num_address: 0x00db6fb7,
        option_sdata_address: 0x00db7048,
        option_pos_cx_address: 0x00db7168,
        option_pos_cy_address: 0x00db714c,
        set_view_event_ns_address: 0x00507160,
        set_se_address: 0x00417600,
        se_address: 0x006d2708,
        item_get_area_init_address: 0x004b8950,
        popup_dialog_init_address: 0x00591520,
        script_header_pointer_address: 0x006d296c,
        item_symbol_init_address: 0x004b8ae0,
        item_symbol_back_address: 0x004b8e70,
        global_flags_address: 0x006d5a70,
        inventory_words: 0x006d5650,
    },
    "1.6.6.2" => AppAddresses {
        game_loop_address: 0x006714a0,
        popup_dialog_draw_address: 0x00593900,
        game_init_address: 0x00dc3848,
        lemeza_address: 0x00dc3844,
        game_process_address: 0x00dc3480,
        warp_menu_status_address: 0x006e1b9c,
        movement_status_address: 0x006e1b90,
        option_sdata_num_address: 0x00dc32c2,
        option_sdata_address: 0x00dc3350,
        option_pos_cx_address: 0x00dc3470,
        option_pos_cy_address: 0x0dc3454,
        set_view_event_ns_address: 0x00509530,
        set_se_address: 0x004186c0,
        se_address: 0x006de844,
        item_get_area_init_address: 0x004ba720,
        popup_dialog_init_address: 0x00593670,
        script_header_pointer_address: 0x006deb2c,
        item_symbol_init_address: 0x004ba8b0,
        item_symbol_back_address: 0x004bac40,
        global_flags_address: 0x006e1e48,
        inventory_words: 0x006e1820,
    }
};

pub trait Application {
    fn attach(&self);
    fn get_address(&self) -> usize;
    fn get_randomizer(&self) -> &Mutex<Result<ArchipelagoClient, ArchipelagoError>>;
    fn get_app_config(&self) -> &AppConfig;
    fn give_item(&self, item: u32);
    fn create_dialog_popup(&self, item_id: u32);
    fn popup_dialog_draw(&self, popup_dialog: &'static TaskData);
    fn pause_game_process(&self);
    fn disable_movement(&self);
    fn disable_warp_menu(&self);
    fn set_lemeza_item_pose(&self);
    fn play_sound_effect(&self, effect_id: u32);
    fn option_stuck(&self, option_num: u32);
    fn option_pos(&self, x: f32, y: f32);
    fn original_item_symbol_init(&self, item: &'static mut TaskData);
    fn app_addresses(&self) -> &AppAddresses;
}

pub trait ApplicationMemoryOps {
    fn read_address<V>(&self, offset: usize) -> &mut V;
    fn read_raw_address<V>(&self, address: usize) -> &mut V;
}

impl ApplicationMemoryOps for Box<dyn Application + Sync> {
    fn read_address<T>(&self, offset: usize) -> &mut T {
        unsafe {
            let addr: usize = std::mem::transmute(self.get_address().wrapping_add(offset));
            &mut*(addr as *mut T)
        }
    }

    fn read_raw_address<T>(&self, address: usize) -> &mut T {
        unsafe {
            let addr: usize = std::mem::transmute(address);
            &mut*(addr as *mut T)
        }
    }
}

