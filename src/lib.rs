use std::fs;
use std::ptr::null_mut;

use toml;
use serde::Deserialize;

use winapi::shared::minwindef::*;
use winapi::um::winnt::DLL_PROCESS_ATTACH;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::processthreadsapi::ExitProcess;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};

use application::Application;
use utils::show_message_box;

pub mod utils;
pub mod network;
pub mod application;

const CONFIG_FILENAME: &str = "lamulana-config.toml";

#[derive(Deserialize)]
pub struct AppConfig {
    pub log_file_name: String,
    pub server_url: String
}

#[no_mangle]
extern "stdcall" fn DllMain(_h_inst_dll: HINSTANCE, fdw_reason: DWORD, _lpv_reserved: LPVOID) -> bool {
    if fdw_reason != DLL_PROCESS_ATTACH {
        return true;
    }

    unsafe {
        let app_config = read_config().map_err(|err| {
            show_message_box(&err);
            ExitProcess(1);
        }).unwrap();
        init_logger(&app_config);
        init_app(&app_config);
        return true;
    }
}

unsafe fn read_config() -> Result<AppConfig, String> {
    let file_contents = fs::read_to_string(CONFIG_FILENAME).map_err(|_| "Error reading config.".to_string())?;
    let app_config = toml::from_str::<AppConfig>(&file_contents).map_err(|_| "Error parsing config.".to_string())?;
    Ok(app_config)
}

unsafe fn init_logger(app_config: &AppConfig) {
    let file_appender = FileAppender::builder()
        .build(&app_config.log_file_name)
        .unwrap();
    let log_config = Config::builder()
        .appender(Appender::builder().build("lamulanamw", Box::new(file_appender)))
        .build(Root::builder().appender("lamulanamw").build(LevelFilter::Debug))
        .unwrap();
    log4rs::init_config(log_config).unwrap();
}

unsafe fn init_app(app_config: &AppConfig) {
    let websocket = network::init(&app_config.server_url);
    let address = GetModuleHandleW(null_mut()).cast::<u8>().wrapping_sub(0x400000);
    Application::attach(address, websocket);
}
