use log::{debug, warn};
use std::collections::{HashMap, VecDeque};
use std::sync::{LazyLock, Mutex, MutexGuard};

use crate::get_application;
use crate::archipelago::api::*;
use crate::archipelago::client::APClient;
use crate::lm_structs::items::ARCHIPELAGO_ITEM_LOOKUP;
use crate::lm_structs::script_header::{ScriptHeader, ScriptSubHeader};
use crate::lm_structs::taskdata::{EventWithBool, TaskData};
use crate::screenplay;

#[derive(Debug)]
pub struct GivenItem {
    pub player_id: i64,
    pub item_id: u32
}

#[derive(Clone)]
pub struct PlayerItem {
    pub player_id: i64,
    pub for_player: bool
}

#[derive(Clone)]
pub struct NetworkItemForPlayer {
    pub network_item: NetworkItem,
    pub rooms: Vec<String>
}

pub struct PlayerItemPopup {
    pub popup_id_address: usize,
    pub popup_id: u32,
    pub encoded: Vec<u16>
}

static PLAYER_ITEMS: LazyLock<Mutex<HashMap<i32, PlayerItem>>> = LazyLock::new(|| { Mutex::new(HashMap::new()) });
static PLAYER_ITEM_POPUP: Mutex<Option<PlayerItemPopup>> = Mutex::new(None);
static SYNC_REQUIRED: Mutex<bool> = Mutex::new(true);
static GAME_COMPLETE: Mutex<bool> = Mutex::new(false);
static ITEMS_TO_GIVE: Mutex<VecDeque<NetworkItemForPlayer>> = Mutex::new(VecDeque::new());
static DEFAULT_POPUP_SCRIPT: LazyLock<Vec<u16>> = LazyLock::new(|| { vec![0x100,0x000a] });
static RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| { tokio::runtime::Runtime::new().unwrap() });

pub type FnGameLoop = extern "C" fn();
pub type FnPopupDialogDrawIntercept = extern "C" fn(&TaskData);
pub type FnItemSymbolInitIntercept = extern "C" fn(&mut TaskData);

pub fn game_loop() {
    let application = get_application();
    let game_init: &mut u32 = application.read_address("game_init");
    let global_flags: &mut [u8;4096] = application.read_address("global_flags");
    let system_flags: &[u32;16] = application.read_address("system_flags");

    if (system_flags[3] & 0x20000) == 0x20000 {
        std::thread::spawn(move || {
            let _ = GAME_COMPLETE.try_lock().map(|mut game_complete| {
                if *game_complete == false {
                    *game_complete = true;
                    RUNTIME.block_on(send_game_complete_notice());
                }
            });
        });
    } else if (system_flags[0] & 0x1000000) == 0x1000000 {
        std::thread::spawn(move || {
            *SYNC_REQUIRED.lock().unwrap() = true;
        });
    } else if *game_init != 0 && global_flags[0x863] > 0 {
        display_item_if_available();
        std::thread::spawn(move || {
            RUNTIME.block_on(get_updates_from_server());
        });

        let item_lock = ITEMS_TO_GIVE.try_lock();
        if item_lock.is_ok() {
            let mut items_to_give = item_lock.unwrap();
            let item_to_give = items_to_give.pop_front();
            match item_to_give {
                Some(ap_item) => {
                    let ap_item_id = ap_item.network_item.item;
                    let lm_item = ARCHIPELAGO_ITEM_LOOKUP.get(&(ap_item_id)).unwrap();

                    let inventory_pointer: &mut usize = application.read_address("inventory_words");
                    let inventory: &[u16;114] = application.read_raw_address(*inventory_pointer);

                    let give_item = if lm_item.item_id == 70 || lm_item.item_id == 19 || lm_item.item_id == 69 {
                        global_flags[lm_item.flag] == 0
                    } else {
                        lm_item.item_id > 104 || inventory[lm_item.item_id] == 0
                    };

                    if give_item {
                        let mut rooms = ap_item.rooms.clone();

                        let field: &mut u8 = application.read_address("current_field");
                        let screen: &mut u8 = application.read_address("current_screen");
                        let scene: &mut u8 = application.read_address("current_scene");
                        let room_index = format!("{},{},{}", field, scene, screen);

                        if rooms.contains(&room_index) {
                            items_to_give.push_back(ap_item);
                        } else {
                            let player_id = ap_item.network_item.player;
                            rooms.push(room_index);
                            items_to_give.push_back(NetworkItemForPlayer { network_item: ap_item.network_item, rooms });
                            if let Ok(ref mut player_items) = PLAYER_ITEMS.lock() {
                                player_items.insert(lm_item.item_id as i32, PlayerItem {
                                    player_id,
                                    for_player: false
                                });
                            }

                            application.give_item(lm_item.item_id as u32);
                            global_flags[lm_item.flag] = 2
                        }
                    }
                },
                None => ()
            }
        }
    } else {
        std::thread::spawn(move || {
            *SYNC_REQUIRED.lock().unwrap() = true;
        });
    }
    application.original_game_loop()
}

