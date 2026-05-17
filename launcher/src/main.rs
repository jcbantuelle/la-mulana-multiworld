#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod ap_connection;
pub mod ap_data;
pub mod consts;
pub mod file_gen;
pub mod file_utils;
pub mod verifier;

use archipelago_api::api::*;
use dll_syringe::{process::OwnedProcess, Syringe};
use log::{debug, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use slint::{ComponentHandle, ModelRc, VecModel, Weak};
use std::error::Error;
use std::sync::Mutex;
use std::process;
use std::rc::Rc;
use thiserror::Error;

use crate::ap_connection::APConnection;
use crate::ap_data::{APData, Game, Player};
use crate::consts::*;
use crate::file_gen::generator;
use crate::file_gen::app_config::AppConfig;

slint::include_modules!();

pub static AP_DATA: Mutex<Option<APData>> = Mutex::new(None);
pub static AP_CONNECTION: Mutex<Option<APConnection>> = Mutex::new(None);

#[derive(Clone, Error, Debug)]
pub enum NewSeedError {
    #[error("Unable to connect to Archipelago, please confirm Server URL")]
    ConnectionFailure,
    #[error("Archipelago connection dropped, please try again")]
    ConnectionDropped,
    #[error("Archipelago refused connection, please confirm Player Name and ID")]
    ConnectionRefused,
    #[error("Archipelago rejected the payload, please confirm all software is up to date")]
    InvalidPacket,
    #[error("Archipelago failed to send slot data, please confirm lamulana APworld is up to date")]
    SlotDataMissing,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    configure_logger().await;

    match verifier::verify_install() {
        Ok(lm_config) => {
            let ap_data = APData::new(lm_config)?;
            match AP_DATA.lock() {
                Ok(mut ap_data_lock) => {
                    *ap_data_lock = Some(ap_data.clone());
                },
                Err(e) => {
                    let generate_ap_data_error_message = "Failed To Acquire AP Lock".to_string();
                    debug!("{}: {:?}", generate_ap_data_error_message, e);
                }
            }
            let launcher = Launcher::new().unwrap();
            let seed_selector = SeedSelector::new().unwrap();

            configure_launcher_window(launcher.as_weak(), seed_selector.as_weak(), ap_data.clone()).await;
            configure_seed_selector_window(seed_selector.as_weak(), launcher.as_weak(), ap_data.clone()).await;

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
        .logger(Logger::builder().build("goblin", LevelFilter::Off))
        .build(Root::builder().appender("lmmw_launcher").build(LevelFilter::Debug))
        .unwrap();
    log4rs::init_config(log_config).unwrap();
}

async fn configure_launcher_window(launcher_handle: Weak<Launcher>, seed_selector_handle: Weak<SeedSelector>, ap_data: APData) {
    let launcher = launcher_handle.clone().unwrap();

    launcher.set_seed_selected(ap_data.seed_selected());
    launcher.set_current_seed(ap_data.seed_name().into());

    let launcher_select_seed_handle = launcher_handle.clone();
    let launcher_restore_handle = launcher_handle.clone();
    let launcher_close_handle = launcher_handle.clone();

    let seed_selector_select_handle = seed_selector_handle.clone().unwrap();
    let seed_selector_restore_handle = seed_selector_handle.clone().unwrap();

    launcher.on_select_seed(move || {
        let _ = seed_selector_select_handle.show();

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

    launcher.on_restore(move || {
        let launcher = launcher_restore_handle.unwrap();
        let mut launcher_error_message = "".to_string();

         match AP_DATA.lock() {
            Ok(mut ap_data_lock) => {
                match ap_data_lock.as_mut() {
                    Some(ap_data) => {
                        match ap_data.restore_original_files() {
                            Ok(_) => {
                                launcher.set_seed_selected(ap_data.seed_selected());
                                launcher.set_current_seed(ap_data.seed_name().into());

                                seed_selector_restore_handle.set_current_seed(ap_data.seed_name().into());
                                seed_selector_restore_handle.set_chosen_seed(ap_data.seed_name().into());
                            },
                            Err(e) => {
                                launcher_error_message = "Failed to Restore Original Files".to_string();
                                debug!("{}: {:?}", launcher_error_message, e);
                            }
                        }
                    },
                    None => {
                        launcher_error_message = "AP Data doesn't exist".to_string();
                        debug!("{}", launcher_error_message);
                    }
                }
            },
            Err(e) => {
                launcher_error_message = "Failed To Acquire AP Lock".to_string();
                debug!("{}: {:?}", launcher_error_message, e);
            }
        }

        launcher.set_error_message(launcher_error_message.into());
    });
}

async fn configure_seed_selector_window(seed_selector_handle: Weak<SeedSelector>, launcher_handle: Weak<Launcher>, ap_data: APData) {
    let seed_selector = seed_selector_handle.clone().unwrap();

    let seeds = Rc::new(VecModel::from(ap_data.seeds()));
    seed_selector.set_seeds(ModelRc::from(seeds));
    seed_selector.set_current_seed(ap_data.seed_name().into());
    seed_selector.set_chosen_seed(ap_data.seed_name().into());

    let seed_selector_close_handle = seed_selector_handle.clone();
    let seed_selector_load_handle = seed_selector_handle.clone();
    let seed_selector_delete_handle = seed_selector_handle.clone();
    let seed_selector_add_seed_handle = seed_selector_handle.clone();

    let launcher_close = launcher_handle.clone().unwrap();
    let launcher_load = launcher_handle.clone().unwrap();
    let launcher_delete = launcher_handle.clone().unwrap();
    let launcher_add_seed_handle = launcher_handle.clone();

    seed_selector.on_close(move || {
        let _ = launcher_close.show();

        let seed_selector = seed_selector_close_handle.unwrap();
        let _ = seed_selector.hide();
    });

    seed_selector.on_delete(move || {
        let seed_selector = seed_selector_delete_handle.clone().unwrap();

        let mut seed_error_message = "".to_string();
        let seed_to_delete = seed_selector.get_chosen_seed().to_string();

        match AP_DATA.lock() {
            Ok(mut ap_data_lock) => {
                match ap_data_lock.as_mut() {
                    Some(ap_data) => {
                        match ap_data.delete_game(seed_to_delete.clone()) {
                            Ok(_) => {
                                match ap_data.active_game {
                                    None => {
                                        launcher_delete.set_seed_selected(false);
                                        launcher_delete.set_current_seed(ap_data.seed_name().into());

                                        seed_selector.set_current_seed(ap_data.seed_name().into());
                                    },
                                    _ => ()
                                }
                                let seeds = Rc::new(VecModel::from(ap_data.seeds()));
                                seed_selector.set_seeds(ModelRc::from(seeds));
                            },
                            Err(e) => {
                                seed_error_message = "Failed to Delete Chosen Seed".to_string();
                                debug!("{}: {:?}", seed_error_message, e);
                            }
                        }
                    },
                    None => {
                        seed_error_message = "AP Data doesn't exist".to_string();
                        debug!("{}", seed_error_message);
                    }
                }
            },
            Err(e) => {
                seed_error_message = "Failed To Acquire AP Lock".to_string();
                debug!("{}: {:?}", seed_error_message, e);
            }
        }

        seed_selector.set_load_seed_error(seed_error_message.into());
    });

    seed_selector.on_load(move || {
        let seed_selector = seed_selector_load_handle.clone().unwrap();

        let mut seed_error_message = "".to_string();
        let seed_to_load = seed_selector.get_chosen_seed().to_string();

        match AP_DATA.lock() {
            Ok(mut ap_data_lock) => {
                match ap_data_lock.as_mut() {
                    Some(ap_data) => {
                        match ap_data.load_game(seed_to_load.clone()) {
                            Ok(_) => {
                                launcher_load.set_seed_selected(true);
                                launcher_load.set_current_seed(seed_to_load.clone().into());
                                let _ = launcher_load.show();

                                seed_selector.set_current_seed(seed_to_load.clone().into());
                                let _ = seed_selector.hide();
                            },
                            Err(e) => {
                                seed_error_message = "Failed to Load Chosen Seed".to_string();
                                debug!("{}: {:?}", seed_error_message, e);
                            }
                        }
                    },
                    None => {
                        seed_error_message = "AP Data doesn't exist".to_string();
                        debug!("{}", seed_error_message);
                    }
                }
            },
            Err(e) => {
                seed_error_message = "Failed To Acquire AP Lock".to_string();
                debug!("{}: {:?}", seed_error_message, e);
            }
        }

        seed_selector.set_load_seed_error(seed_error_message.into());
    });

    seed_selector.on_add_seed(move || {
        let seed_selector = seed_selector_add_seed_handle.clone().unwrap();

        let server_url = seed_selector.get_server_url().to_string();
        let password = seed_selector.get_password().to_string();
        let player_name = seed_selector.get_player_name().to_string();

        let seed_selector_text_handle = seed_selector_add_seed_handle.clone();
        let seed_selector_close_handle = seed_selector_add_seed_handle.clone();
        let launcher_open_handle = launcher_add_seed_handle.clone();

        let _ = slint::spawn_local(async move {
            let _ = tokio::spawn(async move {
                let mut seed_error_message = "".to_string();
                match verify_new_seed(server_url.clone(), password.clone(), player_name.clone()).await {
                    Ok(slot_data) => {
                        let app_config = AppConfig::new(server_url.clone(), password.clone(), slot_data.player_id.clone(), slot_data.players.clone());
                        let local_seed_name = format!("{}-{}", slot_data.seed.clone(), slot_data.player_id.clone());
                        match generator::generate_files(app_config, slot_data.clone(), local_seed_name.clone()) {
                            Ok(_) => {
                                let game = Game {
                                    seed: local_seed_name.clone(),
                                    server_url: server_url.clone(),
                                    you: Player { id: slot_data.player_id.clone(), name: player_name.clone() },
                                    password: password.clone()
                                };

                                // Set Current Seed, switch back to launcher window
                                match AP_DATA.lock() {
                                    Ok(mut ap_data_lock) => {
                                        match ap_data_lock.as_mut() {
                                            Some(ap_data) => {
                                                match ap_data.add_new_game(game) {
                                                    Ok(_) => {
                                                        let seed_selected = ap_data.seed_selected();
                                                        let launcher_current_seed = ap_data.seed_name().clone();
                                                        let selector_current_seed = ap_data.seed_name().clone();
                                                        let seeds = ap_data.seeds().clone();

                                                        let _ = launcher_open_handle.upgrade_in_event_loop(move |launcher| {
                                                            launcher.set_seed_selected(seed_selected);
                                                            launcher.set_current_seed(launcher_current_seed.into());
                                                            let _ = launcher.show();
                                                        }).unwrap();

                                                        let _ = seed_selector_close_handle.upgrade_in_event_loop(move |seed_selector| {
                                                            seed_selector.set_current_seed(selector_current_seed.clone().into());
                                                            seed_selector.set_chosen_seed(selector_current_seed.clone().into());
                                                            let seeds = Rc::new(VecModel::from(seeds));
                                                            seed_selector.set_seeds(ModelRc::from(seeds));
                                                            let _ = seed_selector.hide();
                                                        }).unwrap();
                                                    },
                                                    Err(e) => {
                                                        seed_error_message = "Failed to Configure Files for New Game".to_string();
                                                        debug!("{}: {:?}", seed_error_message, e);
                                                    }
                                                }
                                            },
                                            None => {
                                                seed_error_message = "AP Data doesn't exist".to_string();
                                                debug!("{}", seed_error_message);
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        seed_error_message = "Failed To Acquire AP Lock".to_string();
                                        debug!("{}: {:?}", seed_error_message, e);
                                    }
                                }
                            },
                            Err(e) => {
                                seed_error_message = "Files failed to Generate".to_string();
                                debug!("{}: {:?}", seed_error_message, e);
                            }
                        }
                    },
                    Err(e) => {
                        seed_error_message = "Seed Failed to Validate".to_string();
                        debug!("{}: {:?}", seed_error_message, e);
                    }
                }

                let _ = seed_selector_text_handle.upgrade_in_event_loop(move |seed_selector| {
                    seed_selector.set_add_seed_error(seed_error_message.into());
                }).unwrap();
            }).await.unwrap();
        });
    });
}

async fn verify_new_seed(server_url: String, password: String, player_name: String) -> Result<SlotData, NewSeedError> {
    let ap_connection = APConnection::new();
    let mut ap_client = ap_connection.connect_to_archipelago(player_name, server_url, password).await.map_err(|_| NewSeedError::ConnectionFailure)?;
    loop {
        let payload = ap_client.read().await.map_err(|_| NewSeedError::ConnectionDropped)?;
        match payload {
            ServerPayload::Connected(connected) => {
                return connected.slot_data.ok_or(NewSeedError::SlotDataMissing);
            },
            ServerPayload::ConnectionRefused(connection_refused) => {
                debug!("Connection Refused: {:?}", connection_refused);
                return Err(NewSeedError::ConnectionRefused);
            },
            ServerPayload::InvalidPacket(invalid_packet) => {
                debug!("Invalid Packet: {:?}", invalid_packet);
                return Err(NewSeedError::InvalidPacket);
            },
            _ => ()
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

            match syringe.inject(dll) {
                Ok(_) => {
                    p.wait().unwrap();
                },
                Err(e) => debug!("Failed to inject DLL: {}", e)
            }
        },
        Err(e) => {
            debug!("Could not launch LaMulanaWin: {:?}", e)
        }
    }
}
