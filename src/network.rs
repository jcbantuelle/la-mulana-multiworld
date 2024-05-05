use std::sync::Mutex;
use std::net::TcpStream;

use log::{debug, warn, error};
use serde::{Deserialize, Serialize};
use tungstenite::{stream::MaybeTlsStream, WebSocket, connect, Message};
use archipelago_rs::client::{ ArchipelagoClient, ArchipelagoError };
use archipelago_rs::protocol::{ClientMessage, ClientStatus, ServerMessage};
use std::io::{self, BufRead};

static CHANNEL_NAME: &str = "MultiworldSyncChannel";
static GAME_NAME: &str = "La-Mulana";
static CLIENT_NAME: &str = "la_mulana_rs-0.0.1";

pub fn serialize_message<T: Serialize>(message: T) -> String {
    serde_json::to_string(&message).unwrap()
}

pub trait Randomizer {
    fn read_messages(&self) -> Result<Option<ServerMessage>, ArchipelagoError>;
    fn send_message(&self, message: ClientMessage);
}

pub struct LiveRandomizer {
    pub runtime: tokio::runtime::Runtime,
    pub client: Mutex<ArchipelagoClient>
}

pub struct ReceiveMessageError {
    pub message: String
}

impl LiveRandomizer {
    pub fn new(server_url: &str, slot: &str) -> LiveRandomizer {

        // Connect to AP server
        let server = server_url;

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let res = rt.block_on(async { ArchipelagoClient::new(&server).await });
        let mut client = res.unwrap();
        println!("Connected!");

        // Connect to a given slot on the server

        rt.block_on(async {
            client
                .connect(GAME_NAME, &slot, None, Some(7), vec![CLIENT_NAME.to_string()])
                .await
        });

        LiveRandomizer {
            runtime: rt,
            client: Mutex::new(client)
        }
    }

    pub fn send_message(&self, message: ClientMessage) {
        self.client.lock().unwrap().send(message);
    }

    pub fn read_messages(&self) -> Result<Option<ServerMessage>, ArchipelagoError> {
        self.runtime.block_on( async { self.client.lock().unwrap().recv().await })
    }
}

pub trait NetworkReader {
    fn read(&self) -> Result<(), NetworkReaderError>;
}

pub struct NetworkReaderError {
    pub message: String
}

#[derive(Serialize, Deserialize)]
pub struct SubscribePayload {
    command: String,
    identifier: String
}

#[derive(Serialize, Deserialize)]
pub struct SendPayload {
    command: String,
    identifier: String,
    data: String
}

#[derive(Serialize, Deserialize)]
pub struct Identifier {
    pub id: i32,
    pub channel: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReceivePayload {
    pub identifier: String,
    pub message: ReceiveMessage
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReceiveMessage {
    pub items: Vec<ReceiveItem>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReceiveItem {
    pub item_id: u8,
    pub player_id: u8,
}

#[derive(Serialize, Deserialize)]
pub struct RandomizerMessage {
    pub player_id: i32,
    pub global_flags: Vec<u8>
}
