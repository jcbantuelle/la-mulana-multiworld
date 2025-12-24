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

pub struct APConnectionDetails {
    protocol: String,
    stream: Box<dyn WebSocketStream>
}

impl APClient {
    pub async fn new(url: &str) -> Result<APClient, APError> {
        let tcp_stream_for_tls = Self::tcp_connect(url).await?;
        let tls_builder = TlsConnector::builder().build().map_err(|_| { APError::TlsConnectorFailure })?;
        let tls_connector = tokio_native_tls::TlsConnector::from(tls_builder);

        let domain = match url.find(":") {
            None => url,
            Some(port_index) => {
                &url[..port_index]
            }
        };
        
        let connection_details = match tls_connector.connect(domain, tcp_stream_for_tls).await {
            Ok(tls_stream) => APConnectionDetails{protocol: "wss".to_string(), stream: Box::new(tls_stream)},
            Err(e) => {
                debug!("TLS Connection failed with Error {}, falling back to TCP Connection", e);
                let tcp_stream = Self::tcp_connect(url).await?;
                APConnectionDetails{protocol: "ws".to_string(), stream: Box::new(tcp_stream)}
            }
        };

        let url_with_protocol = format!("{}://{}", connection_details.protocol, url);
        let websocket_stream = subscribe_with(WebSocketConfig::default(), connection_details.stream, url_with_protocol, DeflateExtProvider::default(), SubprotocolRegistry::default()).await;

        match websocket_stream {
            Ok(websocket_stream) => {
                Ok(APClient{ websocket: websocket_stream.into_websocket() })
            },
            Err(e) => {
                debug!("Websocket Connection Failed: {}", e);
                Err(APError::WebsocketConnectionFailure)
            }
        }
    }

    async fn tcp_connect(url: &str) -> Result<TcpStream, APError> {
        TcpStream::connect(url).await.map_err(|e| {
            debug!("Failed to connect to {}: {}", url, e);
            APError::ServerConnectionFailure
        })
    }

    pub async fn read(&mut self) -> Result<ServerPayload, APError> {
        let mut buf = BytesMut::new();
        let message = self.websocket.read(&mut buf).await.map_err(|_| { APError::PayloadReadFailure })?;
        match message {
            Message::Text => {
                let payload = str::from_utf8(&buf).map_err(|e| {
                    debug!("Unable to Convert Payload to String: {}", e);
                    APError::ResponseFormatFailure
                })?;

                let response = serde_json::from_str::<Vec<ServerPayload>>(payload).map_err(|e| {
                    debug!("Parse Error on Payload {}: {}", payload, e);
                    APError::ResponseParseFailure
                })?;

                Ok(response.first().unwrap().clone())
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
    }

    async fn write(&mut self, payload: ClientPayload) -> Result<(), APError> {
        let serialized_payload = serde_json::to_string(&[payload]).map_err(|_| { APError::PayloadSerializationFailure })?;
        debug!("Sending Message To Server: {}", serialized_payload);
        let response= self.websocket.write(serialized_payload, ratchet_rs::PayloadType::Text).await;
        response.map_err(|e| {
            debug!("Failed to Write Payload to Server: {}", e);
            APError::PayloadWriteFailure
        })
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

        self.write(ClientPayload::Connect(connect)).await
    }

    pub async fn connect_update(&mut self, items_handling: ItemHandling, tags: Vec<String>) -> Result<(), APError> {
        let connect_update = ConnectUpdate {
            items_handling,
            tags
        };

        self.write(ClientPayload::ConnectUpdate(connect_update)).await
    }

    pub async fn sync(&mut self) -> Result<(), APError> {
        let sync = Sync{};

        self.write(ClientPayload::Sync(sync)).await
    }

    pub async fn location_checks(&mut self, locations: Vec<i64>) -> Result<(), APError> {
        let location_checks = LocationChecks {
            locations
        };

        self.write(ClientPayload::LocationChecks(location_checks)).await
    }

    pub async fn location_scouts(&mut self, locations: Vec<i64>, create_as_hint: i64) -> Result<(), APError> {
        let location_scouts = LocationScouts {
            locations,
            create_as_hint
        };

        self.write(ClientPayload::LocationScouts(location_scouts)).await
    }

    pub async fn create_hints(&mut self, locations: Vec<i64>, player: i64, status: HintStatus) -> Result<(), APError> {
        let create_hints = CreateHints {
            locations,
            player,
            status
        };

        self.write(ClientPayload::CreateHints(create_hints)).await
    }

    pub async fn update_hint(&mut self, player: i64, location: i64, status: HintStatus) -> Result<(), APError> {
        let update_hint = UpdateHint {
            player,
            location,
            status
        };

        self.write(ClientPayload::UpdateHint(update_hint)).await
    }

    pub async fn status_update(&mut self, status: ClientStatus) -> Result<(), APError> {
        let status_update = StatusUpdate {
            status
        };

        self.write(ClientPayload::StatusUpdate(status_update)).await
    }

    pub async fn say(&mut self, text: String) -> Result<(), APError> {
        let say = Say {
            text
        };

        self.write(ClientPayload::Say(say)).await
    }

    pub async fn get_data_package(&mut self, games: Vec<String>) -> Result<(), APError> {
        let get_data_package = GetDataPackage {
            games
        };

        self.write(ClientPayload::GetDataPackage(get_data_package)).await
    }

    pub async fn bounce(&mut self, games: Vec<String>, slots: Vec<i64>, tags: Vec<String>, data: HashMap<String, String>) -> Result<(), APError> {
        let bounce = Bounce {
            games,
            slots,
            tags,
            data
        };

        self.write(ClientPayload::Bounce(bounce)).await
    }

    pub async fn get(&mut self, keys: Vec<String>) -> Result<(), APError> {
        let get = Get {
            keys
        };

        self.write(ClientPayload::Get(get)).await
    }

    pub async fn set(&mut self, key: String, default: String, want_reply: bool, operations: Vec<DataStorageOperation>) -> Result<(), APError> {
        let set = Set {
            key,
            default,
            want_reply,
            operations
        };

        self.write(ClientPayload::Set(set)).await
    }

    pub async fn set_notify(&mut self, keys: Vec<String>) -> Result<(), APError> {
        let set_notify = SetNotify {
            keys
        };

        self.write(ClientPayload::SetNotify(set_notify)).await
    }
}