pub fn popup_dialog_draw_intercept(popup_dialog: &'static TaskData) {
    let application = get_application();
    let mut player_items = PLAYER_ITEMS.lock().unwrap();

    if let Some(player_item) = player_items.get(&popup_dialog.sbuff[0]) {
        let script_header: &*const ScriptHeader = application.read_address("script_header_pointer");
        let line_header = unsafe { (*script_header.add(3)).data as *mut ScriptSubHeader};
        let line = unsafe { &mut *line_header.add(2) };

        let popup_text = if player_item.for_player {
            format!("  For Another Player!")
        } else {
            let player_id = &player_item.player_id;
            let server_name = "Server".to_string();
            let players = application.get_app_config().players_lookup();
            let player_name = players.get(player_id).unwrap_or(&server_name);
            format!("  From {player_name}!")
        };
        let space_count = popup_text.chars().filter(|c| *c == ' ').count();
        let mut encoded_popup_text = screenplay::encode(popup_text);
        encoded_popup_text.push(0x000a);
        player_items.remove(&popup_dialog.sbuff[0]);

        let popup = PlayerItemPopup {
            popup_id_address: &popup_dialog.id.uid as *const u32 as usize,
            popup_id: popup_dialog.id.uid,
            encoded: encoded_popup_text
        };

        let mut popup_option = PLAYER_ITEM_POPUP.lock().unwrap();
        *popup_option = Some(popup);
        let popup = popup_option.as_ref().unwrap();

        *line = ScriptSubHeader {
            pointer: popup.encoded.as_ptr() as usize,
            data_num: popup.encoded.len() as i32,
            font_num: (popup.encoded.len() - space_count - 1) as i32
        };
        
    }
    application.popup_dialog_draw(popup_dialog)
}

pub fn item_symbol_init_intercept(item: &'static mut TaskData) {
    let application = get_application();
    item.rfunc = item_symbol_back_intercept as EventWithBool;
    application.original_item_symbol_init(item);
}

pub fn item_symbol_back_intercept(item: &mut TaskData) -> u32 {
    let application = get_application();
    let acquired = item.hit_data > 0;
    let item_id = item.buff[1];
    let for_other_player = item_id == 83;

    if for_other_player {
        item.sbuff[2] = 0;
    }

    let item_symbol_back: &*const () = application.read_address("item_symbol_back");
    let item_symbol_back_func: extern "C" fn(&TaskData) -> u32 = unsafe { std::mem::transmute(item_symbol_back) };
    let result = (item_symbol_back_func)(item);

    if acquired && for_other_player {
        if let Ok(ref mut player_items) = PLAYER_ITEMS.lock() {
            player_items.insert(item_id, PlayerItem {
                for_player: true,
                player_id: 0
            });
        }

        application.create_dialog_popup(item_id as u32);
    }

    result
}

fn display_item_if_available() {
    let application = get_application();
    if let Some(popup_option) = PLAYER_ITEM_POPUP.try_lock().ok().as_mut() {
        if let Some(popup) = popup_option.as_ref() {
            if popup.popup_id != *application.read_raw_address::<u32>(popup.popup_id_address) {
                let script_header: &*const ScriptHeader = application.read_address("script_header_pointer");
                let line_header = unsafe { (*script_header.add(3)).data as *mut ScriptSubHeader};
                let line = unsafe { &mut *line_header.add(2) };
                line.pointer = DEFAULT_POPUP_SCRIPT.as_ptr() as usize;
                line.data_num = 2;
                line.font_num = 1;
            }
        }
    }
}

