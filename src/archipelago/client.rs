use bytes::BytesMut;
use log::debug;
use ratchet_rs::{deflate::{Deflate, DeflateExtProvider}, Message, SubprotocolRegistry, subscribe_with, UpgradedClient, WebSocket, WebSocketConfig, WebSocketStream};
use std::collections::HashMap;
use super::api::*;
use tokio::net::TcpStream;
use tokio_native_tls::native_tls::TlsConnector;

pub struct APClient {
    websocket: WebSocket<Box<dyn WebSocketStream>, Deflate>
}

impl APClient {
    pub async fn new(url: &str) -> Result<APClient, APError> {
        let tcp_stream_for_tls = Self::tcp_connect(url).await?;
        let tls_connector = match TlsConnector::builder().build() {
            Ok(connector) => tokio_native_tls::TlsConnector::from(connector),
            Err(_) => return Err(APError::TlsConnectorFailure)
        };

        let domain = match url.find(":") {
            None => url,
            Some(port_index) => {
                &url[..port_index]
            }
        };
        let tls_connection = tls_connector.connect(domain, tcp_stream_for_tls).await;

        let websocket_stream: Result<UpgradedClient<Box<dyn WebSocketStream>,Deflate>, _> = match tls_connection {
            Ok(tls_stream) => {
                let secure_websocket_url = format!("wss://{url}");
                let tls_websocket: Box<dyn WebSocketStream> = Box::new(tls_stream);
                subscribe_with(WebSocketConfig::default(), tls_websocket, secure_websocket_url, DeflateExtProvider::default(), SubprotocolRegistry::default()).await
            },
            Err(e) => {
                debug!("TLS Connection failed with Error {}, falling back to TCP Connection", e);
                let tcp_stream = Self::tcp_connect(url).await?;
                let insecure_websocket_url = format!("ws://{url}");
                let tcp_websocket: Box<dyn WebSocketStream> = Box::new(tcp_stream);
                subscribe_with(WebSocketConfig::default(), tcp_websocket, insecure_websocket_url, DeflateExtProvider::default(), SubprotocolRegistry::default()).await
            }
        };

        match websocket_stream {
            Ok(websocket_stream) => {
                Ok(APClient{ websocket: websocket_stream.into_websocket() })
            },
            Err(e) => {
                Err(APError::WebsocketConnectionFailure)
            }
        }
    }

    async fn tcp_connect(url: &str) -> Result<TcpStream, APError> {
        match TcpStream::connect(url).await {
            Ok(tcp_stream) => Ok(tcp_stream),
            Err(e) => {
                debug!("Failed to connect to {} with error {}", url, e);
                Err(APError::ServerConnectionFailure)
            }
        }
    }

    pub async fn read(&mut self) -> Result<ServerPayload, APError> {
        let mut buf = BytesMut::new();
        match self.websocket.read(&mut buf).await {
            Ok(message) => {
                match message {
                    Message::Text => {
                        match str::from_utf8(&buf) {
                            Ok(payload) => {
                                match serde_json::from_str::<Vec<ServerPayload>>(payload) {
                                    Ok(response) => {
                                        Ok(response.first().unwrap().clone())
                                    },
                                    Err(e) => {
                                        debug!("Parse Error on Payload {} with error {}", payload, e);
                                        Err(APError::ResponseParseFailure)
                                    }
                                }
                            },
                            Err(e) => {
                                Err(APError::ResponseFormatFailure)
                            }
                        }
                    },
                    Message::Binary => {
                        Err(APError::BinaryData)
                    },
                    Message::Close(_) => {
                        Err(APError::NoConnection)
                    },
                    _ => {
                        Err(APError::PingPong)
                    }
                }
            },
            Err(_) => {
                Err(APError::PayloadReadFailure)
            }
        }
    }

