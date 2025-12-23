use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Clone, Error, Debug, Serialize, Deserialize)]
pub enum APError {
    #[error("not connected")]
    NoConnection,
    #[error("tls connector failed to build")]
    TlsConnectorFailure,
    #[error("unable to connect to server")]
    ServerConnectionFailure,
    #[error("unable to establish websocket connection")]
    WebsocketConnectionFailure,
    #[error("unable to serialize payload")]
    PayloadSerializationFailure,
    #[error("failed to write payload")]
    PayloadWriteFailure,
    #[error("failed to read payload")]
    PayloadReadFailure,
    #[error("binary data from server")]
    BinaryData,
    #[error("ping/pong from server")]
    PingPong,
    #[error("failed to parse response from server")]
    ResponseParseFailure,
    #[error("unable to convert response to string")]
    ResponseFormatFailure
}

// Client -> Server Payloads

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ClientPayload {
    Connect(Connect),
    ConnectUpdate(ConnectUpdate),
    Sync(Sync),
    LocationChecks(LocationChecks),
    LocationScouts(LocationScouts),
    CreateHints(CreateHints),
    UpdateHint(UpdateHint),
    StatusUpdate(StatusUpdate),
    Say(Say),
    GetDataPackage(GetDataPackage),
    Bounce(Bounce),
    Get(Get),
    Set(Set),
    SetNotify(SetNotify)
}

#[derive(Clone, Serialize, Deserialize, Debug)]
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

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "cmd", rename = "ConnectUpdate")]
pub struct ConnectUpdate {
    pub items_handling: ItemHandling,
    pub tags: Vec<String>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "cmd", rename = "Sync")]
pub struct Sync {
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "cmd", rename = "LocationChecks")]
pub struct LocationChecks {
    pub locations: Vec<i64>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "cmd", rename = "LocationScouts")]
pub struct LocationScouts {
    pub locations: Vec<i64>,
    pub create_as_hint: i64
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "cmd", rename = "CreateHints")]
pub struct CreateHints {
    pub locations: Vec<i64>,
    pub player: i64,
    pub status: HintStatus
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "cmd", rename = "UpdateHint")]
pub struct UpdateHint {
    pub player: i64,
    pub location: i64,
    pub status: HintStatus
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "cmd", rename = "StatusUpdate")]
pub struct StatusUpdate {
    pub status: ClientStatus
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "cmd", rename = "Say")]
pub struct Say {
    pub text: String
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "cmd", rename = "GetDataPackage")]
pub struct GetDataPackage {
    pub games: Vec<String>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "cmd", rename = "Bounce")]
pub struct Bounce {
    pub games: Vec<String>,
    pub slots: Vec<i64>,
    pub tags: Vec<String>,
    pub data: HashMap<String, String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "cmd", rename = "Get")]
pub struct Get {
    pub keys: Vec<String>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "cmd", rename = "Set")]
pub struct Set {
    pub key: String,
    pub default: String,
    pub want_reply: bool,
    pub operations: Vec<DataStorageOperation>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "cmd", rename = "SetNotify")]
pub struct SetNotify {
    pub keys: Vec<String>
}

// Server -> Client Payloads

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "cmd")]
pub enum ServerPayload {
    RoomInfo(RoomInfo),
    ConnectionRefused(ConnectionRefused),
    Connected(Connected),
    ReceivedItems(ReceivedItems),
    LocationInfo(LocationInfo),
    RoomUpdate(RoomUpdate),
    PrintJSON(PrintJSON),
    DataPackage(DataPackage),
    Bounced(Bounced),
    InvalidPacket(InvalidPacket),
    Retrieved(Retrieved),
    SetReply(SetReply)
}

