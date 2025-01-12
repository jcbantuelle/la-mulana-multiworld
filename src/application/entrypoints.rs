use std::collections::VecDeque;
use std::sync::Mutex;
use std::thread;
use lazy_static::lazy_static;
use log::warn;
use winapi::shared::minwindef::*;
use winapi::um::timeapi::timeGetTime;

use crate::archipelago::client::{ArchipelagoClient, ArchipelagoError};
use crate::archipelago::protocol::ServerMessage;
use crate::{APPLICATION, get_application};
use crate::application::{Application, ApplicationMemoryOps};
use crate::lm_structs::items::ARCHIPELAGO_ITEM_LOOKUP;
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
    static ref PLAYER_ITEM: Mutex<Option<PlayerItem>> = Mutex::new(None);
    static ref PLAYER_ITEM_POPUP: Mutex<Option<PlayerItemPopup>> = Mutex::new(None);
    static ref MESSAGE_QUEUE: Mutex<VecDeque<ServerMessage>> = Mutex::new(VecDeque::new());
}

pub type FnGameLoop = extern "C" fn();
pub type FnPopupDialogDrawIntercept = extern "C" fn(&TaskData);
pub type FnItemSymbolInitIntercept = extern "C" fn(&mut TaskData);

pub fn game_loop() {
    let application = get_application();
    let app_addresses = application.app_addresses();

    if let Some(popup_option) = PLAYER_ITEM_POPUP.try_lock().ok().as_mut() {
        if let Some(popup) = popup_option.as_ref() {
            if popup.popup_id != *application.read_address::<u32>(popup.popup_id_address) {
                *application.read_address::<ScriptSubHeader>(popup.line_address) = popup.old_line;
                **popup_option = None;
            }
        }
    }

    thread::spawn(|| {
        match application.get_randomizer().try_lock() {
            Ok(mut randomizer) => {
                match randomizer.as_mut() {
                    Ok(client) => {
                        let global_flags: &[u8;4096] = application.read_address(app_addresses.global_flags_address);
                        let found_items: Vec<u64> = application.get_app_config().items().iter().filter(|(k,v)|
                            global_flags[**k as usize] == v.obtain_value
                        ).map(|(_,v)|
                            v.location_id
                        ).collect();

                        match client.location_checks(found_items) {
                            Ok(_) => {
                                match client.read() {
                                    Ok(message_wrapper) => {
                                        match message_wrapper {
                                            Some(message) => {
                                                let mut message_queue = MESSAGE_QUEUE.lock().unwrap();
                                                message_queue.push_back(message);
                                            },
                                            None => ()
                                        }
                                    },
                                    Err(_) => ()
                                }
                            },
                            Err(_) => {
                                *randomizer = reconnect_to_server(application);
                            }
                        }
                    },
                    Err(error) => {
                        match error {
                            ArchipelagoError::ConnectionClosed => {
                                *randomizer = reconnect_to_server(application);
                            },
                            _ => ()
                        }
                    }
                }
            }
            Err(_) => () // Okay to pass when cannot lock
        };
    });
        
    let game_init: &mut u32 = application.read_address(app_addresses.game_init_address);
    if *game_init != 0 {
        let mut message: Option<ServerMessage> = None;
        if let Ok(ref mut message_queue) = MESSAGE_QUEUE.try_lock() {
            message = message_queue.pop_front();
        }
        if message.is_some() {
            match message.unwrap() {
                ServerMessage::ReceivedItems(received_items) => {
                    let network_items = received_items.items;

                    for ap_item in network_items {
                        let item = ARCHIPELAGO_ITEM_LOOKUP.get(&(ap_item.item as u64)).unwrap();
                        let inventory_pointer: &mut usize = application.read_address(app_addresses.inventory_words);
                        let inventory: &[u16;114] = application.read_address(*inventory_pointer);
                        let global_flags: &mut [u8;4096] = application.read_address(app_addresses.global_flags_address);

                        let give_item = if item.item_id == 70 || item.item_id == 19 || item.item_id == 69 {
                            global_flags[item.flag] == 0
                        } else {
                            item.item_id > 104 || inventory[item.item_id] == 0
                        };

                        if give_item {
                            let player_id = ap_item.player;
                            if let Some(player_item) = PLAYER_ITEM.lock().ok().as_mut() {
                                **player_item = Some(PlayerItem {
                                    player_id,
                                    for_player: false
                                });
                            }
                            application.give_item(item.item_id as u32);
                            global_flags[item.flag] = item.value
                        }
                    }
                }
                _ => ()
            }
        }
    }
}

