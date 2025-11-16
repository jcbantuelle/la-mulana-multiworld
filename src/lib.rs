#![feature(unboxed_closures)]
#![feature(tuple_trait)]

use std::fs;
use std::ptr::null_mut;
use std::sync::Mutex;
use archipelago::client::{ArchipelagoClient, ArchipelagoError};
use lazy_static::lazy_static;
use std::collections::HashMap;
use toml;
use serde::{Serialize, Deserialize};

use winapi::shared::minwindef::*;
use winapi::um::winnt::DLL_PROCESS_ATTACH;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::processthreadsapi::ExitProcess;

use log::{debug, warn, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use pelite::FileMap;
use pelite::pe32::{Pe, PeFile};

use utils::show_message_box;
use crate::application::Application;

pub mod utils;
pub mod archipelago;
pub mod screenplay;
pub mod application;
pub mod lm_structs;

const CONFIG_FILENAME: &str = "lamulana-config.toml";

#[cfg(not(test))]
lazy_static!{
    pub static ref APPLICATION: Box<dyn Application + Sync> = init_app();
}

#[cfg(test)]
lazy_static!{
    pub static ref APPLICATION: Box<dyn Application + Sync> = tests::init_test_app();
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ArchipelagoPlayer {
    pub id: i32,
    pub name: String
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct ArchipelagoItem {
    pub flag: u16,
    pub location_id: i64,
    pub player_id: i32,
    pub obtain_value: u8
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub server_url: String,
    pub password: String,
    pub log_file_name: String,
    pub local_player_id: i32,
    pub log_level: String,
    pub players: Vec<ArchipelagoPlayer>,
    pub item_mapping: Vec<ArchipelagoItem>,
}

impl AppConfig {
    fn players_lookup(&self) -> HashMap<i32, String> {
        self.players.clone().into_iter().map(|player| (player.id, player.name)).collect::<HashMap<_,_>>()
    }

    fn items(&self) -> HashMap<u16, ArchipelagoItem> {
        self.item_mapping.clone().into_iter().map(|mapping| (mapping.flag, mapping)).collect::<HashMap<_,_>>()
    }
}

pub struct LiveApplication {
    pub address: usize,
    pub randomizer: Mutex<Result<ArchipelagoClient, ArchipelagoError>>,
    pub app_config: AppConfig,
    pub app_version: String
}

#[no_mangle]
extern "stdcall" fn DllMain(_h_inst_dll: HINSTANCE, fdw_reason: DWORD, _lpv_reserved: LPVOID) -> bool {
    if fdw_reason != DLL_PROCESS_ATTACH {
        return true;
    }
    APPLICATION.attach();
    true
}

fn read_config() -> Result<AppConfig, String> {
    let file_contents = fs::read_to_string(CONFIG_FILENAME).map_err(|e| format!("Error reading config: {}", e.to_string()))?;
    let app_config = toml::from_str::<AppConfig>(&file_contents).map_err(|e| format!("Error parsing config: {}", e.to_string()))?;
    Ok(app_config)
}

fn init_logger(app_config: &AppConfig) {
    let level_filter = match app_config.log_level.as_str() {
        "OFF" => LevelFilter::Off,
        "ERROR" => LevelFilter::Error,
        "WARN" => LevelFilter::Warn,
        "INFO" => LevelFilter::Info,
        "DEBUG" => LevelFilter::Debug,
        "TRACE" => LevelFilter::Trace,
        _ => LevelFilter::Debug,
    };
    let file_appender = FileAppender::builder()
        .build(&app_config.log_file_name)
        .unwrap();
    let log_config = Config::builder()
        .appender(Appender::builder().build("lamulanamw", Box::new(file_appender)))
        .build(Root::builder().appender("lamulanamw").build(level_filter))
        .unwrap();
    log4rs::init_config(log_config).unwrap();
}

fn init_app() -> Box<dyn Application + Sync> {
    let address = unsafe { GetModuleHandleW(null_mut()).cast::<u8>().wrapping_sub(0x400000) } as usize;

    let app_config = read_config().map_err(|err| {
        show_message_box(&err);
        unsafe{ ExitProcess(1) };
    }).unwrap();
    init_logger(&app_config);

    let randomizer = Mutex::new(Err(ArchipelagoError::ConnectionClosed));
    let app_version = get_application_version();
    debug!("Starting lamulana multiworld injection for version {}.", app_version);

    Box::new(LiveApplication { address, randomizer, app_config, app_version})
}

fn get_application() -> &'static Box<dyn Application + Sync> {
    &*APPLICATION
}

fn get_application_version() -> String {
    let file_path = "LaMulanaWin.exe";

    if let Ok(map) = FileMap::open(&file_path) {
        let file = PeFile::from_bytes(&map).unwrap();

        let resources = file.resources().unwrap();
        let version_info = resources.version_info().unwrap();

        let fixed_file_info = version_info.fixed().unwrap();
        fixed_file_info.dwFileVersion.to_string()
    }
    else {
        warn!("Could not open LaMulanaWin.exe to detect version");
        panic!()
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use std::sync::Mutex;
    use archipelago_rs::client::ArchipelagoError;
    use archipelago_rs::protocol::{ClientMessage, ServerMessage};
    use crate::{AppConfig, ReceiveMessageError, TaskData};
    use crate::application::{Application, ApplicationMemoryOps};
    use crate::network::{Randomizer, ReceivePayload};
    use lazy_static::lazy_static;

    lazy_static!{
        pub static ref READ_ADDRESS_STACK: Mutex<Vec<u32>> = {
            Mutex::new(vec![])
        };

        pub static ref READ_PAYLOAD_STACK: Mutex<Vec<Result<Option<ServerMessage>, ArchipelagoError>>> = {
            Mutex::new(vec![])
        };

        pub static ref SENT_MESSAGES: Mutex<Vec<ClientMessage>> = {
            Mutex::new(vec![])
        };

        pub static ref TEST_RANDOMIZER: Box<dyn Randomizer + Sync> = {
            Box::new(
                TestRandomizer {}
            )
        };

        pub static ref ITEMS_RECEIVED: Mutex<Vec<u32>> = {
            Mutex::new(vec![])
        };
    }

    pub struct TestApplication {}

    pub struct TestRandomizer {}

    pub fn add_to_read_address_stack(u: u32) {
        let stack_mutex = &*READ_ADDRESS_STACK;
        let mut stack = stack_mutex.lock().unwrap();
        stack.push(u);
    }

    pub fn add_to_read_payload_stack(input: Result<Option<ServerMessage>, ArchipelagoError>) {
        let stack_mutex = &*READ_PAYLOAD_STACK;
        let mut stack = stack_mutex.lock().unwrap();
        stack.push(input);
    }

    pub fn calculate_address<T>(input: &T, address: usize) -> u32 {
        (input as *const T).cast::<u8>().wrapping_sub(address) as u32
    }

    impl Application for TestApplication {
        fn attach(&self) {
            todo!()
        }

        fn get_address(&self) -> usize {
            let stack_mutex = &*READ_ADDRESS_STACK;
            let mut stack = stack_mutex.lock().unwrap();
            stack.pop().expect("No address left in READ_ADDRESS_STACK") as usize
        }

        fn get_randomizer(&self) -> &dyn Randomizer {
            &**TEST_RANDOMIZER
        }

        fn get_app_config(&self) -> &AppConfig {
            todo!()
        }

        fn give_item(&self, item: u32) {
            let items_mutex = &*ITEMS_RECEIVED;
            let mut items = items_mutex.lock().unwrap();
            items.push(item);
        }

        fn create_dialog_popup(&self, item_id: u32) {
            todo!()
        }

        fn popup_dialog_draw(&self, popup_dialog: &TaskData) {
            todo!()
        }

        fn pause_game_process(&self) {
            todo!()
        }

        fn disable_movement(&self) {
            todo!()
        }

        fn disable_warp_menu(&self) {
            todo!()
        }

        fn set_lemeza_item_pose(&self) {
            todo!()
        }

        fn play_sound_effect(&self, effect_id: u32) {
            todo!()
        }

        fn option_stuck(&self, option_num: u32) {
            todo!()
        }

        fn option_pos(&self, x: f32, y: f32) {
            todo!()
        }
    }

    impl Randomizer for TestRandomizer {
        fn read_messages(&self) -> Result<std::option::Option<ServerMessage>, ArchipelagoError> {
            let stack_mutex = &*READ_PAYLOAD_STACK;
            let mut stack = stack_mutex.lock().unwrap();
            stack.pop().expect("No payload left in READ_ADDRESS_STACK")
        }

        fn send_message(&self, message: ClientMessage) {
            let messages_mutex = &*SENT_MESSAGES;
            let mut messages = messages_mutex.lock().unwrap();
            messages.push(message);

        }
    }

    pub fn init_test_app() -> Box<dyn Application + Sync> {
        Box::new(TestApplication { })
    }

    impl ApplicationMemoryOps for TestApplication {
        fn read_address<T>(&self, offset: usize) -> &mut T {
            unsafe {
                let stack_mutex = &*READ_ADDRESS_STACK;
                let mut stack = stack_mutex.lock().unwrap();
                let addr = stack.pop().expect("No address left in READ_ADDRESS_STACK") as usize;

                &mut*(addr as *mut T)
            }
        }
    }
}
