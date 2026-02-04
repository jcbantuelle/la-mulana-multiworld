#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod ap_connection;
pub mod ap_data;
pub mod archipelago;
pub mod consts;
pub mod file_gen;
pub mod file_utils;
pub mod verifier;

use dll_syringe::{process::OwnedProcess, Syringe};
use log::{debug, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use slint::{ComponentHandle, Weak};
use std::error::Error;
use std::sync::Mutex;
use std::process;
use thiserror::Error;

use crate::ap_connection::APConnection;
use crate::ap_data::APData;
use crate::archipelago::api::*;
use crate::consts::*;
use crate::file_gen::generator;
use crate::file_gen::app_config::AppConfig;

slint::include_modules!();

pub static AP_DATA: Mutex<Option<APData>> = Mutex::new(None);
pub static AP_CONNECTION: Mutex<Option<APConnection>> = Mutex::new(None);

#[derive(Clone, Error, Debug)]
pub enum NewSeedError {
    #[error("Player ID is not numeric")]
    InvalidPlayerId,
    #[error("Unable to connect to Archipelago, please confirm Server URL")]
    ConnectionFailure,
    #[error("Archipelago connection dropped, please try again")]
    ConnectionDropped,
    #[error("Archipelago refused connection, please confirm Player Name and ID")]
    ConnectionRefused,
    #[error("Archipelago failed to send slot data, please confirm lamulana APworld is up to date")]
    SlotDataMissing,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    configure_logger().await;

    match verifier::verify_install() {
        Ok(lm_config) => {
            let ap_data = APData::new(lm_config)?;
            let launcher = Launcher::new().unwrap();
            let seed_selector = SeedSelector::new().unwrap();

            configure_launcher_window(launcher.as_weak(), seed_selector.as_weak(), ap_data.clone()).await;
            configure_seed_selector_window(seed_selector.as_weak(), launcher.as_weak()).await;

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

async fn configure_logger() {
    let file_appender = FileAppender::builder()
        .build("lmmw_launcher.txt")
        .unwrap();
    let log_config = Config::builder()
        .appender(Appender::builder().build("lmmw_launcher", Box::new(file_appender)))
        .build(Root::builder().appender("lmmw_launcher").build(LevelFilter::Debug))
        .unwrap();
    log4rs::init_config(log_config).unwrap();
}

async fn configure_launcher_window(launcher_handle: Weak<Launcher>, seed_selector_handle: Weak<SeedSelector>, ap_data: APData) {
    let launcher = launcher_handle.unwrap();
    let seed_selector = seed_selector_handle.unwrap();

    launcher.set_seed_selected(ap_data.seed_selected());
    launcher.set_current_seed(ap_data.seed_name().into());

    let launcher_select_seed_handle = launcher.as_weak();
    let launcher_close_handle = launcher.as_weak();

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
}

async fn configure_seed_selector_window(seed_selector_handle: Weak<SeedSelector>, launcher_handle: Weak<Launcher>) {
    let seed_selector = seed_selector_handle.clone().unwrap();
    let launcher = launcher_handle.unwrap();

    let seed_selector_close_handle = seed_selector_handle.clone();
    let seed_selector_add_seed_handle = seed_selector_handle.clone();

    seed_selector.on_close(move || {
        let _ = launcher.show();

        let seed_selector = seed_selector_close_handle.unwrap();
        let _ = seed_selector.hide();
    });

    seed_selector.on_add_seed(move || {
        let seed_selector = seed_selector_add_seed_handle.clone().unwrap();
        seed_selector.set_add_seed_error("Connecting...".into());

        let server_url = seed_selector.get_server_url().to_string();
        let password = seed_selector.get_password().to_string();
        let player_id_text = seed_selector.get_player_id().to_string();
        let player_name = seed_selector.get_player_name().to_string();

        let seed_selector_error_handle = seed_selector_add_seed_handle.clone();

        let _ = slint::spawn_local(async move {
            let _ = tokio::spawn(async move {
                match verify_new_seed(server_url.clone(), password.clone(), player_id_text.clone(), player_name.clone()).await {
                    Ok(slot_data) => {
                        debug!("{:?}", slot_data);
                        let app_config = AppConfig::new(server_url, password, player_id_text, slot_data.players.clone());
                        match generator::generate_files(app_config, slot_data) {
                            Ok(_) => {
                                // Set Current Seed, switch back to launcher window
                            },
                            Err(e) => {
                                // Update Add Seed Error text for whatever went wrong
                            }
                        }
                    },
                    Err(e) => {
                        debug!("Seed Failed to Validate with Error: {}", e);
                        let _ = seed_selector_error_handle.upgrade_in_event_loop(move |seed_selector| {
                            seed_selector.set_add_seed_error(e.to_string().into());
                        }).unwrap();
                    }
                }
            }).await.unwrap();
        });
    });
}

async fn verify_new_seed(server_url: String, password: String, player_id_text: String, player_name: String) -> Result<SlotData, NewSeedError> {
    let player_id = player_id_text.parse::<i64>().map_err(|_| NewSeedError::InvalidPlayerId)?;
    let ap_connection = APConnection::new();
    let mut ap_client = ap_connection.connect_to_archipelago(player_name, server_url, password, player_id).await.map_err(|_| NewSeedError::ConnectionFailure)?;
    loop {
        let payload = ap_client.read().await.map_err(|_| NewSeedError::ConnectionDropped)?;
        match payload {
            ServerPayload::Connected(connected) => {
                return connected.slot_data.ok_or(NewSeedError::SlotDataMissing);
            },
            ServerPayload::ConnectionRefused(_) => {
                return Err(NewSeedError::ConnectionRefused);
            },
            _ => { debug!("Got payload other than Connected from AP Connection: {:?}", payload); }
        }
    }
}

async fn launch_game() {
    match process::Command::new(LAMULANA_EXECUTABLE_NAME).spawn() {
        Ok(mut p) => {
            let dll = "LaMulanaMW.dll";

            let process_id = p.id();
            let target_process = OwnedProcess::from_pid(process_id).unwrap();
            let syringe = Syringe::for_process(target_process);

            debug!("Injecting into {} of PID {} with {}.", LAMULANA_EXECUTABLE_NAME, process_id, dll);
            match syringe.inject(dll) {
                Ok(_) => {
                    debug!("Injected and now waiting on process exit.");
                    p.wait().unwrap();
                },
                Err(e) => debug!("Could not inject: {}", e)
            }
        },
        Err(e) => {
            debug!("Could not launch LaMulanaWin: {:?}", e)
        }
    }
}
