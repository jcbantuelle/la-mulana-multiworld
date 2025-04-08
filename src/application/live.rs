use crate::{LiveApplication, Application, get_application_version};
use crate::application::{ADDRESS_LOOKUP, AppAddresses, ApplicationMemoryOps};
use crate::lm_structs::taskdata::TaskData;
use crate::application::entrypoints::{game_loop, popup_dialog_draw_intercept, FnGameLoop, FnPopupDialogDrawIntercept};

use log::error;
use retour::{Function, static_detour, StaticDetour};
use crate::utils::show_message_box;

static_detour! {
    static GameLoopDetour: extern "C" fn();
}

static_detour! {
    static PopupDialogDrawInterceptDetour: extern "C" fn(&'static mut TaskData);
}

static_detour! {
    static ItemSymbolInitInterceptDetour: extern "C" fn(&'static mut TaskData);
}

impl Application for LiveApplication {
    fn attach(&self) {
        let version = get_application_version();

        if let Some(app_addresses) = ADDRESS_LOOKUP.get(&version) {
            unsafe {
                let game_loop_addr: FnGameLoop = std::mem::transmute(self.get_address().wrapping_add(app_addresses.game_loop_address));
                let _ = self.enable_detour(GameLoopDetour.initialize(game_loop_addr, game_loop), "GameLoopDetour");

                let popup_dialog_draw_intercept_addr: FnPopupDialogDrawIntercept = std::mem::transmute(self.get_address().wrapping_add(app_addresses.popup_dialog_draw_address));
                let _ = self.enable_detour(PopupDialogDrawInterceptDetour.initialize(popup_dialog_draw_intercept_addr, popup_dialog_draw_intercept), "PopupDialogDrawInterceptDetour");
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

    fn popup_dialog_draw(&self, popup_dialog: &'static mut TaskData) {
        PopupDialogDrawInterceptDetour.call(popup_dialog);
    }

    fn original_game_loop(&self) {
        GameLoopDetour.call()
    }

    fn app_addresses(&self) -> &AppAddresses {
        // Okay to unwrap here with version vetted at DLL load
        ADDRESS_LOOKUP.get(&self.app_version).unwrap()
    }
}

impl LiveApplication {
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
}

impl ApplicationMemoryOps for LiveApplication {
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
