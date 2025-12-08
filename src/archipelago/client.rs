use log::debug;
use ratchet_rs::{subscribe_with, WebSocket, WebSocketConfig, deflate::{DeflateExtProvider, Deflate}, SubprotocolRegistry};
use serde::Serialize;
use std::collections::HashMap;
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

    // Client -> Server Communication

    pub async fn connect(&mut self, password: &str, game: &str, name: &str, uuid: i64, items_handling: ItemHandling, tags: Vec<String>, slot_data: bool) -> Result<(), APError> {
        let version = NetworkVersion {
            class: "Version".to_string(),
            build: 0,
            major: 6,
            minor: 4
        };

        let connect = Connect {
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

    pub async fn connect_update(&mut self, items_handling: ItemHandling, tags: Vec<String>) -> Result<(), APError> {
        let connect_update = ConnectUpdate {
            items_handling,
            tags
        };

        let connect_update_payload = serde_json::to_string(&[connect_update]);
        self.send(connect_update_payload).await
    }

    pub async fn sync(&mut self) -> Result<(), APError> {
        let sync = Sync{};

        let sync_payload = serde_json::to_string(&[sync]);
        self.send(sync_payload).await
    }

    pub async fn location_checks(&mut self, locations: Vec<i64>) -> Result<(), APError> {
        let location_checks = LocationChecks {
            locations
        };

        let location_checks_payload = serde_json::to_string(&[location_checks]);
        self.send(location_checks_payload).await
    }

    pub async fn location_scouts(&mut self, locations: Vec<i64>, create_as_hint: i64) -> Result<(), APError> {
        let location_scouts = LocationScouts {
            locations,
            create_as_hint
        };

        let location_scouts_payload = serde_json::to_string(&[location_scouts]);
        self.send(location_scouts_payload).await
    }

    pub async fn create_hints(&mut self, locations: Vec<i64>, player: i64, status: HintStatus) -> Result<(), APError> {
        let create_hints = CreateHints {
            locations,
            player,
            status
        };

        let create_hints_payload = serde_json::to_string(&[create_hints]);
        self.send(create_hints_payload).await
    }

    pub async fn update_hint(&mut self, player: i64, location: i64, status: HintStatus) -> Result<(), APError> {
        let update_hint = UpdateHint {
            player,
            location,
            status
        };

        let update_hint_payload = serde_json::to_string(&[update_hint]);
        self.send(update_hint_payload).await
    }

    pub async fn status_update(&mut self, status: ClientStatus) -> Result<(), APError> {
        let status_update = StatusUpdate {
            status
        };

        let status_update_payload = serde_json::to_string(&[status_update]);
        self.send(status_update_payload).await
    }

    pub async fn say(&mut self, text: String) -> Result<(), APError> {
        let say = Say {
            text
        };

        let say_payload = serde_json::to_string(&[say]);
        self.send(say_payload).await
    }

    pub async fn get_data_package(&mut self, games: Vec<String>) -> Result<(), APError> {
        let get_data_package = GetDataPackage {
            games
        };

        let get_data_package_payload = serde_json::to_string(&[get_data_package]);
        self.send(get_data_package_payload).await
    }

    pub async fn bounce(&mut self, games: Vec<String>, slots: Vec<i64>, tags: Vec<String>, data: HashMap<String, String>) -> Result<(), APError> {
        let bounce = Bounce {
            games,
            slots,
            tags,
            data
        };

        let bounce_payload = serde_json::to_string(&[bounce]);
        self.send(bounce_payload).await
    }

    pub async fn get(&mut self, keys: Vec<String>) -> Result<(), APError> {
        let get = Get {
            keys
        };

        let get_payload = serde_json::to_string(&[get]);
        self.send(get_payload).await
    }

    pub async fn set<T: Serialize>(&mut self, key: String, default: T, want_reply: bool, operations: Vec<DataStorageOperation<T>>) -> Result<(), APError> {
        let set = Set {
            key,
            default,
            want_reply,
            operations
        };

        let set_payload = serde_json::to_string(&[set]);
        self.send(set_payload).await
    }

    pub async fn set_notify(&mut self, keys: Vec<String>) -> Result<(), APError> {
        let set_notify = SetNotify {
            keys
        };

        let set_notify_payload = serde_json::to_string(&[set_notify]);
        self.send(set_notify_payload).await
    }
}
