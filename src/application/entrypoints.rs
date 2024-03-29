use log::debug;
use std::sync::Mutex;
use lazy_static::lazy_static;

use winapi::shared::minwindef::*;
use winapi::um::timeapi::timeGetTime;
use winapi::um::processthreadsapi::ExitProcess;

use crate::{APPLICATION, Application, get_application};
use crate::application::{ApplicationMemoryOps, GAME_INIT_ADDRESS, GLOBAL_FLAGS_ADDRESS, ITEM_SYMBOL_BACK_ADDRESS, ITEM_SYMBOL_INIT_ADDRESS, SCRIPT_HEADER_POINTER_ADDRESS};
use crate::lm_structs::items::{generate_item_translator};
use crate::utils::show_message_box;
use crate::network::{RandomizerMessage, serialize_message};
use crate::lm_structs::taskdata::TaskData;
use crate::lm_structs::taskdata::EventWithBool;
use crate::lm_structs::script_header::{ScriptHeader, ScriptSubHeader};
use crate::screenplay;

#[derive(Debug)]
pub struct GivenItem {
    pub player_id: i32,
    pub item_id: u32
}

pub struct PlayerItem {
    pub player_id: i32,
    pub for_player: bool
}

pub struct PlayerItemPopup {
    pub popup_id_address: usize,
    pub popup_id: u32,
    pub encoded: Vec<u16>,
    pub line_address: usize,
    pub old_line: ScriptSubHeader,
}

lazy_static! {
    static ref ITEMS_TO_GIVE: Mutex<Vec<GivenItem>> = Mutex::new(vec![]);
    static ref PLAYER_ITEM: Mutex<Option<PlayerItem>> = Mutex::new(None);
    static ref PLAYER_ITEM_POPUP: Mutex<Option<PlayerItemPopup>> = Mutex::new(None);
}

pub extern "stdcall" fn app_init(patch_version: winapi::shared::ntdef::INT) {
    if patch_version != 1 {
        let init_message = format!("EXE Patch Version does not match DLL. Please re-patch.");
        show_message_box(&init_message);
        unsafe {
            ExitProcess(1);
        }
    }
}

pub extern "stdcall" fn game_loop() -> DWORD {
    let application = get_application();
    let game_init: &mut u32 = application.read_address(GAME_INIT_ADDRESS);
    if *game_init != 0 {
        let _ = application.get_randomizer().read_messages().map(|payload| {
            let mut items_to_give = ITEMS_TO_GIVE.lock().unwrap();
            let global_flags: &[u8;2055] = application.read_address(GLOBAL_FLAGS_ADDRESS);
            let global_item_lookup = generate_item_translator();

            /* We have to do the diff here and see what items the player really should get */

            for item in payload.message.items {
                let player_id = item.player_id;
                if let Some(global_flag_id) = global_item_lookup.get(&item.item_id) {
                    let global_flag_id = global_flag_id.index;
                    if global_flags[global_flag_id as usize] != 255 {
                        items_to_give.push(GivenItem {
                            player_id: player_id as i32,
                            item_id: item.item_id as u32
                        });
                    }
                }

                debug!("Received item {} from player ID {}.", item.item_id, player_id);
            }
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
                    application.give_item(next_item.item_id);
                }
            }
        }

        if let Some(popup_option) = PLAYER_ITEM_POPUP.try_lock().ok().as_mut() {
            if let Some(popup) = popup_option.as_ref() {
                if popup.popup_id != *application.read_address::<u32>(popup.popup_id_address) {
                    *application.read_address::<ScriptSubHeader>(popup.line_address) = popup.old_line;
                    **popup_option = None;
                }
            }
        }
    }

    get_time()
}

