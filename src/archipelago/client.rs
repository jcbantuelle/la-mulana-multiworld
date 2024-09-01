use thiserror::Error;
use log::{error, debug};
use websocket::message::Type;
use websocket::{ClientBuilder, WebSocketError, OwnedMessage};
use websocket::sync::Client;
use websocket::sync::stream::NetworkStream;
use crate::archipelago::protocol::*;

use crate::AppConfig;

#[derive(Debug)]
pub struct NonTextMessage {
    /// Type of WebSocket message
    pub opcode: Type,
    /// Optional status code to send when closing a connection.
    /// (only used if this message is of Type::Close)
    pub cd_status_code: Option<u16>,
    /// Main payload
    pub payload: Vec<u8>
}

#[derive(Error, Debug)]
pub enum ArchipelagoError {
    #[error("illegal response")]
    IllegalResponse {
        received: ServerMessage,
        expected: &'static str,
    },
    #[error("connection closed by server")]
    ConnectionClosed,
    #[error("data failed to serialize")]
    FailedSerialize(#[from] serde_json::Error),
    #[error("unexpected non-text result from websocket")]
    NonTextWebsocketResult(NonTextMessage),
    #[error("network error")]
    NetworkError(#[from] WebSocketError),
}

pub struct ArchipelagoClient {
    ws: Client<Box<dyn NetworkStream + Send>>,
    room_info: RoomInfo,
    data_package: Option<DataPackageObject>,
}

impl ArchipelagoClient {
    pub fn new(app_config: AppConfig) -> Result<ArchipelagoClient, ArchipelagoError> {
        let url = &app_config.server_url;
        let mut wss_url = String::new();
        wss_url.push_str("wss://");
        wss_url.push_str(url);

        let mut ws = match ClientBuilder::new(&wss_url).unwrap().connect(None) {
            Ok(result) => result,
            Err(error) => {
                let mut ws_url = String::new();
                ws_url.push_str("ws://");
                ws_url.push_str(url);

                match ClientBuilder::new(&ws_url).unwrap().connect(None) {
                    Ok(result) => result,
                    Err(error) => {
                        return Err(ArchipelagoError::NetworkError(error))        
                    }
                }
            },
        };

        let server_response = match ws.recv_message().unwrap() {
            websocket::OwnedMessage::Text(room_info) => room_info,
            _ => {
                return Err(ArchipelagoError::ConnectionClosed)
            }
        };

        let mut iter = serde_json::from_str::<Vec<ServerMessage>>(&server_response).unwrap().into_iter();
        let room_info = match iter.next() {
            Some(ServerMessage::RoomInfo(room)) => room,
            Some(received) =>
                return Err(ArchipelagoError::IllegalResponse {
                    received,
                    expected: "Expected RoomInfo",
                }),
            None => return Err(ArchipelagoError::ConnectionClosed)
        };

        Ok(ArchipelagoClient {
            ws,
            room_info,
            data_package: None,
        })
    }

    pub fn read_messages(&mut self) {
        match self.ws.recv_message().unwrap() {
            OwnedMessage::Ping(ping) => {
                let pong = OwnedMessage::Pong(ping);
                self.ws.send_message(&pong);
            },
            _ => ()
        }
    }
}
