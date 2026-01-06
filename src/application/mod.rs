pub mod entrypoints;

use log::{debug, error, trace};
use retour::{Function, static_detour, StaticDetour};
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use crate::AppConfig;
use crate::application::entrypoints::{item_symbol_init_intercept, game_loop, popup_dialog_draw_intercept, FnGameLoop, FnPopupDialogDrawIntercept, FnItemSymbolInitIntercept};
use crate::archipelago::api::APError;
use crate::archipelago::client::APClient;
use crate::lm_structs::taskdata::TaskData;
use crate::utils::show_message_box;

static_detour! {
    static GameLoopDetour: extern "C" fn();
}

static_detour! {
    static PopupDialogDrawInterceptDetour: extern "C" fn(&'static TaskData);
}

static_detour! {
    static ItemSymbolInitInterceptDetour: extern "C" fn(&'static mut TaskData);
}

pub struct Application {
    pub address: usize,
    pub randomizer: Mutex<Result<APClient, APError>>,
    pub app_config: AppConfig,
    pub app_version: String
}

impl Application {
    pub fn attach(&self) {
        let version = self.application_version();

        if let Some(_) = Application::ADDRESS_LOOKUP.get(version) {
            unsafe {
                let game_loop_addr: FnGameLoop = std::mem::transmute(self.extract_offset("game_loop"));
                let _ = self.enable_detour(GameLoopDetour.initialize(game_loop_addr, game_loop), "GameLoopDetour");

                let popup_dialog_draw_intercept_addr: FnPopupDialogDrawIntercept = std::mem::transmute(self.extract_offset("popup_dialog_draw"));
                let _ = self.enable_detour(PopupDialogDrawInterceptDetour.initialize(popup_dialog_draw_intercept_addr, popup_dialog_draw_intercept), "PopupDialogDrawInterceptDetour");

                let item_symbol_init_intercept_addr: FnItemSymbolInitIntercept = std::mem::transmute(self.extract_offset("item_symbol_init"));
                let _ = self.enable_detour(ItemSymbolInitInterceptDetour.initialize(item_symbol_init_intercept_addr, item_symbol_init_intercept), "ItemSymbolInitInterceptDetour");

                trace!("Enabled all detours.");
            }
        }
        else {
            let error_message = format!("Unsupported version {}.", version);
            show_message_box(&error_message);
        }
    }

    fn get_address(&self) -> usize {
        self.address
    }

    fn get_randomizer(&self) -> &Mutex<Result<APClient, APError>> {
        &self.randomizer
    }

    fn get_app_config(&self) -> &AppConfig {
        &self.app_config
    }

    fn give_item(&self, item: u32) {
        debug!("Giving item ID {}", item);

        self.option_pos(0.0, 0.0);
        self.option_stuck(item);
        self.option_stuck(160);
        self.option_stuck(120);
        self.option_stuck(39);
        let item_get_area_init: *const usize = self.read_address("item_get_area_init");
        let set_view_event_ns: &*const () = self.read_address("set_view_event_ns");
        let set_view_event_ns_func: extern "C" fn(u16, *const usize) -> *const TaskData = unsafe { std::mem::transmute(set_view_event_ns) };
        (set_view_event_ns_func)(16, item_get_area_init);
        trace!("set_view_event_ns_func called");
    }

    fn create_dialog_popup(&self, item_id: u32) {
        debug!("Creating dialog popup for item ID {}", item_id);

        self.option_stuck(item_id);
        let popup_dialog_init: *const usize = self.read_address("popup_dialog_init");
        let set_task: &*const () = self.read_address("set_view_event_ns");
        let set_task_func: extern "C" fn(u16, *const usize) -> *const TaskData = unsafe { std::mem::transmute(set_task) };
        (set_task_func)(16, popup_dialog_init);
        trace!("Called popup_dialog_init for item ID {}", item_id);

        self.pause_game_process();
        trace!("Pause game process for item ID {}", item_id);
        self.set_lemeza_item_pose();
        trace!("Set item pose for item ID {}", item_id);
        self.disable_warp_menu();
        trace!("Disabled warp menu for item ID {}", item_id);
        self.disable_movement();
        trace!("Disabled movement for item ID {}", item_id);
        self.play_sound_effect(0x618);
        trace!("Played sound effect for item ID {}", item_id);
    }

    fn popup_dialog_draw(&self, popup_dialog: &'static TaskData) {
        PopupDialogDrawInterceptDetour.call(popup_dialog)
    }

    fn pause_game_process(&self) {
        let val: &mut u32 = self.read_address("game_process");
        *val |= 2;
    }

    fn disable_movement(&self) {
        let system_flags: &mut [u32;16] = self.read_address("system_flags");
        system_flags[0] |= 1;
    }

    fn disable_warp_menu(&self) {
        let system_flags: &mut [u32;16] = self.read_address("system_flags");
        system_flags[3] |= 0x100000;
    }

    fn set_lemeza_item_pose(&self) {
        let lemeza_address: &mut usize = self.read_address("lemeza_pointer");
        let lemeza: &mut TaskData = self.read_raw_address(*lemeza_address);
        (*lemeza).sbuff[6] = 0xf;
    }

    fn play_sound_effect(&self, effect_id: u32) {
        let se_address: &mut u32 = self.read_address("se");
        let set_se: &*const () = self.read_address("set_se");
        let set_se_func: extern "C" fn(u32, u32, u32, u32, u32, u32) = unsafe { std::mem::transmute(set_se) };
        (set_se_func)(*se_address + effect_id,0x27,0xf,0x3f499326,0,0x3f000000);
    }

    fn option_stuck(&self, option_num: u32) {
        let s_data_num: &mut u8 = self.read_address("option_sdata_num");
        if *s_data_num < 32 {
            let s_data: &mut [u32;32] = self.read_address("option_sdata");
            s_data[*s_data_num as usize] = option_num;
            *s_data_num = *s_data_num + 1
        }
    }

    fn option_pos(&self, x: f32, y: f32) {
        *self.read_address("option_pos_cx") = x;
        *self.read_address("option_pos_cy") = y;
    }

    fn original_item_symbol_init(&self, item: &'static mut TaskData) {
        ItemSymbolInitInterceptDetour.call(item)
    }

    fn original_game_loop(&self) {
        GameLoopDetour.call()
    }

    fn application_version(&self) -> &str {
        &self.app_version
    }

    unsafe fn enable_detour<'a, T: Function>(&self, detour_result: Result<&'a StaticDetour<T>, retour::Error>, detour_name: &str) -> &'a StaticDetour<T> {
        match detour_result {
            Ok(e) => {
                match e.enable() {
                    Ok(_) => {
                        e
                    },
                    Err(e) => {
                        let error_message = format!("Error enabling detour {}: {}", detour_name, e);
                        error!("{}", error_message);
                        panic!("{}", error_message)
                    }
                }
            },
            Err(e) => {
                let error_message = format!("Error attaching to detour {}: {}", detour_name, e);
                error!("{}", error_message);
                panic!("{}", error_message)
            }
        }
    }

    fn read_address<T>(&self, offset_name: &str) -> &mut T {
        let address = self.extract_offset(offset_name);
        self.read_raw_address(address)
    }

    fn read_raw_address<T>(&self, address: usize) -> &mut T {
        unsafe {
            let addr: usize = std::mem::transmute(address);
            &mut*(addr as *mut T)
        }
    }

    fn extract_offset(&self, offset_name: &str) -> usize {
        let version = self.application_version();
        let addresses = Application::ADDRESS_LOOKUP;
        let address = addresses.get(version).unwrap();
        let offset = address.get(&offset_name).unwrap();
        self.get_address().wrapping_add(*offset)
    }

    pub const ADDRESS_LOOKUP: LazyLock<HashMap<&str, HashMap<&str, usize>>> = LazyLock::new(|| {
        let version_1_0_0_1 = HashMap::from([
            ("set_se",                0x00417600),
            ("item_get_area_init",    0x004b8950),
            ("item_symbol_init",      0x004b8ae0),
            ("item_symbol_back",      0x004b8e70),
            ("set_view_event_ns",     0x00507160),
            ("popup_dialog_init",     0x00591520),
            ("popup_dialog_draw",     0x005917b0),
            ("game_loop",             0x00607b70),
            ("se",                    0x006d2708),
            ("script_header_pointer", 0x006d296c),
            ("inventory_words",       0x006d5650),
            ("system_flags",          0x006d59c0),
            ("global_flags",          0x006d5a70),
            ("current_scene",         0x00db4bb3),
            ("current_screen",        0x00db4bb6),
            ("current_field",         0x00db4bb7),
            ("option_sdata_num",      0x00db6fb7),
            ("option_sdata",          0x00db7048),
            ("option_pos_cx",         0x00db714c),
            ("option_pos_cy",         0x00db7168),
            ("game_process",          0x00db7178),
            ("lemeza_pointer",        0x00db7538),
            ("game_init",             0x00db753c)
        ]);
        let version_1_6_6_2 = HashMap::from([
            ("set_se",                0x004186c0),
            ("item_get_area_init",    0x004ba720),
            ("item_symbol_init",      0x004ba8b0),
            ("item_symbol_back",      0x004bac40),
            ("set_view_event_ns",     0x00509530),
            ("popup_dialog_init",     0x00593670),
            ("popup_dialog_draw",     0x00593900),
            ("game_loop",             0x00609fb0),
            ("se",                    0x006de844),
            ("script_header_pointer", 0x006deb2c),
            ("inventory_words",       0x006e1820),
            ("system_flags",          0x006e1b90),
            ("global_flags",          0x006e1e48),
            ("current_scene",         0x00dc0ebe),
            ("current_screen",        0x00dc0ebf),
            ("current_field",         0x00dc0ee6),
            ("option_sdata_num",      0x00dc32c2),
            ("option_sdata",          0x00dc3350),
            ("option_pos_cx",         0x00dc3454),
            ("option_pos_cy",         0x00dc3470),
            ("game_process",          0x00dc3480),
            ("lemeza_pointer",        0x00dc3844),
            ("game_init",             0x00dc3848)
        ]);
        HashMap::from([
            ("1.0.0.1", version_1_0_0_1),
            ("1.6.6.2", version_1_6_6_2)
        ])
    });
}
