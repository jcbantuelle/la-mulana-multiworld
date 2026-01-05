#![feature(unboxed_closures)]
#![feature(tuple_trait)]

use archipelago::api::APError;
use log::{debug, warn, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use pelite::FileMap;
use pelite::pe32::{Pe, PeFile};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use std::ptr::null_mut;
use std::sync::{LazyLock, Mutex};
use toml;
use utils::show_message_box;
use winapi::shared::minwindef::*;
use winapi::um::winnt::DLL_PROCESS_ATTACH;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::processthreadsapi::ExitProcess;

use crate::application::Application;

pub mod application;
pub mod archipelago;
pub mod lm_structs;
pub mod screenplay;
pub mod utils;

const CONFIG_FILENAME: &str = "lamulana-config.toml";

pub static APPLICATION: LazyLock<Application> = LazyLock::new(|| { init_app() });

#[derive(Serialize, Deserialize, Clone)]
pub struct ArchipelagoPlayer {
    pub id: i64,
    pub name: String
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct ArchipelagoItem {
    pub flag: u16,
    pub location_id: i64,
    pub player_id: i64,
    pub obtain_value: u8
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub server_url: String,
    pub password: String,
    pub log_file_name: String,
    pub local_player_id: i64,
    pub log_level: String,
    pub players: Vec<ArchipelagoPlayer>,
    pub item_mapping: Vec<ArchipelagoItem>,
}

impl AppConfig {
    fn players_lookup(&self) -> HashMap<i64, String> {
        self.players.clone().into_iter().map(|player| (player.id, player.name)).collect::<HashMap<_,_>>()
    }

    fn items(&self) -> HashMap<u16, ArchipelagoItem> {
        self.item_mapping.clone().into_iter().map(|mapping| (mapping.flag, mapping)).collect::<HashMap<_,_>>()
    }
}

#[no_mangle]
extern "system" fn DllMain(_h_inst_dll: HINSTANCE, fdw_reason: DWORD, _lpv_reserved: LPVOID) -> bool {
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

fn init_app() -> Application {
    let address = unsafe { GetModuleHandleW(null_mut()).cast::<u8>().wrapping_sub(0x400000) } as usize;

    let app_config = read_config().map_err(|err| {
        show_message_box(&err);
        unsafe{ ExitProcess(1) };
    }).unwrap();
    init_logger(&app_config);

    let randomizer = Mutex::new(Err(APError::NoConnection));
    let app_version = get_application_version();
    debug!("Starting lamulana multiworld injection for version {}.", app_version);

    Application { address, randomizer, app_config, app_version}
}

fn get_application() -> &'static Application {
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
