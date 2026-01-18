#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod ap_connection;
pub mod archipelago;
pub mod consts;
pub mod file_utils;
pub mod verifier;

use dll_syringe::{process::OwnedProcess, Syringe};
use slint::ComponentHandle;
use std::process;
use std::error::Error;

use crate::ap_connection::{APConnection, APData};
use crate::consts::*;
use crate::verifier::Verifier;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let verifier = Verifier::new();

    match verifier.verify_install() {
        Ok(_) => {
            let _ = load_ap_data()?;

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

fn load_ap_data() -> Result<(), String> {
    let mut ap_data = AP_DATA.lock().map_err(|e| {
        format!("Error {} while attempting to obtain AP Data lock.", e)
    })?;
    if file_utils::path_exists(&AP_DATA_PATH, false)? {
        let serialized_ap_data = file_utils::read_file_as_string(&AP_DATA_PATH)?;
        let deserialized_ap_data = serde_json::from_str::<APData>(&serialized_ap_data).map_err(|e| {
            format!("Error {} while attempting to deserialize AP Data.", e)
        })?;
        *ap_data = deserialized_ap_data;
    } else {
        let serialized_ap_data = serde_json::to_string::<APData>(&ap_data).map_err(|e| {
            format!("Error {} while attempting to serialize AP Data.", e)
        })?;
        file_utils::write_file(&AP_DATA_PATH, &serialized_ap_data)?;
    }
    Ok(())
}

async fn launch_game() {
    match process::Command::new(LAMULANA_EXECUTABLE_NAME).spawn() {
        Ok(mut p) => {
            let process_id = p.id();
            let target_process = OwnedProcess::from_pid(process_id).unwrap();
            let syringe = Syringe::for_process(target_process);

            println!("Injecting into {} of PID {} with {}.", LAMULANA_EXECUTABLE_NAME, process_id, LAMULANA_MW_DLL_NAME);
            match syringe.inject(LAMULANA_MW_DLL_NAME) {
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
