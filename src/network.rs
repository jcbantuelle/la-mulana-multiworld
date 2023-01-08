use std::sync::Mutex;
use std::net::TcpStream;

use log::{debug, error};
use serde::{Deserialize, Serialize};
use tungstenite::{stream::MaybeTlsStream, WebSocket, connect, Message};

static CHANNEL_NAME: &str = "MultiworldSyncChannel";
static DEFAULT_CHANNEL_ID: u64 = 15;

pub struct Randomizer {
    pub websocket: Mutex<WebSocket<MaybeTlsStream<TcpStream>>>
}

impl Randomizer {
    pub fn new(server_url: &str) -> Randomizer {
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

        let ident = Identifier {
            id: DEFAULT_CHANNEL_ID,
            channel: CHANNEL_NAME.to_string()
        };
        let initial_payload = InitialPayload {
            command: "subscribe".to_string(),
            identifier: serde_json::to_string(&ident).unwrap()
        };

        ws_connection.write_message(Message::Text(serde_json::to_string(&initial_payload).unwrap())).expect("Unable to Connect To Websocket Channel");
        Randomizer {
            websocket: Mutex::new(ws_connection)
        }
    }

    pub fn send_message<T: Serialize>(&self, message: T) {
        let ident = Identifier {
            id: DEFAULT_CHANNEL_ID,
            channel: CHANNEL_NAME.to_string()
        };
        let payload = Payload {
            command: "message".to_string(),
            identifier: serde_json::to_string(&ident).unwrap(),
            data: serde_json::to_string(&message).unwrap()
        };
        let payload = serde_json::to_string(&payload).unwrap();
        let message = Message::Text(payload);
        match self.websocket.lock().unwrap().write_message(message) {
            Ok(_) => debug!("Successfully send messages to the randomizer."),
            Err(e) => error!("send_message: Error sending messages to the randomizer.")
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct InitialPayload {
    command: String,
    identifier: String
}

#[derive(Serialize, Deserialize)]
pub struct Payload {
    command: String,
    identifier: String,
    data: String
}

#[derive(Serialize, Deserialize)]
pub struct Identifier {
    id: u64,
    channel: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestMessagePayload {
    identifier: String,
    message: TestMessage
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestMessage {
    body: String
}

#[derive(Serialize, Deserialize)]
pub struct RandomizerMessage {
    pub player_id: u32,
    pub body: String
}

