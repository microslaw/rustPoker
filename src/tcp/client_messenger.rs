use crate::tcp::message_types::{ClientMessageTypes, ServerMessageTypes};
use async_std::io::{ReadExt, WriteExt};
use async_std::stream;
use core::str;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use super::server_messenger::ServerMessage;

pub struct ClientMessenger {
    stream: TcpStream,
    server_addr: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClientMessage {
    pub message_type: ClientMessageTypes,
    pub payload_json: String,
}

impl ClientMessenger {
    pub async fn new(server_addr: &'static str) -> Self {
        ClientMessenger {
            stream: TcpStream::connect(server_addr).await.unwrap(),
            server_addr: server_addr.to_string(),
        }
    }

    pub async fn send<T>(&mut self, message_type: ClientMessageTypes, message_payload: T)
    where
        T: Serialize,
    {
        let payload_json = serde_json::to_string(&message_payload).unwrap();

        let message = ClientMessage {
            message_type,
            payload_json,
        };

        let message_json = serde_json::to_string(&message).unwrap();

        println!("sending {}", message_json);
        self.stream
            .write_all(message_json.as_bytes())
            .await
            .unwrap();
    }

    pub async fn receive(&mut self) -> ServerMessage {

        let mut buffer:[u8; 1024] = [0; 1024];

        while buffer[0] == 0 {
            self.stream.read(&mut buffer).await.unwrap();
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        let message_json = str::from_utf8(&buffer)
            .unwrap()
            .trim_matches(char::from(0))
            .trim();

        println!("received {}", message_json);

        let message: ServerMessage = serde_json::from_str(&message_json).unwrap();


        return message;
    }
}
