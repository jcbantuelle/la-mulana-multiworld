use std::fs;
use std::ptr::null_mut;
use winapi::um::winnt::DLL_PROCESS_ATTACH;
use winapi::shared::minwindef::*;
use serde::Deserialize;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, load_config_file, Root};
use toml;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::processthreadsapi::ExitProcess;

pub mod application;
pub mod network;
pub mod structs;

use application::{ Application, show_message_box };
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
        let _ = read_config_and_init_app().map_err(|err| {
            show_message_box(&err);
            ExitProcess(1);
        });
        return true;
    }
}

unsafe fn read_config_and_init_app() -> Result<(), String> {
    let file_contents = fs::read_to_string(CONFIG_FILENAME).map_err(|_| "Error reading config.".to_string())?;
    let app_config = toml::from_str::<AppConfig>(&file_contents).map_err(|_| "Error parsing config.".to_string())?;
    let file_appender = FileAppender::builder()
        .build(app_config.log_file_name)
        .unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("lamulanamw", Box::new(file_appender)))
        .build(Root::builder().appender("lamulanamw").build(LevelFilter::Debug))
        .unwrap();
    log4rs::init_config(config).unwrap();
    let websocket = network::init(&app_config.server_url);
    let address = GetModuleHandleW(null_mut()).cast::<u8>().wrapping_sub(0x400000);
    Ok(Application::attach(address, websocket))
}
