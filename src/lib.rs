#![feature(unboxed_closures)]
#![feature(tuple_trait)]

use std::ptr::null_mut;
use lazy_static::lazy_static;

use winapi::shared::minwindef::*;
use winapi::um::winnt::DLL_PROCESS_ATTACH;
use winapi::um::libloaderapi::GetModuleHandleW;

use log::{debug, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use pelite::FileMap;
use pelite::pe32::{Pe, PeFile};

use crate::application::Application;

pub mod utils;
pub mod archipelago;
pub mod screenplay;
pub mod application;
pub mod lm_structs;

lazy_static!{
    pub static ref APPLICATION: Box<dyn Application + Sync> = init_app();
}

pub struct LiveApplication {
    pub address: usize,
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

fn init_logger() {
    let file_appender = FileAppender::builder()
        .build("lamulana_easter.txt")
        .unwrap();
    let log_config = Config::builder()
        .appender(Appender::builder().build("lamulana_easter", Box::new(file_appender)))
        .build(Root::builder().appender("lamulana_easter").build(LevelFilter::Debug))
        .unwrap();
    log4rs::init_config(log_config).unwrap();
}

fn init_app() -> Box<dyn Application + Sync> {
    let address = unsafe { GetModuleHandleW(null_mut()).cast::<u8>().wrapping_sub(0x400000) } as usize;
    init_logger();

    let app_version = get_application_version();

    Box::new(LiveApplication { address, app_version})
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
        debug!("Could not open LaMulanaWin.exe to detect version");
        panic!()
    }
}