#[derive(Clone, Serialize, Deserialize, Debug)]
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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConnectionRefused {
    pub errors: Vec<String>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Connected {
    pub team: i64,
    pub slot: i64,
    pub players: Vec<NetworkPlayer>,
    pub missing_locations: Vec<i64>,
    pub checked_locations: Vec<i64>,
    pub slot_data: Option<HashMap<String, String>>,
    pub slot_info: HashMap<String, NetworkSlot>,
    pub hint_points: i64
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ReceivedItems {
    pub index: u16,
    pub items: Vec<NetworkItem>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LocationInfo {
    pub location: Vec<NetworkItem>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RoomUpdate {
    pub version: Option<NetworkVersion>,
    pub generator_version: Option<NetworkVersion>,
    pub tags: Option<Vec<String>>,
    pub password: Option<bool>,
    pub permissions: Option<HashMap<String, Permission>>,
    pub hint_cost: Option<i64>,
    pub location_check_points: Option<i64>,
    pub games: Option<Vec<String>>,
    pub datapackage_checksums: Option<HashMap<String, String>>,
    pub seed_name: Option<String>,
    pub time: Option<f64>,
    pub team: Option<i64>,
    pub slot: Option<i64>,
    pub players: Option<Vec<NetworkPlayer>>,
    pub missing_locations: Option<Vec<i64>>,
    pub checked_locations: Option<Vec<i64>>,
    pub slot_data: Option<HashMap<String, String>>,
    pub slot_info: Option<HashMap<String, NetworkSlot>>,
    pub hint_points: Option<i64>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PrintJSON {
    pub data: Vec<JSONMessagePart>,
    pub r#type: String,
    pub receiving: Option<i64>,
    pub item: Option<NetworkItem>,
    pub found: Option<bool>,
    pub team: Option<i64>,
    pub slot: Option<i64>,
    pub message: Option<String>,
    pub tags: Option<Vec<String>>,
    pub countdown: Option<i64>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DataPackage {
    pub data: DataPackageObject
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Bounced {
    pub games: Vec<String>,
    pub slots: Vec<i64>,
    pub tags: Vec<String>,
    pub data: HashMap<String, String>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InvalidPacket {
    pub r#type: String,
    pub original_cmd: String,
    pub text: String
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Retrieved {
    pub keys: HashMap<String, String>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SetReply {
    pub key: String,
    pub value: String,
    pub original_value: String,
    pub slot: i64
}

// AP-Defined Structs/Enums

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NetworkVersion {
    pub class: String,
    pub build: i64,
    pub major: i64,
    pub minor: i64
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NetworkPlayer {
    pub team: i64,
    pub slot: i64,
    pub alias: String,
    pub name: String
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NetworkSlot {
    pub name: String,
    pub game: String,
    pub r#type: SlotType,
    pub group_members: Vec<i64>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NetworkItem {
    pub item: i64,
    pub location: i64,
    pub player: i64,
    pub flags: i64
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct JSONMessagePart {
    pub r#type: Option<String>,
    pub text: Option<String>,
    pub color: Option<String>,
    pub flags: Option<i64>,
    pub player: Option<i64>,
    pub hint_status: Option<HintStatus>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DataPackageObject {
    pub games: HashMap<String, GameData>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameData {
    pub item_name_to_id: HashMap<String, i64>,
    pub location_name_to_id: HashMap<String, i64>,
    pub checksum: String
}

#[derive(Clone, Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum ItemHandling {
    NeverReceiveItems = 0,
    OtherWorldsOnly = 1,
    OtherWorldsAndSelf = 3,
    OtherWorldsAndStartingInventory = 5,
    OtherWorldsSelfAndStartingInventory = 6
}

#[derive(Clone, Serialize_repr, Deserialize_repr, Debug)]
#[repr(u16)]
pub enum HintStatus {
    HintUnspecified = 0,
    HintNoPriority = 10,
    HintAvoid = 20,
    HintPriority = 30,
    HintFound = 40
}

#[derive(Clone, Serialize_repr, Deserialize_repr, Debug)]
#[repr(u16)]
pub enum ClientStatus {
    ClientUnknown = 0,
    ClientConnected = 5,
    ClientReady = 10,
    ClientPlaying = 20,
    ClientGoal = 30
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DataStorageOperation {
    pub operation: Operation,
    pub value: String
}

#[derive(Clone, Serialize, Deserialize, Debug)]
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

#[derive(Clone, Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum Permission {
    Disabled = 0, // completely disables access
    Enabled = 1, // allows manual use
    Goal = 2, // allows manual use after goal completion
    Auto = 6, // forces use after goal completion, only works for release and collect
    AutoEnabled = 7 // forces use after goal completion, allows manual use any time
}

#[derive(Clone, Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum SlotType {
    Spectator = 0,
    Player = 1,
    Group = 2
}
