use tungstenite::{stream::MaybeTlsStream, WebSocket, connect, Message};
use std::sync::Mutex;
use log::error;
use std::net::TcpStream;
use serde::{Deserialize, Serialize};

static CHANNEL_NAME: &str = "MultiworldSyncChannel";
static DEFAULT_CHANNEL_ID: u64 = 15;

pub fn init(server_url: &str) -> Mutex<WebSocket<MaybeTlsStream<TcpStream>>> {
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
    Mutex::new(ws_connection)
}

#[derive(Serialize, Deserialize)]
pub struct InitialPayload {
    command: String,
    identifier: String
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