pub extern "stdcall" fn popup_dialog_draw_intercept(popup_dialog: &TaskData) {
    let application = get_application();
    let mut player_item_option = PLAYER_ITEM.lock().unwrap();
    if let Some(player_item) = player_item_option.as_ref() {
        let script_header: &*const ScriptHeader = application.read_address(SCRIPT_HEADER_POINTER_ADDRESS);
        let line_header = unsafe { (*script_header.add(3)).data as *mut ScriptSubHeader};
        let line = unsafe { &mut *line_header.add(2) };

        let item_for_text = if player_item.for_player { "For" } else { "From" };
        let player_name = application.get_app_config().players.get(&player_item.player_id).unwrap();

        let popup = PlayerItemPopup {
            popup_id_address: &popup_dialog.id.uid as *const u32 as usize,
            popup_id: popup_dialog.id.uid,
            encoded: screenplay::encode(format!("  {} {}", item_for_text, player_name)),
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

        application.popup_dialog_draw(popup_dialog);

        *player_item_option = None;
    } else {
        application.popup_dialog_draw(popup_dialog);
    }
}

pub extern "stdcall" fn item_symbol_init_intercept(item: &mut TaskData) {
    let item_symbol_init: &*const () = get_application().read_address(ITEM_SYMBOL_INIT_ADDRESS);
    let item_symbol_init_func: extern "C" fn(&TaskData) = unsafe { std::mem::transmute(item_symbol_init) };
    (item_symbol_init_func)(item);
    item.rfunc = item_symbol_back_intercept as EventWithBool;
}

pub fn item_symbol_back_intercept(item: &mut TaskData) -> u32 {
    let acquired = item.hit_data > 0;
    let item_id = item.buff[1];
    let chest: &mut TaskData = get_application().read_address(item.addr[0]);
    let player_id_for_item = chest.sbuff[6];
    let item_for_other = player_id_for_item != APPLICATION.get_app_config().user_id;

    if acquired && item_for_other {
        item.sbuff[2] = 0;
    }

    let item_symbol_back: &*const () = APPLICATION.read_address(ITEM_SYMBOL_BACK_ADDRESS);
    let item_symbol_back_func: extern "C" fn(&TaskData) -> u32 = unsafe { std::mem::transmute(item_symbol_back) };
    let result = (item_symbol_back_func)(item);

    if acquired && item_for_other {
        let player_item = PlayerItem {
            player_id: player_id_for_item,
            for_player: true
        };

        {
            let mut player_item_option = PLAYER_ITEM.lock().unwrap();
            *player_item_option = Some(player_item);
        }

        let global_flags: &[u8;2055] = APPLICATION.read_address(GLOBAL_FLAGS_ADDRESS);
        APPLICATION.create_dialog_popup(item_id as u32);
        APPLICATION.get_randomizer().send_message(&serialize_message (
            RandomizerMessage {
                player_id: APPLICATION.get_app_config().user_id,
                global_flags: global_flags.to_vec()
            }
        ))
    }

    result
}

#[cfg(not(test))]
pub fn get_time() -> DWORD {
    unsafe { timeGetTime() }
}

#[cfg(test)]
pub fn get_time() -> DWORD {
    0
}


#[cfg(test)]
mod tests {
    use std::io::ErrorKind;
    use crate::{APPLICATION, Application, ReceivePayload};
    use crate::application::entrypoints::game_loop;
    use crate::application::{GAME_INIT_ADDRESS, GLOBAL_FLAGS_ADDRESS};
    use crate::network::ReceiveMessage;
    use crate::tests::{TestApplication, add_to_read_address_stack, calculate_address, add_to_read_payload_stack, READ_PAYLOAD_STACK};

    #[test]
    fn test_game_loop_with_error_from_receive_payload() {
        let game_init = 1;
        let global_flags: [u8;2055] = [0 as u8; 2055];
        add_to_read_address_stack(calculate_address(&global_flags, GLOBAL_FLAGS_ADDRESS));
        add_to_read_address_stack(calculate_address(&game_init, GAME_INIT_ADDRESS));
        add_to_read_payload_stack(
            Err(tungstenite::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, "Test network error")))
        );

        game_loop();

        let read_payload_stack = &*READ_PAYLOAD_STACK;
        assert_eq!(read_payload_stack.lock().unwrap().len(), 0);
    }
}
