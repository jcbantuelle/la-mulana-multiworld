// Copyright Ryan Goldstein

// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "cmd")]
pub enum ClientMessage {
    Connect(Connect),
    Sync,
    LocationChecks(LocationChecks),
    LocationScouts(LocationScouts),
    StatusUpdate(StatusUpdate),
    Say(Say),
    GetDataPackage(GetDataPackage),
    Bounce(Bounce),
    Get(Get),
    Set(Set),
    SetNotify(SetNotify),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "cmd")]
pub enum ServerMessage {
    RoomInfo(RoomInfo),
    ConnectionRefused(ConnectionRefused),
    Connected(Connected),
    ReceivedItems(ReceivedItems),
    LocationInfo(LocationInfo),
    RoomUpdate(RoomUpdate),
    Print(Print),
    PrintJSON(PrintJSON),
    DataPackage(DataPackage),
    Bounced(Bounced),
    InvalidPacket(InvalidPacket),
    Retrieved(Retrieved),
    SetReply(SetReply),
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone)]
#[repr(u16)]
pub enum Permission {
    Disabled = 0,
    Enabled = 1,
    Goal = 2,
    Auto = 6,
    AutoEnabled = 7,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkVersion {
    pub major: i32,
    pub minor: i32,
    pub build: i32,
    pub class: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkPlayer {
    pub team: i32,
    pub slot: i32,
    pub alias: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkItem {
    pub item: i32,
    pub location: i32,
    pub player: i32,
    pub flags: i32,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone)]
#[repr(u16)]
pub enum SlotType {
    Spectator = 0,
    Player = 1,
    Group = 2,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkSlot {
    pub name: String,
    pub game: String,
    pub r#type: SlotType,
    pub group_members: Vec<i32>,
}

pub fn network_version() -> NetworkVersion {
    NetworkVersion {
        major: 0,
        minor: 5,
        build: 1,
        class: "Version".to_string(),
    }
}

// REQUESTS

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Connect {
    pub password: Option<String>,
    pub name: String,
    pub version: NetworkVersion,
    pub items_handling: Option<i32>,
    pub tags: Vec<String>,
    pub uuid: String,
    pub game: String,
    pub slot_data: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectUpdate {
    pub items_handling: i32,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocationChecks {
    pub locations: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocationScouts {
    pub locations: Vec<i32>,
    pub create_as_hint: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatusUpdate {
    pub status: ClientStatus,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone)]
#[repr(u16)]
pub enum ClientStatus {
    ClientUnknown = 0,
    ClientReady = 10,
    ClientPlaying = 20,
    ClientGoal = 30,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Say {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetDataPackage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub games: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bounce {
    pub games: Option<Vec<String>>,
    pub slots: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub data: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Get {
    pub keys: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Set {
    pub key: String,
    pub default: Value,
    pub want_reply: bool,
    pub operations: Vec<DataStorageOperation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataStorageOperation {
    pub replace: String, // TODO: enum-ify?
    pub value: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetNotify {
    pub keys: Vec<String>,
}

// RESPONSES

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoomInfo {
    pub version: NetworkVersion,
    pub generator_version: NetworkVersion,
    pub tags: Vec<String>,
    pub password: bool,
    pub permissions: HashMap<String, Permission>,
    pub hint_cost: i32,
    pub location_check_points: i32,
    pub games: Vec<String>,
    pub datapackage_checksums: HashMap<String, String>,
    pub seed_name: String,
    pub time: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectionRefused {
    pub errors: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Connected {
    pub team: i32,
    pub slot: i32,
    pub players: Vec<NetworkPlayer>,
    pub missing_locations: Vec<i32>,
    pub checked_locations: Vec<i32>,
    pub slot_data: Value,
    pub slot_info: HashMap<String, NetworkSlot>, // TODO: docs claim this is an int key. they are lying?
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReceivedItems {
    pub index: i32,
    pub items: Vec<NetworkItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocationInfo {
    pub locations: Vec<NetworkItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoomUpdate {
    // Copied from RoomInfo
    pub version: Option<NetworkVersion>,
    pub tags: Option<Vec<String>>,
    pub password: Option<bool>,
    pub permissions: Option<HashMap<String, Permission>>,
    pub hint_cost: Option<i32>,
    pub location_check_points: Option<i32>,
    pub games: Option<Vec<String>>,
    pub datapackage_versions: Option<HashMap<String, i32>>,
    pub seed_name: Option<String>,
    pub time: Option<f32>,
    // Exclusive to RoomUpdate
    pub hint_points: Option<i32>,
    pub players: Option<Vec<NetworkPlayer>>,
    pub checked_locations: Option<Vec<i32>>,
    pub missing_locations: Option<Vec<i32>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Print {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrintJSON {
    pub data: Vec<JSONMessagePart>,
    pub r#type: Option<String>,
    pub receiving: Option<i32>,
    pub item: Option<NetworkItem>,
    pub found: Option<bool>,
    pub countdown: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JSONMessagePart {
    pub r#type: Option<String>,
    pub text: Option<String>,
    pub color: Option<String>,
    pub flags: Option<i32>,
    pub player: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataPackage {
    pub data: DataPackageObject,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataPackageObject {
    pub games: HashMap<String, GameData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameData {
    pub item_name_to_id: HashMap<String, i32>,
    pub location_name_to_id: HashMap<String, i32>,
    pub version: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bounced {
    pub games: Vec<String>,
    pub slots: Vec<i32>,
    pub tags: Vec<String>,
    pub data: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InvalidPacket {
    pub r#type: String,
    pub original_cmd: Option<String>,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Retrieved {
    keys: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetReply {
    key: String,
    value: Value,
    original_value: Value,
}
