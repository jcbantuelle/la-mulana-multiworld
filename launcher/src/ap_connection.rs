use archipelago_api::api::*;
use archipelago_api::client::APClient;

#[derive(Clone, Debug)]
pub struct APConnection {
}

impl APConnection {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn connect_to_archipelago(&self, connection_name: String, server_url: String, password: String) -> Result<APClient, APError>{
        let mut ap_client = APClient::new(&server_url).await?;
        ap_client.connect(&password, "La-Mulana", &connection_name, None, ItemHandling::OtherWorldsOnly, vec![], true).await.map(|_| { ap_client })
    }
}
