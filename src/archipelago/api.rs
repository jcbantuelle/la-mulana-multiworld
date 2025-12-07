use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum APError {
    #[error("not connected")]
    NoConnection,
    #[error("unable to connect to server")]
    ServerConnectionFailure,
    #[error("unable to establish websocket connection")]
    WebsocketConnectionFailure,
    #[error("unable to serialize payload")]
    PayloadSerializationFailure,
    #[error("failed to send paylod")]
    PayloadSendFailure
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "Connect")]
pub struct Connect {
    pub password: String,
    pub game: String,
    pub name: String,
    pub uuid: i64,
    pub version: NetworkVersion,
    pub items_handling: u8,
    pub tags: Vec<String>,
    pub slot_data: bool
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "Sync")]
pub struct Sync {
}

#[derive(Serialize, Deserialize)]
pub struct NetworkVersion {
    pub class: String,
    pub build: i64,
    pub major: i64,
    pub minor: i64
}
