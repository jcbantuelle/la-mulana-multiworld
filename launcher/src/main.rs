#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod ap_connection;
pub mod ap_data;
pub mod archipelago;
pub mod consts;
pub mod file_utils;
pub mod verifier;

use log::{debug, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};

use dll_syringe::{process::OwnedProcess, Syringe};
use slint::ComponentHandle;
use std::error::Error;
use std::sync::Mutex;
use std::process;

use crate::ap_connection::APConnection;
use crate::ap_data::APData;
use crate::archipelago::api::*;
use crate::consts::*;

slint::include_modules!();

pub static AP_DATA: Mutex<Option<APData>> = Mutex::new(None);
pub static AP_CONNECTION: Mutex<Option<APConnection>> = Mutex::new(None);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let file_appender = FileAppender::builder()
        .build("lmmw_launcher.txt")
        .unwrap();
    let log_config = Config::builder()
        .appender(Appender::builder().build("lmmw_launcher", Box::new(file_appender)))
        .build(Root::builder().appender("lmmw_launcher").build(LevelFilter::Debug))
        .unwrap();
    log4rs::init_config(log_config).unwrap();

    match verifier::verify_install() {
        Ok(lm_config) => {
            let ap_data = APData::new(lm_config)?;
            let launcher = Launcher::new().unwrap();
            launcher.set_seed_selected(ap_data.seed_selected());
            launcher.set_current_seed(ap_data.seed_name().into());

            let launcher_select_seed_handle = launcher.as_weak().clone();
            let launcher_open_handle = launcher.as_weak().clone();
            let launcher_close_handle = launcher.as_weak().clone();

            let seed_selector = SeedSelector::new().unwrap();
            let seed_selector_close_handle = seed_selector.as_weak().clone();
            let seed_selector_add_seed_handle = seed_selector.as_weak().clone();

            seed_selector.on_close(move || {
                let launcher = launcher_open_handle.unwrap();
                let _ = launcher.show();

                let seed_selector = seed_selector_close_handle.unwrap();
                let _ = seed_selector.hide();
            });

            seed_selector.on_add_seed(move || {
                let seed_selector = seed_selector_add_seed_handle.unwrap();

                let server_url = seed_selector.get_server_url().to_string();
                let password = seed_selector.get_password().to_string();
                let player_id_text = seed_selector.get_player_id().to_string();

                let _ = slint::spawn_local(async move {
                    let _ = tokio::spawn(async move {
                        match player_id_text.parse::<i64>() {
                            Ok(player_id) => {
                                let ap_connection = APConnection::new();
                                match ap_connection.connect_to_archipelago("File Generator".to_string(), server_url, password, player_id).await {
                                    Ok(mut ap_client) => {
                                        loop {
                                            match ap_client.read().await {
                                                Ok(payload) => {
                                                    match payload {
                                                        ServerPayload::Connected(connected) => {
                                                            match connected.slot_data {
                                                                Some(slot_data) => {
                                                                    debug!("Slot Data: {:?}", slot_data);
                                                                    // Generate Files
                                                                },
                                                                None => {
                                                                    // Display Validation Error on No Slot Data for File Gen
                                                                }
                                                            }
                                                            break;
                                                        },
                                                        _ => { debug!("Got payload other than Connected from AP Connection: {:?}", payload) }
                                                    }
                                                },
                                                Err(e) => {
                                                    // Display Validation Eerror on AP Read
                                                    break;
                                                }
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        // Display Validation Error on Connecting to AP
                                    }
                                }
                            },
                            Err(e) => {
                                // Display Validation Error on Player ID
                            }
                        }
                    }).await.unwrap();
                });
            });

            launcher.on_select_seed(move || {
                let _ = seed_selector.show();

                let launcher = launcher_select_seed_handle.unwrap();
                let _ = launcher.hide();
            });

            launcher.on_close(move || {
                let launcher = launcher_close_handle.unwrap();
                let _ = launcher.hide();
            });

            launcher.on_launch_game(move || {
                let _ = slint::spawn_local(async move {
                    let _ = tokio::spawn(async move { launch_game().await }).await.unwrap();
                });
            });

            launcher.on_connect_to_archipelago(move || {
                // let _ = slint::spawn_local(async move {
                //     let _ = tokio::spawn(async move {
                //         let ap_connection = APConnection::new();
                //         ap_connection.connect_to_archipelago().await
                //     }).await.unwrap();
                // });
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
