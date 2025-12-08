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

// Server -> Client Payloads

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "RoomInfo")]
pub struct RoomInfo {
    pub version: NetworkVersion,
    pub generator_version: NetworkVersion,
    pub tags: Vec<String>,
    pub password: bool,
    pub permissions: HashMap<String, Permission>,
    pub hint_cost: i64,
    pub location_check_points: i64,
    pub games: Vec<String>,
    pub datapackage_checksums: HashMap<String, String>,
    pub seed_name: String,
    pub time: f64
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "ConnectionRefused")]
pub struct ConnectionRefused {
    pub errors: Vec<String>
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "Connected")]
pub struct Connected<T> {
    pub team: i64,
    pub slot: i64,
    pub players: Vec<NetworkPlayer>,
    pub missing_locations: Vec<i64>,
    pub checked_locations: Vec<i64>,
    pub slot_data: HashMap<String, T>,
    pub slot_info: HashMap<i64, NetworkSlot>,
    pub hint_points: i64
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "ReceivedItems")]
pub struct ReceivedItems {
    pub index: i64,
    pub items: Vec<NetworkItem>
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "LocationInfo")]
pub struct LocationInfo {
    pub location: Vec<NetworkItem>
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "RoomUpdate")]
pub struct RoomUpdate {
    pub players: Vec<NetworkPlayer>,
    pub checked_locations: Vec<i64>,
    pub missing_locations: Vec<i64>
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "PrintJSON")]
pub struct PrintJSON {
    pub data: Vec<JSONMessagePart>,
    pub r#type: String,
    pub receiving: i64,
    pub item: NetworkItem,
    pub found: bool,
    pub team: i64,
    pub slot: i64,
    pub message: String,
    pub tags: Vec<String>,
    pub countdown: i64
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "DataPackage")]
pub struct DataPackage {
    pub data: DataPackageObject
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "Bounced")]
pub struct Bounced {
    pub games: Vec<String>,
    pub slots: Vec<i64>,
    pub tags: Vec<String>,
    pub data: HashMap<String, String>
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "InvalidPacket")]
pub struct InvalidPacket {
    pub r#type: String,
    pub original_cmd: String,
    pub text: String
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "Retrieved")]
pub struct Retrieved<T> {
    pub keys: HashMap<String, T>
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename = "SetReply")]
pub struct SetReply<T> {
    pub key: String,
    pub value: T,
    pub original_value: T,
    pub slot: i64
}

// AP-Defined Structs/Enums

#[derive(Serialize, Deserialize)]
pub struct NetworkVersion {
    pub class: String,
    pub build: i64,
    pub major: i64,
    pub minor: i64
}

#[derive(Serialize, Deserialize)]
pub struct NetworkPlayer {
    pub team: i64,
    pub slot: i64,
    pub alias: String,
    pub name: String
}

#[derive(Serialize, Deserialize)]
pub struct NetworkSlot {
    pub name: String,
    pub game: String,
    pub r#type: SlotType,
    pub group_members: Vec<i64>
}

#[derive(Serialize, Deserialize)]
pub struct NetworkItem {
    pub item: i64,
    pub location: i64,
    pub player: i64,
    pub flags: i64
}

#[derive(Serialize, Deserialize)]
pub struct JSONMessagePart {
    pub r#type: String,
    pub text: String,
    pub color: String,
    pub flags: i64,
    pub player: i64,
    pub hint_status: HintStatus
}

#[derive(Serialize, Deserialize)]
pub struct DataPackageObject {
    pub games: HashMap<String, GameData>
}

#[derive(Serialize, Deserialize)]
pub struct GameData {
    pub item_name_to_id: HashMap<String, i64>,
    pub location_name_to_id: HashMap<String, i64>,
    pub checksum: String
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

#[derive(Serialize, Deserialize)]
pub enum Permission {
    Disabled = 0, // completely disables access
    Enabled = 1, // allows manual use
    Goal = 2, // allows manual use after goal completion
    Auto = 6, // forces use after goal completion, only works for release and collect
    AutoEnabled = 7 // forces use after goal completion, allows manual use any time
}

#[derive(Serialize, Deserialize)]
pub enum SlotType {
    Spectator = 0,
    Player = 1,
    Group = 2
}
