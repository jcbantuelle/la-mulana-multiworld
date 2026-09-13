use archipelago_api::api::*;
use log::debug;
use slint::{ModelRc, VecModel, Weak};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use thiserror::Error;

use crate::ap_connection::APConnection;
use crate::ap_data::{APData, Game, Player};
use crate::file_gen::app_config::AppConfig;
use crate::file_gen::generator;
use crate::windows::launcher::LauncherWindow;

slint::include_modules!();

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
pub struct SeedSelectorWindow {
    ap_data_handle: Arc<Mutex<Option<APData>>>,
    seed_selector: SeedSelector
}

impl SeedSelectorWindow {
    pub fn new(ap_data_handle: Arc<Mutex<Option<APData>>>) -> SeedSelectorWindow {
        SeedSelectorWindow {
            ap_data_handle,
            seed_selector: SeedSelector::new().inspect_err(|e| { debug!("SeedSelectorWindow failed to initialize: {:?}", e); }).unwrap()
        }
    }

    pub fn weak(&self) -> Weak<SeedSelector> {
        self.seed_selector.as_weak()
    }

    pub fn handle(&self) -> SeedSelector {
        self.weak().clone().unwrap()
    }

    pub fn ap_data(&self) -> APData {
        let ap_data = match self.ap_data_handle.lock() {
            Ok(ap_data_lock) => {
                match ap_data_lock.clone() {
                    Some(ap_data) => {
                        Ok(ap_data)
                    },
                    None => {
                        Err("AP_DATA doesn't exist when retrieving from SeedSelector")
                    }
                }
            },
            Err(_) => {
                Err("Failed to acquire AP_DATA lock when retriving from SeedSelector")
            }
        };
        ap_data.inspect_err(|e| { debug!("{:?}", e); }).unwrap()
    }

    pub async fn configure(&self, launcher_window: &LauncherWindow) {
        let seed_selector = self.handle();
        let ap_data = self.ap_data();

        SeedSelectorWindow::update_ap_data(&ap_data, &seed_selector);
        self.configure_close_window(launcher_window).await;
        self.configure_delete(launcher_window).await;
        self.configure_load(launcher_window).await;
        self.configure_add(launcher_window).await;
    }

    pub fn update_ap_data(ap_data: &APData, seed_selector: &SeedSelector) {
        let seeds = Rc::new(VecModel::from(ap_data.seeds()));
        seed_selector.set_seeds(ModelRc::from(seeds));
        seed_selector.set_current_seed(ap_data.seed_name().into());
        seed_selector.set_chosen_seed(ap_data.seed_name().into());
    }

    async fn configure_close_window(&self, launcher_window: &LauncherWindow) {
        let launcher = launcher_window.handle();
        let seed_selector = self.handle();

        self.seed_selector.on_close_window(move || {
            let _ = launcher.show();
            let _ = seed_selector.hide();
        });
    }

    async fn configure_delete(&self, launcher_window: &LauncherWindow) {
        let launcher = launcher_window.handle();
        let seed_selector = self.handle();
        let ap_data_handle = Arc::clone(&self.ap_data_handle);

        self.seed_selector.on_delete(move || {
            let seed_to_delete = seed_selector.get_chosen_seed().to_string();

            match ap_data_handle.lock() {
                Ok(mut ap_data_lock) => {
                    match ap_data_lock.as_mut() {
                        Some(ap_data) => {
                            if ap_data.delete_game(seed_to_delete).is_err() {
                                SeedSelectorWindow::load_seed_error(&seed_selector, "Failed to delete chosen seed");
                            } else {
                                SeedSelectorWindow::load_seed_error(&seed_selector, "");
                                SeedSelectorWindow::update_ap_data(ap_data, &seed_selector);
                                LauncherWindow::update_ap_data(ap_data, &launcher);
                            }
                        },
                        None => {
                            SeedSelectorWindow::load_seed_error(&seed_selector, "AP_DATA doesn't exist during seed deletion");
                        }
                    }
                },
                Err(_) => {
                    SeedSelectorWindow::load_seed_error(&seed_selector, "Failed To acquire AP_DATA lock during Seed Deletion");
                }
            }
        });
    }

