#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod ap_connection;
pub mod ap_data;
pub mod archipelago;
pub mod consts;
pub mod file_utils;
pub mod verifier;

use dll_syringe::{process::OwnedProcess, Syringe};
use slint::ComponentHandle;
use std::error::Error;
use std::sync::Mutex;
use std::process;

use crate::ap_connection::APConnection;
use crate::ap_data::APData;
use crate::consts::*;

slint::include_modules!();

pub static AP_DATA: Mutex<Option<APData>> = Mutex::new(None);
pub static AP_CONNECTION: Mutex<Option<APConnection>> = Mutex::new(None);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    match verifier::verify_install() {
        Ok(lm_config) => {
            let ap_data = APData::new(lm_config)?;

            let launcher = Launcher::new().unwrap();
            let launcher_handle = launcher.as_weak();

            launcher.on_launch_game(move || {
                let _ = slint::spawn_local(async move {
                    let _ = tokio::spawn(async move { launch_game().await }).await.unwrap();
                });
            });

            launcher.on_connect_to_archipelago(move || {
                let _ = slint::spawn_local(async move {
                    let _ = tokio::spawn(async move {
                        let ap_connection = APConnection::new();
                        ap_connection.connect_to_archipelago().await
                    }).await.unwrap();
                });
            });

            launcher.on_close(move || {
                let launcher = launcher_handle.unwrap();
                let _ = launcher.hide();
            });

            launcher.run()?;
        },
        Err(error_message) => {
            let error_message_window = ErrorMessage::new()?;
            error_message_window.set_error_message(error_message.into());
            let error_message_window_handle = error_message_window.as_weak();

            error_message_window.on_close(move || {
                let error_message_window = error_message_window_handle.unwrap();
                let _ = error_message_window.hide();
            });

            error_message_window.run()?;
        }
    }

    Ok(())
}

async fn launch_game() {
    match process::Command::new(LAMULANA_EXECUTABLE_NAME).spawn() {
        Ok(mut p) => {
            let dll = "LaMulanaMW.dll";

            let process_id = p.id();
            let target_process = OwnedProcess::from_pid(process_id).unwrap();
            let syringe = Syringe::for_process(target_process);

            println!("Injecting into {} of PID {} with {}.", LAMULANA_EXECUTABLE_NAME, process_id, dll);
            match syringe.inject(dll) {
                Ok(_) => {
                    println!("Injected and now waiting on process exit.");
                    p.wait().unwrap();
                },
                Err(e) => println!("Could not inject: {}", e)
            }
        },
        Err(e) => {
            println!("Could not launch LaMulanaWin: {:?}", e)
        }
    }
}