    async fn write(&mut self, payload: Result<String, serde_json::Error>) -> Result<(), APError> {
        match payload {
            Ok(serialized_payload) => {
                match self.websocket.write(serialized_payload, ratchet_rs::PayloadType::Text).await {
                    Ok(result) => {
                        Ok(result)
                    },
                    Err(e) => {
                        Err(APError::PayloadWriteFailure)
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
        self.write(connect_payload).await
    }

    pub async fn connect_update(&mut self, items_handling: ItemHandling, tags: Vec<String>) -> Result<(), APError> {
        let connect_update = ConnectUpdate {
            items_handling,
            tags
        };

        let connect_update_payload = serde_json::to_string(&[connect_update]);
        self.write(connect_update_payload).await
    }

    pub async fn sync(&mut self) -> Result<(), APError> {
        let sync = Sync{};

        let sync_payload = serde_json::to_string(&[sync]);
        self.write(sync_payload).await
    }

    pub async fn location_checks(&mut self, locations: Vec<i64>) -> Result<(), APError> {
        let location_checks = LocationChecks {
            locations
        };

        let location_checks_payload = serde_json::to_string(&[location_checks]);
        self.write(location_checks_payload).await
    }

    pub async fn location_scouts(&mut self, locations: Vec<i64>, create_as_hint: i64) -> Result<(), APError> {
        let location_scouts = LocationScouts {
            locations,
            create_as_hint
        };

        let location_scouts_payload = serde_json::to_string(&[location_scouts]);
        self.write(location_scouts_payload).await
    }

    pub async fn create_hints(&mut self, locations: Vec<i64>, player: i64, status: HintStatus) -> Result<(), APError> {
        let create_hints = CreateHints {
            locations,
            player,
            status
        };

        let create_hints_payload = serde_json::to_string(&[create_hints]);
        self.write(create_hints_payload).await
    }

    pub async fn update_hint(&mut self, player: i64, location: i64, status: HintStatus) -> Result<(), APError> {
        let update_hint = UpdateHint {
            player,
            location,
            status
        };

        let update_hint_payload = serde_json::to_string(&[update_hint]);
        self.write(update_hint_payload).await
    }

    pub async fn status_update(&mut self, status: ClientStatus) -> Result<(), APError> {
        let status_update = StatusUpdate {
            status
        };

        let status_update_payload = serde_json::to_string(&[status_update]);
        self.write(status_update_payload).await
    }

    pub async fn say(&mut self, text: String) -> Result<(), APError> {
        let say = Say {
            text
        };

        let say_payload = serde_json::to_string(&[say]);
        self.write(say_payload).await
    }

    pub async fn get_data_package(&mut self, games: Vec<String>) -> Result<(), APError> {
        let get_data_package = GetDataPackage {
            games
        };

        let get_data_package_payload = serde_json::to_string(&[get_data_package]);
        self.write(get_data_package_payload).await
    }

    pub async fn bounce(&mut self, games: Vec<String>, slots: Vec<i64>, tags: Vec<String>, data: HashMap<String, String>) -> Result<(), APError> {
        let bounce = Bounce {
            games,
            slots,
            tags,
            data
        };

        let bounce_payload = serde_json::to_string(&[bounce]);
        self.write(bounce_payload).await
    }

    pub async fn get(&mut self, keys: Vec<String>) -> Result<(), APError> {
        let get = Get {
            keys
        };

        let get_payload = serde_json::to_string(&[get]);
        self.write(get_payload).await
    }

    pub async fn set(&mut self, key: String, default: String, want_reply: bool, operations: Vec<DataStorageOperation>) -> Result<(), APError> {
        let set = Set {
            key,
            default,
            want_reply,
            operations
        };

        let set_payload = serde_json::to_string(&[set]);
        self.write(set_payload).await
    }

    pub async fn set_notify(&mut self, keys: Vec<String>) -> Result<(), APError> {
        let set_notify = SetNotify {
            keys
        };

        let set_notify_payload = serde_json::to_string(&[set_notify]);
        self.write(set_notify_payload).await
    }
}
