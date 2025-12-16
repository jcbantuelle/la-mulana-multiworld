pub mod entrypoints;
pub mod live;

use phf::phf_map;
use std::sync::Mutex;

use crate::archipelago::api::APError;
use crate::archipelago::client::APClient;
use crate::AppConfig;
use crate::lm_structs::taskdata::TaskData;

pub struct AppAddresses {
    pub popup_dialog_draw: usize,
    pub game_init: usize,
    pub lemeza_pointer: usize,
    pub game_process: usize,
    pub option_sdata_num: usize,
    pub option_sdata: usize,
    pub option_pos_cx: usize,
    pub option_pos_cy: usize,
    pub set_view_event_ns: usize,
    pub set_se: usize,
    pub se: usize,
    pub item_get_area_init: usize,
    pub popup_dialog_init: usize,
    pub script_header_pointer: usize,
    pub item_symbol_init: usize,
    pub item_symbol_back: usize,
    pub global_flags: usize,
    pub inventory_words: usize,
    pub current_field: usize,
    pub current_screen: usize,
    pub current_scene: usize,
    pub system_flags: usize,
    pub game_loop: usize
}

const ADDRESS_LOOKUP: phf::Map<&'static str, AppAddresses> = phf_map! {
    "1.0.0.1" => AppAddresses {
        set_se:                 0x00417600,
        item_get_area_init:     0x004b8950,
        item_symbol_init:       0x004b8ae0,
        item_symbol_back:       0x004b8e70,
        set_view_event_ns:      0x00507160,
        popup_dialog_init:      0x00591520,
        popup_dialog_draw:      0x005917b0,
        game_loop:              0x00607b70,
        se:                     0x006d2708,
        script_header_pointer:  0x006d296c,
        inventory_words:        0x006d5650,
        system_flags:           0x006d59c0,
        global_flags:           0x006d5a70,
        current_scene:          0x00db4bb3,
        current_screen:         0x00db4bb6,
        current_field:          0x00db4bb7,
        option_sdata_num:       0x00db6fb7,
        option_sdata:           0x00db7048,
        option_pos_cx:          0x00db714c,
        option_pos_cy:          0x00db7168,
        game_process:           0x00db7178,
        lemeza_pointer:         0x00db7538,
        game_init:              0x00db753c
    },
    "1.6.6.2" => AppAddresses {
        set_se:                 0x004186c0,
        item_get_area_init:     0x004ba720,
        item_symbol_init:       0x004ba8b0,
        item_symbol_back:       0x004bac40,
        set_view_event_ns:      0x00509530,
        popup_dialog_init:      0x00593670,
        popup_dialog_draw:      0x00593900,
        game_loop:              0x00609fb0,
        se:                     0x006de844,
        script_header_pointer:  0x006deb2c,
        inventory_words:        0x006e1820,
        system_flags:           0x006e1b90,
        global_flags:           0x006e1e48,
        current_scene:          0x00dc0ebe,
        current_screen:         0x00dc0ebf,
        current_field:          0x00dc0ee6,
        option_sdata_num:       0x00dc32c2,
        option_sdata:           0x00dc3350,
        option_pos_cx:          0x00dc3454,
        option_pos_cy:          0x00dc3470,
        game_process:           0x00dc3480,
        lemeza_pointer:         0x00dc3844,
        game_init:              0x00dc3848
    }
};

pub trait Application {
    fn attach(&self);
    fn get_address(&self) -> usize;
    fn get_randomizer(&self) -> &Mutex<Result<APClient, APError>>;
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
    fn original_game_loop(&self);
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

