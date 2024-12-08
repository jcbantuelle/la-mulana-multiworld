use std::sync::Mutex;

use crate::archipelago::client::{ArchipelagoClient, ArchipelagoError};
use crate::{AppConfig, LiveApplication, Application};
use crate::application::{ApplicationMemoryOps, GAME_LOOP_ADDRESS, GAME_PROCESS_ADDRESS, ITEM_GET_AREA_INIT_ADDRESS, ITEM_SYMBOL_INIT_ADDRESS, ITEM_SYMBOL_INIT_INTERCEPT, ITEM_SYMBOL_INIT_POINTER_ADDRESS, LEMEZA_ADDRESS, MOVEMENT_STATUS_ADDRESS, OPTION_POS_CX_ADDRESS, OPTION_POS_CY_ADDRESS, OPTION_SDATA_ADDRESS, OPTION_SDATA_NUM_ADDRESS, POPUP_DIALOG_DRAW_ADDRESS, POPUP_DIALOG_DRAW_INTERCEPT, POPUP_DIALOG_INIT_ADDRESS, SE_ADDRESS, SET_SE_ADDRESS, SET_VIEW_EVENT_NS_ADDRESS, WARP_MENU_STATUS_ADDRESS};
use crate::lm_structs::taskdata::TaskData;
use crate::application::entrypoints::{item_symbol_init_intercept, game_loop, popup_dialog_draw_intercept, FnGameLoop, FnPopupDialogDrawIntercept, FnItemSymbolInitIntercept};
use winapi::shared::ntdef::INT;

use std::error::Error;
use std::marker::Tuple;
use std::mem;
use log::{debug, error, trace};
use retour::{Function, static_detour, StaticDetour};
use winapi::shared::minwindef::DWORD;
use crate::utils::show_message_box;

static_detour! {
    static GameLoopDetour: extern "stdcall" fn() -> DWORD;
}