    async fn configure_load(&self, launcher_window: &LauncherWindow) {
        let launcher = launcher_window.handle();
        let seed_selector = self.handle();
        let ap_data_handle = Arc::clone(&self.ap_data_handle);

        self.seed_selector.on_load(move || {
            let seed_to_load = seed_selector.get_chosen_seed().to_string();

            match ap_data_handle.lock() {
                Ok(mut ap_data_lock) => {
                    match ap_data_lock.as_mut() {
                        Some(ap_data) => {
                            if ap_data.load_game(seed_to_load).is_err() {
                                SeedSelectorWindow::load_seed_error(&seed_selector, "Failed to load chosen seed");
                            } else {
                                SeedSelectorWindow::load_seed_error(&seed_selector, "");
                                SeedSelectorWindow::update_ap_data(ap_data, &seed_selector);
                                LauncherWindow::update_ap_data(ap_data, &launcher);
                                let _ = launcher.show();
                                let _ = seed_selector.hide();
                            }
                        },
                        None => {
                            SeedSelectorWindow::load_seed_error(&seed_selector, "AP_DATA doesn't exist during seed load");
                        }
                    }
                },
                Err(_) => {
                    SeedSelectorWindow::load_seed_error(&seed_selector, "Failed To Acquire AP_DATA lock during seed load");
                }
            }
        });
    }

    async fn configure_add(&self, launcher_window: &LauncherWindow) {
        let weak_launcher_add = launcher_window.weak();
        let weak_seed_selector_add = self.weak();
        let seed_selector_handle = self.handle();
        let ap_data_handle_add = Arc::clone(&self.ap_data_handle);

        self.seed_selector.on_add_seed(move || {
            let server_url = seed_selector_handle.get_server_url().to_string();
            let password = seed_selector_handle.get_password().to_string();
            let player_name = seed_selector_handle.get_player_name().to_string();

            let weak_launcher = weak_launcher_add.clone();
            let weak_seed_selector = weak_seed_selector_add.clone();
            let ap_data_handle = ap_data_handle_add.clone();

            let _ = slint::spawn_local(async move {
                let _ = tokio::spawn(async move {
                    match SeedSelectorWindow::verify_new_seed(server_url.clone(), password.clone(), player_name.clone()).await {
                        Ok(slot_data) => {
                            let app_config = AppConfig::new(server_url.clone(), password.clone(), slot_data.player_id.clone(), slot_data.players.clone());
                            let local_seed_name = format!("{}-{}", slot_data.seed.clone(), slot_data.player_id.clone());
                            if generator::generate_files(app_config, slot_data.clone(), local_seed_name.clone()).is_err() {
                                let _= weak_seed_selector.upgrade_in_event_loop(move |seed_selector| {
                                    SeedSelectorWindow::add_seed_error(&seed_selector, "New seed files failed to generate");
                                });
                            } else {
                                let game = Game {
                                    seed: local_seed_name.clone(),
                                    server_url: server_url.clone(),
                                    you: Player { id: slot_data.player_id.clone(), name: player_name.clone() },
                                    password: password.clone()
                                };

                                match ap_data_handle.lock() {
                                    Ok(mut ap_data_lock) => {
                                        match ap_data_lock.as_mut() {
                                            Some(ap_data) => {
                                                if ap_data.add_new_game(game).is_err() {
                                                    let _ = weak_seed_selector.upgrade_in_event_loop(move |seed_selector| {
                                                        SeedSelectorWindow::add_seed_error(&seed_selector, "Failed to configure files for new seed");
                                                    });
                                                } else {
                                                    let launcher_ap_data = ap_data.clone();
                                                    let _ = weak_launcher.upgrade_in_event_loop(move |launcher| {
                                                        LauncherWindow::update_ap_data(&launcher_ap_data, &launcher);
                                                        let _ = launcher.show();
                                                    });

                                                    let seed_selector_ap_data = ap_data.clone();
                                                    let _ = weak_seed_selector.upgrade_in_event_loop(move |seed_selector| {
                                                        SeedSelectorWindow::add_seed_error(&seed_selector, "");
                                                        SeedSelectorWindow::update_ap_data(&seed_selector_ap_data, &seed_selector);
                                                        let _ = seed_selector.hide();
                                                    });
                                                }
                                            },
                                            None => {
                                                let _ = weak_seed_selector.upgrade_in_event_loop(move |seed_selector| {
                                                    SeedSelectorWindow::add_seed_error(&seed_selector, "AP_DATA doesn't exist during new seed file generation");
                                                });
                                            }
                                        }
                                    },
                                    Err(_) => {
                                        let _ = weak_seed_selector.upgrade_in_event_loop(move |seed_selector| {
                                            SeedSelectorWindow::add_seed_error(&seed_selector, "Failed To acquire AP_DATA lock during new seed file generation");
                                        });
                                    }
                                }
                            }
                        },
                        Err(_) => {
                            let _ = weak_seed_selector.upgrade_in_event_loop(move |seed_selector| {
                                SeedSelectorWindow::add_seed_error(&seed_selector, "New seed failed to validate");
                            });
                        }
                    }
                }).await.unwrap();
            });
        });
    }

    fn add_seed_error(seed_selector: &SeedSelector, error_message: &str) {
        seed_selector.set_add_seed_error(error_message.to_string().into());
    }

    fn load_seed_error(seed_selector: &SeedSelector, error_message: &str) {
        seed_selector.set_load_seed_error(error_message.to_string().into());
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

}
