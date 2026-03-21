use log::debug;
use serde::{Serialize, Deserialize};

use crate::consts::{AP_DATA_PATH, AP_PATH};
use crate::file_utils;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LaMulanaConfig {
    pub version: String,
    pub save_path: String,
    pub rcd_digest: String,
    pub dat_digest: String,
    pub effects_digest: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Game {
    pub seed: String,
    pub server_url: String,
    pub you: Player,
    pub password: String
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Player {
    pub id: i64,
    pub name: String
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct APData {
    pub config: LaMulanaConfig,
    pub games: Vec<Game>,
    pub active_game: Option<Game>
}

impl APData {
    pub fn new(lm_config: LaMulanaConfig) -> Result<APData, String> {
        let ap_data_found = file_utils::path_exists(&AP_DATA_PATH, false)?;

        if ap_data_found {
            let serialized_ap_data = file_utils::read_file_as_string(&AP_DATA_PATH)?;
            match serde_json::from_str::<APData>(&serialized_ap_data) {
                Ok(mut ap_data) => {
                    ap_data.config = lm_config;
                    return Ok(ap_data);
                },
                Err(e) => {
                    debug!("Error {} while attempting to deserialize AP Data, regenerating", e);
                }
            }
        }

        let ap_data = APData { config: lm_config, games: Vec::new(), active_game: None };
        ap_data.serialize_data()?;

        Ok(ap_data)
    }

    pub fn seed_selected(&self) -> bool {
        self.active_game.is_some()
    }

    pub fn seed_name(&self) -> String {
        match &self.active_game {
            Some(active_game) => {
                active_game.seed.clone()
            },
            None => {
                "No Seed Selected".to_string()
            }
        }
    }

    pub fn add_new_game(&mut self, game: Game) -> Result<(), String> {
        if let Some(active_game) = &self.active_game {
            let save_destination = format!("{}{}/save/", AP_PATH, active_game.seed);
            file_utils::move_saves(self.config.save_path.clone(), save_destination)?;
        }

        self.active_game = Some(game.clone());
        self.games.push(game.clone());

        self.serialize_data()?;
        file_utils::update_game_files(game.seed, self.config.save_path.clone())?;

        Ok(())
    }

    fn serialize_data(&self) -> Result<(), String> {
        let serialized_ap_data = serde_json::to_string::<APData>(&self).map_err(|e| {
            format!("Error {} while attempting to serialize AP Data.", e)
        })?;
        file_utils::write_file(&AP_DATA_PATH, &serialized_ap_data)
    }
}
