use std::ptr::{null_mut};
use winapi::um::winnt::DLL_PROCESS_ATTACH;
use winapi::shared::minwindef::*;
use winapi::um::libloaderapi::GetModuleHandleW;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};

pub mod application;
use application::Application;

static LOG_FILE_NAME: &str = "lamulanamw.log";

#[no_mangle]
extern "stdcall" fn DllMain(_h_inst_dll: HINSTANCE, fdw_reason: DWORD, _lpv_reserved: LPVOID) -> bool {
    if fdw_reason != DLL_PROCESS_ATTACH {
        return true;
    }

    unsafe {
        let file_appender = FileAppender::builder()
            .build(LOG_FILE_NAME)
            .unwrap();
        let config = Config::builder()
            .appender(Appender::builder().build("lamulanamw", Box::new(file_appender)))
            .build(Root::builder().appender("lamulanamw").build(LevelFilter::Debug))
            .unwrap();
        log4rs::init_config(config).unwrap();

        let address = GetModuleHandleW(null_mut()).cast::<u8>().wrapping_sub(0x400000);
        Application::attach(address);
        return true;
    }
}

