use log::debug;
use ratchet_rs::{subscribe_with, WebSocket, WebSocketConfig, deflate::{DeflateExtProvider, Deflate}, SubprotocolRegistry};
use super::api::*;
use tokio::net::TcpStream;

pub struct APClient {
    websocket: WebSocket<tokio::net::TcpStream, Deflate>
}

impl APClient {
    pub async fn new(url: &str) -> Result<APClient, APError> {
        let tcp_connection = TcpStream::connect(url).await;
        match tcp_connection {
            Ok(stream) => {
                let websocket_url = format!("wss://{url}");
                match subscribe_with(WebSocketConfig::default(), stream, websocket_url, DeflateExtProvider::default(), SubprotocolRegistry::default()).await {
                    Ok(websocket_stream) => {
                        Ok(APClient{ websocket: websocket_stream.into_websocket() })
                    },
                    Err(e) => {
                        debug!("Websocket Connection Failed with error {}", e);
                        Err(APError::WebsocketConnectionFailure)
                    }
                }
            },
            Err(e) => {
                Err(APError::ServerConnectionFailure)
            }
        }
    }

    async fn send(&mut self, payload: Result<String, serde_json::Error>) -> Result<(), APError> {
        match payload {
            Ok(serialized_payload) => {
                match self.websocket.write(serialized_payload, ratchet_rs::PayloadType::Text).await {
                    Ok(result) => {
                        Ok(result)
                    },
                    Err(_) => {
                        Err(APError::PayloadSendFailure)
                    }
                }
            },
            Err(_) => {
                Err(APError::PayloadSerializationFailure)
            }
        }
    }

    pub async fn connect(&mut self, password: &str, game: &str, name: &str, uuid: i64, items_handling: u8, tags: Vec<String>, slot_data: bool) -> Result<(), APError> {
        let version = NetworkVersion {
            class: "Version".to_string(),
            build: 0,
            major: 6,
            minor: 4
        };

        let connect = Connect{
            password: password.to_string(),
            game: game.to_string(),
            name: name.to_string(),
            uuid,
            version,
            items_handling,
            tags,
            slot_data
        };

        let connect_payload = serde_json::to_string(&[connect]);
        self.send(connect_payload).await
    }

    pub async fn sync(&mut self) -> Result<(), APError> {
        let sync = Sync{};

        let sync_payload = serde_json::to_string(&[sync]);
        self.send(sync_payload).await
    }
}