static_detour! {
    static PopupDialogDrawInterceptDetour: extern "C" fn(&'static TaskData);
}

static_detour! {
    static ItemSymbolInitInterceptDetour: extern "C" fn(&'static mut TaskData);
}

impl Application for LiveApplication {
    fn attach(&self) {
        unsafe {
            let game_loop_addr: FnGameLoop = std::mem::transmute(self.get_address().wrapping_add(GAME_LOOP_ADDRESS));
            let _ = self.enable_detour(GameLoopDetour.initialize(game_loop_addr, game_loop), "GameLoopDetour");

            let popup_dialog_draw_intercept_addr: FnPopupDialogDrawIntercept = std::mem::transmute(self.get_address().wrapping_add(POPUP_DIALOG_DRAW_ADDRESS));
            let _ = self.enable_detour(PopupDialogDrawInterceptDetour.initialize(popup_dialog_draw_intercept_addr, popup_dialog_draw_intercept), "PopupDialogDrawInterceptDetour");

            let item_symbol_init_intercept_addr: FnItemSymbolInitIntercept = std::mem::transmute(self.get_address().wrapping_add(ITEM_SYMBOL_INIT_ADDRESS));
            let _ = self.enable_detour(ItemSymbolInitInterceptDetour.initialize(item_symbol_init_intercept_addr, item_symbol_init_intercept), "ItemSymbolInitInterceptDetour");
        }
    }

    fn get_address(&self) -> usize {
        self.address
    }

    fn get_randomizer(&self) -> &Mutex<Result<ArchipelagoClient, ArchipelagoError>> {
        &self.randomizer
    }

    fn get_app_config(&self) -> &AppConfig {
        &self.app_config
    }

    fn give_item(&self, item: u32) {
        self.option_pos(0.0, 0.0);
        self.option_stuck(item);
        self.option_stuck(160);
        self.option_stuck(120);
        self.option_stuck(39);

        let item_get_area_init: *const usize = self.read_address(ITEM_GET_AREA_INIT_ADDRESS);
        let set_view_event_ns: &*const () = self.read_address(SET_VIEW_EVENT_NS_ADDRESS);
        let set_view_event_ns_func: extern "C" fn(u16, *const usize) -> *const TaskData = unsafe { std::mem::transmute(set_view_event_ns) };
        (set_view_event_ns_func)(16, item_get_area_init);
    }

    fn create_dialog_popup(&self, item_id: u32) {
        self.option_stuck(item_id);

        let popup_dialog_init: *const usize = self.read_address(POPUP_DIALOG_INIT_ADDRESS);
        let set_task: &*const () = self.read_address(SET_VIEW_EVENT_NS_ADDRESS);
        let set_task_func: extern "C" fn(u16, *const usize) -> *const TaskData = unsafe { std::mem::transmute(set_task) };
        (set_task_func)(16, popup_dialog_init);

        self.pause_game_process();
        self.set_lemeza_item_pose();
        self.disable_warp_menu();
        self.disable_movement();
        self.play_sound_effect(0x618);
    }

    fn popup_dialog_draw(&self, popup_dialog: &'static TaskData) {
        PopupDialogDrawInterceptDetour.call(popup_dialog);
    }

    fn pause_game_process(&self) {
        let val: &mut u32 = self.read_address(GAME_PROCESS_ADDRESS);
        *val |= 2;
    }

    fn disable_movement(&self) {
        let val: &mut u32 = self.read_address(MOVEMENT_STATUS_ADDRESS);
        *val |= 1;
    }

    fn disable_warp_menu(&self) {
        let val: &mut u32 = self.read_address(WARP_MENU_STATUS_ADDRESS);
        *val |= 0x100000;
    }

    fn set_lemeza_item_pose(&self) {
        let lemeza_address: &mut usize = self.read_address(LEMEZA_ADDRESS);
        let lemeza: &mut TaskData = self.read_address(*lemeza_address);
        (*lemeza).sbuff[6] = 0xf;
    }

    fn play_sound_effect(&self, effect_id: u32) {
        let se_address: &mut u32 = self.read_address(SE_ADDRESS);
        let set_se: &*const () = self.read_address(SET_SE_ADDRESS);
        let set_se_func: extern "C" fn(u32, u32, u32, u32, u32, u32) = unsafe { std::mem::transmute(set_se) };
        (set_se_func)(*se_address + effect_id,0x27,0xf,0x3f499326,0,0x3f000000);
    }

    fn option_stuck(&self, option_num: u32) {
        let s_data_num: &mut u8 = self.read_address(OPTION_SDATA_NUM_ADDRESS);
        if *s_data_num < 32 {
            let s_data: &mut [u32;32] = self.read_address(OPTION_SDATA_ADDRESS);
            s_data[*s_data_num as usize] = option_num;
            *s_data_num = *s_data_num + 1
        }
    }

    fn option_pos(&self, x: f32, y: f32) {
        *self.read_address(OPTION_POS_CX_ADDRESS) = x;
        *self.read_address(OPTION_POS_CY_ADDRESS) = y;
    }

    fn original_item_symbol_init(&self, item: &'static mut TaskData) {
        debug!("live.application.original_item_symbol_item called!");
        ItemSymbolInitInterceptDetour.call(item)
    }
}

impl LiveApplication {
    unsafe fn enable_detour<'a, T: Function>(&self, detour_result: Result<&'a StaticDetour<T>, retour::Error>, detour_name: &str) -> &'a StaticDetour<T> {
        match detour_result {
            Ok(e) => {
                match e.enable() {
                    Ok(_) => {
                        debug!("Detour is enabled for {}", detour_name);
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
}

impl ApplicationMemoryOps for LiveApplication {
    fn read_address<T>(&self, offset: usize) -> &mut T {
        unsafe {
            let addr: usize = std::mem::transmute(self.get_address().wrapping_add(offset));
            &mut*(addr as *mut T)
        }
    }
}

