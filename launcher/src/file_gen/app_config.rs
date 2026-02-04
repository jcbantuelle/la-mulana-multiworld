use serde::{Serialize, Deserialize};

use crate::archipelago::api::{ArchipelagoPlayer, ItemData, Location};
use crate::file_gen::lm_consts::GLOBAL_FLAGS;

#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub server_url: String,
    pub password: String,
    pub log_file_name: String,
    pub local_player_id: i64,
    pub log_level: String,
    pub players: Vec<ArchipelagoPlayer>,
    pub item_mapping: Vec<ArchipelagoItem>,
    next_filler_flag: u16
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct ArchipelagoItem {
    pub flag: u16,
    pub location_id: i64,
    pub player_id: i64,
    pub obtain_value: u8
}

impl AppConfig {
    pub fn new(server_url: String, password: String, player_id: String, players: Vec<ArchipelagoPlayer>) -> Self {
        AppConfig {
            server_url,
            password,
            log_file_name: "lamulanamw.txt".to_string(),
            local_player_id: player_id.parse::<i64>().unwrap(),
            log_level: "DEBUG".to_string(),
            players,
            item_mapping: Vec::new(),
            next_filler_flag: GLOBAL_FLAGS["filler_items"] as u16
        }
    }

    pub fn add_item(&mut self, item: Option<&ItemData>, item_id: i64, location: &Location) -> u16 {
        let flag: u16 = if item_id == 38 || item_id == 83 || item.is_none() || item.unwrap().obtain_flag.is_none() {
            self.filler_flag()
        } else {
            item.unwrap().obtain_flag.unwrap() as u16
        };

        let ap_item = ArchipelagoItem {
            flag,
            location_id: location.address.unwrap() as i64,
            player_id: location.item.as_ref().unwrap().player as i64,
            obtain_value: 2
        };

        self.item_mapping.push(ap_item);

        flag
    }

    fn filler_flag(&mut self) -> u16 {
        let next_flag = self.next_filler_flag;
        self.next_filler_flag += 1;
        next_flag
    }
}
