use std::sync::Mutex;
use std::net::TcpStream;

use log::{debug, error};
use serde::{Deserialize, Serialize};
use tungstenite::{stream::MaybeTlsStream, WebSocket, connect, Message};

static CHANNEL_NAME: &str = "MultiworldSyncChannel";

pub struct Randomizer {
    pub websocket: Mutex<WebSocket<MaybeTlsStream<TcpStream>>>,
    pub identifier: Identifier
}

pub fn serialize_message<T: Serialize>(message: T) -> String {
    serde_json::to_string(&message).unwrap()
}

impl Randomizer {
    pub fn new(server_url: &str, user_id: i32) -> Randomizer {
        let url = url::Url::parse(server_url).unwrap();
        let (mut ws_connection, _) = connect(url).expect("Failed to connect");
        match ws_connection.get_ref() {
            MaybeTlsStream::NativeTls(ref tls) => {
                tls.get_ref().set_nonblocking(true).map_err(|err| {
                    error!("Could not set socket as nonblocking: {}", err);
                }).unwrap();
            },
            _ => ()
        };

        let identifier = Identifier {
            id: user_id,
            channel: CHANNEL_NAME.to_string()
        };
        let subscribe_payload = SubscribePayload {
            command: "subscribe".to_string(),
            identifier: serde_json::to_string(&identifier).unwrap()
        };

        ws_connection.write_message(Message::Text(serde_json::to_string(&subscribe_payload).unwrap())).expect("Unable to Connect To Websocket Channel");
        Randomizer {
            websocket: Mutex::new(ws_connection),
            identifier
        }
    }

    pub fn send_message(&self, message: &str) {
        let send_payload = SendPayload {
            command: "message".to_string(),
            identifier: serde_json::to_string(&self.identifier).unwrap(),
            data: message.to_string()
        };
        let body = serde_json::to_string(&send_payload).unwrap();
        debug!("Sending message of {}...", body);
        let send_message = Message::Text(body);
        match self.websocket.lock().unwrap().write_message(send_message) {
            Ok(_) => debug!("Successfully send messages to the randomizer."),
            Err(e) => error!("send_message: Error sending messages to the randomizer - {:?}", e)
        }
    }

    pub fn read_messages(&self) -> Result<ReceivePayload, tungstenite::Error> {
        self.websocket.lock().unwrap().read_message().map(|message| {
            let data = message.into_data();
            serde_json::from_slice::<ReceivePayload>(data.as_ref()).expect("Did not receive expected payload from server.")
        })
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
