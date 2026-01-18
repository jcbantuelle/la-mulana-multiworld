use log::debug;
use serde::{Serialize, Deserialize};

use crate::archipelago::api::*;
use crate::archipelago::client::APClient;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct APData {
    pub games: Vec<Game>,
    pub active_game: Option<Game>
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
pub struct Game {
    pub seed: String,
    pub you: Player,
    pub password: String,
    pub players: Vec<Player>,
    pub items: Vec<Item>
}

#[derive(Clone, Debug)]
pub struct APConnection {
}

impl APConnection {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn connect_to_archipelago(&self) {
        let mut randomizer = APClient::new("localhost:6969").await;
        match randomizer.as_mut() {
            Ok(ap_client) => {
                let player_id = 1;
                let player_name = "Justin";
                let password = "";
                match ap_client.connect(password, "La-Mulana", &player_name, player_id, ItemHandling::OtherWorldsOnly, vec![], true).await {
                    Ok(_) => {},
                    Err(e) => {
                        debug!("Connect Failure with error {:?}", e);
                    }
                }
            },
            Err(e) => {
                debug!("AP Client Not Connected with Error {}", e);
            }
        };
    }
}
