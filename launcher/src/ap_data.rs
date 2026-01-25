use log::debug;
use serde::{Serialize, Deserialize};

use crate::consts::{AP_PATH};
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
    pub you: Player,
    pub password: String,
    pub players: Vec<Player>,
    pub items: Vec<Item>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Player {
    pub id: i64,
    pub name: String
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Item {
    pub flag: u16,
    pub location_id: i64,
    pub player_id: i64,
    pub obtain_value: u8
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct APData {
    pub config: LaMulanaConfig,
    pub games: Vec<Game>,
    pub active_game: Option<Game>
}

impl APData {
    pub fn new(lm_config: LaMulanaConfig) -> Result<APData, String> {
        let data_path = format!("{}ap_data.json", AP_PATH);
        let ap_data_found = file_utils::path_exists(&data_path, false)?;

        if ap_data_found {
            let serialized_ap_data = file_utils::read_file_as_string(&data_path)?;
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
        let serialized_ap_data = serde_json::to_string::<APData>(&ap_data).map_err(|e| {
            format!("Error {} while attempting to serialize AP Data.", e)
        })?;
        file_utils::write_file(&data_path, &serialized_ap_data)?;
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
}
