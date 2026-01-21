use log::debug;

use crate::archipelago::api::*;
use crate::archipelago::client::APClient;

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
