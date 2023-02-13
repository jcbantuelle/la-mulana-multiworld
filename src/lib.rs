use std::fs;
use std::ptr::null_mut;
use lazy_static::lazy_static;

use toml;
use serde::Deserialize;

use winapi::shared::minwindef::*;
use winapi::um::winnt::DLL_PROCESS_ATTACH;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::processthreadsapi::ExitProcess;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};

use utils::show_message_box;
use crate::network::Randomizer;

pub mod utils;
pub mod network;
pub mod screenplay;
pub mod application;
pub mod lm_structs;

const CONFIG_FILENAME: &str = "lamulana-config.toml";

lazy_static!{
    pub static ref APPLICATION: Application = init_app();
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub log_file_name: String,
    pub server_url: String,
    pub user_id: u64,
    pub buddy_id: u64
}

pub struct Application {
    pub address: usize,
    pub randomizer: Randomizer,
    pub app_config: AppConfig
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
    let file_contents = fs::read_to_string(CONFIG_FILENAME).map_err(|_| "Error reading config.".to_string())?;
    let app_config = toml::from_str::<AppConfig>(&file_contents).map_err(|_| "Error parsing config.".to_string())?;
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

fn init_app() -> Application {
    let address = unsafe { GetModuleHandleW(null_mut()).cast::<u8>().wrapping_sub(0x400000) } as usize;

    let app_config = read_config().map_err(|err| {
        show_message_box(&err);
        unsafe{ ExitProcess(1) };
    }).unwrap();
    init_logger(&app_config);

    let randomizer = Randomizer::new(&app_config.server_url, app_config.user_id);

    Application { address, randomizer, app_config }
}
