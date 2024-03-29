use std::fs;
use std::ptr::null_mut;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

use toml;
use serde::{Serialize, Deserialize};
use serde_with::{serde_as, DisplayFromStr};

use winapi::shared::minwindef::*;
use winapi::um::winnt::DLL_PROCESS_ATTACH;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::processthreadsapi::ExitProcess;

use log::{debug, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use tungstenite::Error;

use utils::show_message_box;
use crate::application::Application;
use crate::lm_structs::taskdata::TaskData;
use crate::network::{LiveRandomizer, Randomizer, ReceivePayload};

pub mod utils;
pub mod network;
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

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub log_file_name: String,
    pub server_url: String,
    pub user_id: i32,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub players: HashMap<i32, String>
}

pub struct LiveApplication {
    pub address: usize,
    pub randomizer: LiveRandomizer,
    pub app_config: AppConfig
}

impl Randomizer for LiveRandomizer {
    fn read_messages(&self) -> Result<ReceivePayload, Error> {
        self.read_messages()
    }

    fn send_message(&self, message: &str) {
        self.send_message(message)
    }
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
    let file_appender = FileAppender::builder()
        .build(&app_config.log_file_name)
        .unwrap();
    let log_config = Config::builder()
        .appender(Appender::builder().build("lamulanamw", Box::new(file_appender)))
        .build(Root::builder().appender("lamulanamw").build(LevelFilter::Debug))
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

    let randomizer = LiveRandomizer::new(&app_config.server_url, app_config.user_id);

    Box::new(LiveApplication { address, randomizer, app_config })
}

fn get_application() -> &'static Box<dyn Application + Sync> {
    &*APPLICATION
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use std::sync::Mutex;
    use crate::{AppConfig, TaskData};
    use crate::application::{Application, ApplicationMemoryOps};
    use crate::network::{Randomizer, ReceivePayload};
    use lazy_static::lazy_static;
    use tungstenite::Error;

    lazy_static!{
        pub static ref READ_ADDRESS_STACK: Mutex<Vec<u32>> = {
            Mutex::new(vec![])
        };

        pub static ref READ_PAYLOAD_STACK: Mutex<Vec<Result<ReceivePayload, Error>>> = {
            Mutex::new(vec![])
        };

        pub static ref SENT_MESSAGES: Mutex<Vec<String>> = {
            Mutex::new(vec![])
        };

        pub static ref TEST_RANDOMIZER: Box<dyn Randomizer + Sync> = {
            Box::new(
                TestRandomizer {}
            )
        };
    }

    pub struct TestApplication {}

    pub struct TestRandomizer {}

    pub fn add_to_read_address_stack(u: u32) {
        let stack_mutex = &*READ_ADDRESS_STACK;
        let mut stack = stack_mutex.lock().unwrap();
        stack.push(u);
    }

    pub fn add_to_read_payload_stack(input: Result<ReceivePayload, Error>) {
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
            todo!()
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
        fn read_messages(&self) -> Result<ReceivePayload, Error> {
            let stack_mutex = &*READ_PAYLOAD_STACK;
            let mut stack = stack_mutex.lock().unwrap();
            stack.pop().expect("No payload left in READ_ADDRESS_STACK")
        }

        fn send_message(&self, message: &str) {
            let messages_mutex = &*SENT_MESSAGES;
            let mut messages = messages_mutex.lock().unwrap();
            messages.push(message.to_string());

        }
    }

    pub fn init_test_app() -> Box<dyn Application + Sync> {
        Box::new(TestApplication { })
    }

    impl ApplicationMemoryOps for TestApplication {
        fn read_address<T>(&self, offset: usize) -> &mut T {
            unsafe {
                let addr: usize = std::mem::transmute(self.get_address().wrapping_add(offset));
                &mut*(addr as *mut T)
            }
        }
    }
}