pub fn popup_dialog_draw_intercept(popup_dialog: &'static TaskData) {
    let application = get_application();
    let app_addresses = application.app_addresses();
    let mut player_item_option = PLAYER_ITEM.lock().unwrap();

    if let Some(player_item) = player_item_option.as_ref() {
        let script_header: &*const ScriptHeader = application.read_address(app_addresses.script_header_pointer_address);
        let line_header = unsafe { (*script_header.add(3)).data as *mut ScriptSubHeader};
        let line = unsafe { &mut *line_header.add(2) };

        let popup_text = if player_item.for_player {
            format!("For Another Player")
        } else {
            let player_id = &player_item.player_id;
            let server_name = "Server".to_string();
            let players = application.get_app_config().players_lookup();
            let player_name = players.get(player_id).unwrap_or(&server_name);
            format!("From {}", player_name)
        };

        let popup = PlayerItemPopup {
            popup_id_address: &popup_dialog.id.uid as *const u32 as usize,
            popup_id: popup_dialog.id.uid,
            encoded: screenplay::encode(format!("  {}", popup_text)),
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

pub fn item_symbol_init_intercept(item: &'static mut TaskData) {
    let application = get_application();
    item.rfunc = item_symbol_back_intercept as EventWithBool;
    application.original_item_symbol_init(item);
}

pub fn item_symbol_back_intercept(item: &mut TaskData) -> u32 {
    let application = get_application();
    let app_addresses = application.app_addresses();
    let acquired = item.hit_data > 0;
    let item_id = item.buff[1];
    let for_other_player = item_id == 83;

    if for_other_player {
        item.sbuff[2] = 0;
    }

    let item_symbol_back: &*const () = APPLICATION.read_address(app_addresses.item_symbol_back_address);
    let item_symbol_back_func: extern "C" fn(&TaskData) -> u32 = unsafe { std::mem::transmute(item_symbol_back) };
    let result = (item_symbol_back_func)(item);

    if acquired && for_other_player {
        let player_item = PlayerItem {
            for_player: true,
            player_id: 0
        };

        {
            let mut player_item_option = PLAYER_ITEM.lock().unwrap();
            *player_item_option = Some(player_item);
        }

        APPLICATION.create_dialog_popup(item_id as u32);
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

pub fn reconnect_to_server(application: &Box<dyn Application + Sync>) -> Result<ArchipelagoClient, ArchipelagoError> {
    let app_config = application.get_app_config();
    ArchipelagoClient::new(&app_config.server_url).map(|mut randomizer| {
        let player_id = app_config.local_player_id;
        let players = app_config.players_lookup();
        let player_name = players.get(&player_id).unwrap();
        let password = if app_config.password.is_empty() { None } else { Some(app_config.password.as_str()) };
        let _ = randomizer.connect("La-Mulana", &player_name, &player_id.to_string(), password, Some(1), vec![], false);
        let _ = randomizer.sync();
        randomizer
    })
}

#[cfg(test)]
mod tests {
    use archipelago_rs::client::ArchipelagoError;
    use archipelago_rs::protocol::{NetworkItem, ServerMessage};
    use archipelago_rs::protocol::ReceivedItems;
    use crate::{APPLICATION, Application, ReceivePayload, screenplay};
    use crate::application::entrypoints::{game_loop, PLAYER_ITEM_POPUP, PlayerItemPopup};
    use crate::application::{GAME_INIT_ADDRESS, GLOBAL_FLAGS_ADDRESS};
    use crate::lm_structs::script_header::ScriptSubHeader;
    use crate::tests::{add_to_read_address_stack, calculate_address, add_to_read_payload_stack, READ_PAYLOAD_STACK, ITEMS_RECEIVED};

    #[test]
    fn test_game_loop_with_error_from_receive_payload() {
        let game_init = 1;
        let global_flags: [u8;2055] = [0 as u8; 2055];
        add_to_read_address_stack(calculate_address(&global_flags, GLOBAL_FLAGS_ADDRESS));
        add_to_read_address_stack(calculate_address(&game_init, GAME_INIT_ADDRESS));
        add_to_read_payload_stack(
            Err(
                ArchipelagoError::ConnectionClosed
            )
        );

        game_loop();

        let read_payload_stack = &*READ_PAYLOAD_STACK;
        assert_eq!(read_payload_stack.lock().unwrap().len(), 0);
    }

    #[test]
    fn test_game_loop_getting_item_from_server() {
        let game_init = 1;
        let global_flags: [u8;2055] = [0u8; 2055];
        let subscript_header = ScriptSubHeader {
            pointer: 0,
            data_num: 0,
            font_num: 0,
        };
        let popup_id = 1; // It doesn't match the id of the popup above
        let popup_id_address = calculate_address(&popup_id, 0) as usize;
        let popup = PlayerItemPopup {
            popup_id_address,
            popup_id: 0,
            encoded: screenplay::encode(format!("  {} {}", "Test", "Test")),
            line_address: 0,
            old_line: subscript_header
        };
        let mut popup_option = PLAYER_ITEM_POPUP.lock().unwrap();
        *popup_option = Some(popup);

        add_to_read_address_stack(calculate_address(&global_flags, GLOBAL_FLAGS_ADDRESS));
        add_to_read_address_stack(calculate_address(&game_init, GAME_INIT_ADDRESS));

        add_to_read_payload_stack(
            Ok(
                Some(
                    ServerMessage::ReceivedItems(
                        ReceivedItems {
                            index: 1,
                            items: vec![
                                NetworkItem {
                                    item: 1,
                                    location: 0,
                                    player: 0,
                                    flags: 0,
                                }
                            ],
                        }
                    )
                )
            )
        );

        game_loop();

        let items_received_mutex = &*ITEMS_RECEIVED;
        assert_eq!(items_received_mutex.lock().unwrap().len(), 1);
    }
}
