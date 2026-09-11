#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod ap_connection;
pub mod ap_data;
pub mod consts;
pub mod file_gen;
pub mod file_utils;
pub mod verifier;
pub mod windows;

use log::{debug, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use std::error::Error;
use std::sync::{Arc, LazyLock, Mutex};

use crate::ap_data::{APData, LaMulanaConfig};

use crate::windows::launcher::LauncherWindow;
use crate::windows::seed_selector::SeedSelectorWindow;

slint::include_modules!();

pub static AP_DATA: LazyLock<Arc<Mutex<Option<APData>>>> = LazyLock::new(|| { Arc::new(Mutex::new(None))});

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    configure_logger().await;

    match verifier::verify_install() {
        Ok(lm_config) => {
            load_ap_data(lm_config);

            let launcher_window = LauncherWindow::new(Arc::clone(&AP_DATA));
            let seed_selector_window = SeedSelectorWindow::new(Arc::clone(&AP_DATA));

            launcher_window.configure(&seed_selector_window).await;
            seed_selector_window.configure(&launcher_window).await;

            launcher_window.open();
        },
        Err(error_message) => {
            let error_message_window = ErrorMessage::new().inspect_err(|e| { debug!("Verification Issue window failed to initialize: {:?}", e); })?;
            error_message_window.set_error_message(error_message.into());
            let error_message_window_handle = error_message_window.as_weak();

            error_message_window.on_close_window(move || {
                let error_message_window = error_message_window_handle.unwrap();
                let _ = error_message_window.hide();
            });

            error_message_window.run().inspect_err(|e| { debug!("Verification Issue window failed to run: {:?}", e); })?;
        }
    }

    Ok(())
}

async fn configure_logger() {
    let file_appender = FileAppender::builder()
        .build("lmmw_launcher.txt")
        .unwrap();
    let log_config = Config::builder()
        .appender(Appender::builder().build("lmmw_launcher", Box::new(file_appender)))
        .logger(Logger::builder().build("goblin", LevelFilter::Off))
        .build(Root::builder().appender("lmmw_launcher").build(LevelFilter::Debug))
        .unwrap();
    log4rs::init_config(log_config).unwrap();
}

fn load_ap_data(lm_config: LaMulanaConfig) {
    let ap_data = APData::new(lm_config).inspect_err(|e| { debug!("Failed to initialize AP_DATA: {:?}", e); }).unwrap();
    match AP_DATA.lock() {
        Ok(mut ap_data_lock) => {
            *ap_data_lock = Some(ap_data.clone());
        },
        Err(e) => {
            debug!("Failed to acquire AP_DATA lock during initialization: {:?}", e);
        }
    }
}
