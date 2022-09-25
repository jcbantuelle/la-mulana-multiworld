use std::fs;
use std::ptr::null_mut;
use winapi::um::winnt::DLL_PROCESS_ATTACH;
use winapi::shared::minwindef::*;
use serde::Deserialize;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use toml;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::processthreadsapi::ExitProcess;

pub mod application;
pub mod network;
pub mod structs;

use application::{ Application, show_message_box };
const CONFIG_FILENAME: &str = "lamulana-config.toml";

#[derive(Deserialize)]
struct AppConfig {
    log_file_name: String,
    server_url: String
}

#[no_mangle]
extern "stdcall" fn DllMain(_h_inst_dll: HINSTANCE, fdw_reason: DWORD, _lpv_reserved: LPVOID) -> bool {
    if fdw_reason != DLL_PROCESS_ATTACH {
        return true;
    }

    unsafe {
        read_config_and_init_app();
        return true;
    }
}

unsafe fn read_config_and_init_app() {
    match fs::read_to_string(CONFIG_FILENAME) {
        Ok(file_contents) => {
            match toml::from_str::<AppConfig>(&file_contents) {
                Ok(app_config) => {
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
                    Application::attach(address, websocket);
                },
                Err(_) => {
                    show_message_box("Unable to parse config.");
                    ExitProcess(1);
                }
            };
        },
        Err(_) => {
            show_message_box(&format!("{} does not exist.", CONFIG_FILENAME));
            ExitProcess(1);
        }
    };
}
