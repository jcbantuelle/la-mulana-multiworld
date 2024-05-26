use std::future::Future;
use std::sync::Mutex;
use std::net::TcpStream;

use log::{debug, warn, error};
use serde::{Deserialize, Serialize};
use tungstenite::{stream::MaybeTlsStream, WebSocket, connect, Message};
use archipelago_rs::client::{ ArchipelagoClient, ArchipelagoError };
use archipelago_rs::protocol::{ClientMessage, ClientStatus, ServerMessage};
use std::io::{self, BufRead};
use std::pin::Pin;
use tokio_tungstenite::tungstenite::Error;

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
    pub client: Mutex<ArchipelagoClient>,
    pub slot: String
}

pub struct ReceiveMessageError {
    pub message: String
}

impl LiveRandomizer {
    pub fn new(server_url: &str, slot: &str) -> LiveRandomizer {

        // Connect to AP server
        let server = server_url;
        debug!("Connecting to {}...", server);

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        debug!("Created new runtime and about to block on client creation...");
        let res = rt.block_on(async { ArchipelagoClient::new(&server).await });
        let mut client = res.unwrap();

        debug!("Done connecting to server");
        LiveRandomizer {
            runtime: rt,
            client: Mutex::new(client),
            slot: slot.to_string()
        }
    }

    pub fn send_message(&self, message: ClientMessage) {
        let result = self.try_and_connect_on_failure::<(), (), ClientMessage>(
            |m: ClientMessage| {
                Box::pin(
                    async {
                        self.client.lock().unwrap().send(m).await
                    }
                )
            }, message);

        match result {
            Ok(_) => {},
            Err(e) => {
                error!("network.send_message: Could not send message {:?}", e)
            }
        }
    }

    pub fn read_messages(&self) -> Result<Option<ServerMessage>, ArchipelagoError> {
        self.try_and_connect_on_failure::<Option<ServerMessage>, Option<ServerMessage>, ()>(
            |_| {
                Box::pin(
                    async {
                        self.client.lock().unwrap().recv().await
                    }
                )
            }, ()
        )
    }

    pub fn try_and_connect_on_failure<'a, T: Sized, U: Sized, V: Clone>(&self, f: impl Fn(V) -> Pin<Box<dyn Future<Output=Result<T, ArchipelagoError>> + 'a>>, input: V) -> Result<U, ArchipelagoError> {
        let mut count = 0;
        let mut sent = true;

        while count < 3 && !sent {
            let m = self.runtime.block_on(
                async {
                    f(input.clone()).await
                }
            );

            sent = m.is_ok();
            if !sent {
                count = count + 1;
            }
        }

        Err(
            ArchipelagoError::NetworkError(Error::ConnectionClosed)
        )
    }

    fn connect(&mut self) {
        debug!("About to connect to server with GAME_NAME of {} and slot {}...", GAME_NAME, self.slot);
        self.runtime.block_on(async {
            self.client.lock().unwrap()
                .connect(GAME_NAME, &self.slot, None, Some(7), vec![CLIENT_NAME.to_string()])
                .await
        }).expect("Could not connect to server");
    }

    fn clone_message(&self, message: &ClientMessage) -> ClientMessage {
        match message {
           ClientMessage::Connect(connect)  => ClientMessage::Connect(connect.clone()),
            _ => ClientMessage::Sync
        }
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
