use serde::{Serialize, Deserialize};

use crate::consts::{AP_PATH};
use crate::file_utils;

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
    pub games: Vec<Game>,
    pub active_game: Option<Game>
}

impl APData {
    pub fn new() -> Result<APData, String> {
        let data_path = format!("{}ap_data.json", AP_PATH);
        if file_utils::path_exists(&data_path, false)? {
            let serialized_ap_data = file_utils::read_file_as_string(&data_path)?;
            serde_json::from_str::<APData>(&serialized_ap_data).map_err(|e| {
                format!("Error {} while attempting to deserialize AP Data.", e)
            })
        } else {
            let ap_data = APData { games: Vec::new(), active_game: None };
            let serialized_ap_data = serde_json::to_string::<APData>(&ap_data).map_err(|e| {
                format!("Error {} while attempting to serialize AP Data.", e)
            })?;
            file_utils::write_file(&data_path, &serialized_ap_data)?;
            Ok(ap_data)
        }
    }
}