async fn get_updates_from_server() {
    let application = get_application();

    // Get Handle to Randomizer, and Connect to Server If Not Currently Connected
    let Ok(mut randomizer_lock) = application.get_randomizer().try_lock() else { return };
    let Ok(randomizer) = randomizer_lock.as_mut() else {
        connect_to_server(randomizer_lock).await;
        return
    };

    // Send Sync Request to Server if Needed, Attempt Reconnect if Network Error
    if SYNC_REQUIRED.try_lock().is_ok_and(|mut sync_lock| {
        let sync_required = *sync_lock;
        *sync_lock = false;
        sync_required
    }) {
        match randomizer.sync().await {
            Ok(_) => {},
            Err(ap_error) => {
                match ap_error {
                    APError::NoConnection => {},
                    e => {
                        warn!("Unhandled Network Error {}, attempting reconnect", e);
                    }
                }
                connect_to_server(randomizer_lock).await;
                return
            }
        }
    }

    let global_flags: &mut [u8;4096] = application.read_address("global_flags");
    let found_items: Vec<i64> = application.get_app_config().items().iter().filter(|(k,_)|
        global_flags[**k as usize] == 2
    ).map(|(_,v)|
        v.location_id
    ).collect();

    // Send List of Found Items to Server, Attempt Reconnect if Network Error
    match randomizer.location_checks(found_items).await {
        Ok(_) => {},
        Err(_) => {
            warn!("Attempt to Send to AP Server Failed, Attempting Reconnect");
            connect_to_server(randomizer_lock).await;
            return
        }
    }

    // Read Next Message From Server
    match randomizer.read().await {
        Ok(response) => {
            debug!("Received Message From Server: {:?}", response);
            match response {
                ServerPayload::ReceivedItems(received_items) => {
                    if received_items.index > 0 {
                        let mut received_item_index = ((global_flags[0x867] as u16) << 8) | global_flags[0x868] as u16;
                        if received_item_index != received_items.index {
                            *SYNC_REQUIRED.lock().unwrap() = true;
                        }
                        received_item_index += 1;
                        global_flags[0x867] = (received_item_index >> 8) as u8;
                        global_flags[0x868] = received_item_index as u8;
                    }

                    let items_from_ap = received_items.items;
                    let mut items_to_give = ITEMS_TO_GIVE.lock().unwrap();
                    for network_item in items_from_ap {
                        let item_for_player = NetworkItemForPlayer {
                            network_item,
                            rooms: Vec::new()
                        };
                        items_to_give.push_back(item_for_player);
                    }
                },
                _ => {}
            }
        },
        Err(e) => {
            match e {
                APError::PingPong => {}, // Suppress Ping/Pong Responses
                APError::NoConnection => {
                    debug!("Connection to Server Lost, Attempting Reconnect");
                    connect_to_server(randomizer_lock).await;
                    return
                },
                _ => {
                    debug!("Unexpected Binary Data from Server");
                }
            }
        }
    }
}

pub async fn connect_to_server(mut randomizer: MutexGuard<'_, Result<APClient, APError>>) {
    let application = get_application();
    let app_config = application.get_app_config();
    *randomizer = APClient::new(&app_config.server_url).await;
    match randomizer.as_mut() {
        Ok(ap_client) => {
            let player_id = app_config.local_player_id;
            let players = app_config.players_lookup();
            let player_name = players.get(&player_id).unwrap();
            let password = &app_config.password;
            match ap_client.connect(password, "La-Mulana", &player_name, player_id, ItemHandling::OtherWorldsOnly, vec![], false).await {
                Ok(_) => {},
                Err(e) => {
                    debug!("Connect Failure with error {:?}", e);
                }
            }
        },
        Err(e) => {
            debug!("AP Client Not Connected with Error {}", e);
        }
    };
}

pub async fn send_game_complete_notice() {
    let application = get_application();
    let mut randomizer = application.get_randomizer().lock().unwrap();
    match randomizer.as_mut() {
        Ok(ap_client) => {
            match ap_client.status_update(ClientStatus::ClientGoal).await {
                Ok(_) => {},
                Err(e) => {
                    debug!("Game Completion Notice Failure with error {:?}", e);
                }
            }
        },
        Err(e) => {
            debug!("AP Client Not Connected with Error {}", e);
        }
    };
}
