use archipelago_api::api::{ArchipelagoPlayer, ItemData, Location};
use log::debug;
use serde::{Serialize, Deserialize};

use crate::file_gen::generator::FileGenerationError;
use crate::file_gen::lm_consts::{GLOBAL_FLAGS, ITEM_CODES};

#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig {
    next_filler_flag: i16,
    pub server_url: String,
    pub password: String,
    pub log_file_name: String,
    pub local_player_id: i64,
    pub log_level: String,
    pub players: Vec<ArchipelagoPlayer>,
    pub item_mapping: Vec<ArchipelagoItem>
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct ArchipelagoItem {
    pub flag: i16,
    pub location_id: i64,
    pub player_id: i64,
    pub obtain_value: u8
}

impl AppConfig {
    pub fn new(server_url: String, password: String, player_id: i64, players: Vec<ArchipelagoPlayer>) -> Self {
        AppConfig {
            server_url,
            password,
            log_file_name: "lamulanamw.txt".to_string(),
            local_player_id: player_id,
            log_level: "DEBUG".to_string(),
            players,
            item_mapping: Vec::new(),
            next_filler_flag: GLOBAL_FLAGS["filler_items"]
        }
    }

    pub fn add_item(&mut self, item: ItemData, item_id: i16, location: &Location) -> Result<i16, FileGenerationError> {
        let flag = match item.obtain_flag {
            Some(obtain_flag) => {
                if item_id == ITEM_CODES["Shell Horn"] || item_id == ITEM_CODES["Holy Grail (Full)"] {
                    self.filler_flag()
                } else {
                    obtain_flag
                }
            },
            None => self.filler_flag()
        };

        let location_id = location.address.clone().ok_or_else(|| {
            debug!("Address field was missing on Location in Slot Data: {:?}", location);
            FileGenerationError::MalformedSlotData
        })?;

        let player_id = location.item.clone().ok_or_else(|| {
            debug!("Item field was missing on Location in Slot Data: {:?}", location);
            FileGenerationError::MalformedSlotData
        })?.player;

        let ap_item = ArchipelagoItem {
            flag,
            location_id,
            player_id,
            obtain_value: 2
        };

        self.item_mapping.push(ap_item);

        Ok(flag)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, FileGenerationError> {
        let app_config = toml::to_vec(&self).map_err(|e| {
            debug!("Serilization Failure with error: {}", e);
            FileGenerationError::AppConfigSerializeFailure
        })?;
        Ok(app_config)
    }

    fn filler_flag(&mut self) -> i16 {
        let next_flag = self.next_filler_flag;
        self.next_filler_flag += 1;
        next_flag
    }
}
