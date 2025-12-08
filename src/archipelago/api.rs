use serde::{Serialize, Deserialize};
use std::collections::HashMap;
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

// Client -> Server Payloads

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "Connect")]
pub struct Connect {
    pub password: String,
    pub game: String,
    pub name: String,
    pub uuid: i64,
    pub version: NetworkVersion,
    pub items_handling: ItemHandling,
    pub tags: Vec<String>,
    pub slot_data: bool
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "ConnectUpdate")]
pub struct ConnectUpdate {
    pub items_handling: ItemHandling,
    pub tags: Vec<String>
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "Sync")]
pub struct Sync {
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "LocationChecks")]
pub struct LocationChecks {
    pub locations: Vec<i64>
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "LocationScouts")]
pub struct LocationScouts {
    pub locations: Vec<i64>,
    pub create_as_hint: i64
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "CreateHints")]
pub struct CreateHints {
    pub locations: Vec<i64>,
    pub player: i64,
    pub status: HintStatus
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "UpdateHint")]
pub struct UpdateHint {
    pub player: i64,
    pub location: i64,
    pub status: HintStatus
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "StatusUpdate")]
pub struct StatusUpdate {
    pub status: ClientStatus
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "Say")]
pub struct Say {
    pub text: String
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "GetDataPackage")]
pub struct GetDataPackage {
    pub games: Vec<String>
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "Bounce")]
pub struct Bounce {
    pub games: Vec<String>,
    pub slots: Vec<i64>,
    pub tags: Vec<String>,
    pub data: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "Get")]
pub struct Get {
    pub keys: Vec<String>
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "Set")]
pub struct Set<T> {
    pub key: String,
    pub default: T,
    pub want_reply: bool,
    pub operations: Vec<DataStorageOperation<T>>
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "SetNotify")]
pub struct SetNotify {
    pub keys: Vec<String>
}

// AP-Defined Structs

#[derive(Serialize, Deserialize)]
pub struct NetworkVersion {
    pub class: String,
    pub build: i64,
    pub major: i64,
    pub minor: i64
}

#[derive(Serialize, Deserialize)]
pub enum ItemHandling {
    NeverReceiveItems = 0,
    OtherWorldsOnly = 1,
    OtherWorldsAndSelf = 3,
    OtherWorldsAndStartingInventory = 5,
    OtherWorldsSelfAndStartingInventory = 6
}

#[derive(Serialize, Deserialize)]
pub enum HintStatus {
    HintUnspecified = 0,
    HintNoPriority = 10,
    HintAvoid = 20,
    HintPriority = 30,
    HintFound = 40
}

#[derive(Serialize, Deserialize)]
pub enum ClientStatus {
    ClientUnknown = 0,
    ClientConnected = 5,
    ClientReady = 10,
    ClientPlaying = 20,
    ClientGoal = 30
}

#[derive(Serialize, Deserialize)]
pub struct DataStorageOperation<T> {
    pub operation: Operation,
    pub value: T
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
    Replace,
    Default,
    Add,
    Mul,
    Pow,
    Mod,
    Floor,
    Ceil,
    Max,
    Min,
    And,
    Or,
    Xor,
    LeftShift,
    RightShift,
    Remove,
    Pop,
    Update
}
