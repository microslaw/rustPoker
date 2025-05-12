use crate::tcp::message_types::{ClientMessageTypes, ServerMessageTypes};
use async_std::io::{ReadExt, WriteExt};
use core::str;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use trpl::Runtime;

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

        self.stream.write_all(message_json.as_bytes()).await.unwrap();
    }

    pub async fn receive(&mut self) -> (ClientMessageTypes, String) where {
        let mut buffer = [0; 1024];

        self.stream.read(&mut buffer).await.unwrap();
        let message = str::from_utf8(&buffer)
            .unwrap()
            .trim_matches(char::from(0))
            .trim();
        let message_type: ClientMessageTypes = serde_json::from_str(&message).unwrap();

        self.stream.read(&mut buffer).await.unwrap();
        let message = str::from_utf8(&buffer)
            .unwrap()
            .trim_matches(char::from(0))
            .trim()
            .to_owned();

        return (message_type, message);
    }
}
