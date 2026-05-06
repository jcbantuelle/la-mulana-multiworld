use crate::archipelago::api::*;
use crate::archipelago::client::APClient;

#[derive(Clone, Debug)]
pub struct APConnection {
}

impl APConnection {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn connect_to_archipelago(&self, connection_name: String, server_url: String, password: String, player_id: i64) -> Result<APClient, APError>{
        let mut ap_client = APClient::new(&server_url).await?;
        ap_client.connect(&password, "La-Mulana", &connection_name, player_id, ItemHandling::OtherWorldsOnly, vec![], true).await.map(|_| { ap_client })
    }
}
