// Copyright Ryan Goldstein

// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use thiserror::Error;
use log::{error, debug};
use websocket::message::Type;
use websocket::{ClientBuilder, WebSocketError, OwnedMessage};
use websocket::sync::Client;
use websocket::sync::stream::NetworkStream;
use crate::archipelago::protocol::*;

#[derive(Error, Debug)]
pub enum ArchipelagoError {
    #[error("illegal response")]
    IllegalResponse {
        received: ServerMessage,
        expected: &'static str,
    },
    #[error("connection closed by server")]
    ConnectionClosed,
    #[error("data failed to serialize")]
    FailedSerialize(#[from] serde_json::Error),
    #[error("unexpected non-text result from websocket")]
    NonTextWebsocketResult(OwnedMessage),
    #[error("network error")]
    NetworkError(#[from] WebSocketError),
}

/**
 * A convenience layer to manage your connection to and communication with Archipelago
 */
pub struct ArchipelagoClient {
    ws: Client<Box<dyn NetworkStream + Send>>,
    room_info: RoomInfo,
    data_package: Option<DataPackageObject>,
    pub message_queue: Vec<ServerMessage>,
}

impl ArchipelagoClient {
     /**
     * Create an instance of the client and connect to the server on the given URL
     */
    pub fn new(url: &str) -> Result<ArchipelagoClient, ArchipelagoError> {
        // Attempt WSS, downgrade to WS if the TLS handshake fails
        let mut wss_url = String::new();
        wss_url.push_str("wss://");
        wss_url.push_str(url);

        let mut ws = match ClientBuilder::new(&wss_url).unwrap().connect(None) {
            Ok(result) => result,
            Err(_) => {
                let mut ws_url = String::new();
                ws_url.push_str("ws://");
                ws_url.push_str(url);

                match ClientBuilder::new(&ws_url).unwrap().connect(None) {
                    Ok(result) => result,
                    Err(error) => {
                        return Err(ArchipelagoError::NetworkError(error))        
                    }
                }
            },
        };
        
        let room_info = match Self::recv(&mut ws) {
            Ok(message) => {
                match message {
                    Some(ServerMessage::RoomInfo(room)) => room,
                    Some(received) => {
                        return Err(ArchipelagoError::IllegalResponse {
                            received,
                            expected: "Expected RoomInfo",
                        })
                    },
                    None => return Err(ArchipelagoError::ConnectionClosed)
                }
            },
            Err(e) => return Err(ArchipelagoError::ConnectionClosed)
        };

        Ok(ArchipelagoClient {
            ws,
            room_info,
            data_package: None,
            message_queue: vec![]
        })
    }

    pub fn send(&mut self, message: ClientMessage) -> Result<(), ArchipelagoError> {
        let request = serde_json::to_string(&[message])?;
        let message = OwnedMessage::Text(request);
        self.ws.send_message(&message);
        Ok(())
    }

    pub fn read(&mut self) -> Result<Option<ServerMessage>, ArchipelagoError> {
        Self::recv(&mut self.ws)
    }

    /**
     * Read a message from the server
     */
    fn recv(ws: &mut Client<Box<dyn NetworkStream + Send>>) -> Result<Option<ServerMessage>, ArchipelagoError> {
        match ws.recv_message() {
            Ok(message) => {
                match message {
                    OwnedMessage::Ping(ping) => {
                        let pong = OwnedMessage::Pong(ping);
                        ws.send_message(&pong);
                        Ok(None)
                    },
                    OwnedMessage::Text(response) => {
                        match serde_json::from_str::<Vec<ServerMessage>>(&response) {
                            Ok(text) => Ok(text.into_iter().next()),
                            Err(e) => Err(e.into())
                        }
                    },
                    OwnedMessage::Close(_) => Err(ArchipelagoError::ConnectionClosed),
                    msg => Err(ArchipelagoError::NonTextWebsocketResult(msg)),
                }
            },
            Err(e) => Err(e.into())
        }
    }

    /**
     * Send a connect request to the Archipelago server
     *
     * Will attempt to read a Connected packet in response, and will return an error if
     * another packet is found
     */
    pub fn connect(
        &mut self,
        game: &str,
        name: &str,
        uuid: &str,
        password: Option<&str>,
        items_handling: Option<i32>,
        tags: Vec<String>,
        slot_data: bool,
    ) -> Result<Connected, ArchipelagoError> {
        match self.send(ClientMessage::Connect(Connect {
            game: game.to_string(),
            name: name.to_string(),
            uuid: uuid.to_string(),
            password: password.map(|p| p.to_string()),
            version: network_version(),
            items_handling,
            tags,
            slot_data,
        })) {
            Ok(_) => (),
            Err(_) => return Err(ArchipelagoError::ConnectionClosed)
        };

        match self.read() {
            Ok(message) => {
                match message {
                    Some(ServerMessage::Connected(connected)) => Ok(connected),
                    Some(received) => {
                        Err(ArchipelagoError::IllegalResponse {
                            received,
                            expected: "Expected Connected",
                        })
                    },
                    None => return Err(ArchipelagoError::ConnectionClosed)
                }
            },
            Err(_) => return Err(ArchipelagoError::ConnectionClosed)
        }
    }

    pub fn location_checks(&mut self, locations: Vec<u64>) -> Result<(), ArchipelagoError> {
        match self.send(ClientMessage::LocationChecks(LocationChecks { locations })) {
            Ok(_) => Ok(()),
            Err(_) => Err(ArchipelagoError::ConnectionClosed)
        }
    }

    /**
     * Sent to server to request a ReceivedItems packet to synchronize items.
     *
     * Will buffer any non-ReceivedItems packets returned
     */
    pub fn sync(&mut self) -> Result<(), ArchipelagoError> {
        match self.send(ClientMessage::Sync) {
            Ok(_) => Ok(()),
            Err(_) => Err(ArchipelagoError::ConnectionClosed)
        }
    }
}
