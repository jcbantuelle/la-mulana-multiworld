use std::fs;
use std::ptr::null_mut;
use lazy_static::lazy_static;
use std::collections::HashMap;

use toml;
use serde::{Serialize, Deserialize};
use serde_with::{serde_as, DisplayFromStr};

use winapi::shared::minwindef::*;
use winapi::um::winnt::DLL_PROCESS_ATTACH;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::processthreadsapi::ExitProcess;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use tungstenite::Error;

use utils::show_message_box;
use crate::lm_structs::taskdata::TaskData;
use crate::network::{LiveRandomizer, ReceivePayload};

pub mod utils;
pub mod network;
pub mod screenplay;
pub mod application;
pub mod lm_structs;

const CONFIG_FILENAME: &str = "lamulana-config.toml";

pub trait Randomizer {
    fn read_messages(&self) -> Result<ReceivePayload, tungstenite::Error>;
    fn send_message(&self, message: &str);
}

pub trait Application {
    fn attach(&self);
    fn get_address(&self) -> usize;
    fn get_randomizer(&self) -> &dyn Randomizer;
    fn get_app_config(&self) -> &AppConfig;
    fn give_item(&self, item: u32);
    fn create_dialog_popup(&self, item_id: u32);
    fn popup_dialog_draw(&self, popup_dialog: &TaskData);
}

pub trait ApplicationMemoryOps {
    fn read_address<V>(&self, offset: usize) -> &mut V;
}

lazy_static!{
    pub static ref APPLICATION: Box<dyn Application + Sync> = init_app();
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
