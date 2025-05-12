use super::message_types::MessageTypes;
use crate::tcp::message_types::{ClientMessageTypes, ServerMessageTypes};
use async_std::io::{ReadExt, WriteExt};
use async_std::stream;
use core::str;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use trpl::Runtime;

pub struct ServerMessenger {
    listener: TcpListener,
    addr: String,
    connections: Vec<TcpStream>,
}

trait MessagePayload<'de>: Serialize + Deserialize<'de> {}

impl ServerMessenger {
    pub fn new(addr: &'static str) -> Self {
        let rt = Runtime::new().unwrap();
        let new_server = rt.block_on(async {
            ServerMessenger {
                listener: TcpListener::bind(addr).await.unwrap(),
                addr: addr.to_string(),
                connections: Vec::new(),
            }
        });

        return new_server;
    }

    pub async fn start(&mut self) {
        println!("Server started on {}", self.addr);
        loop {
            let stream: TcpStream;
            (stream, _) = self.listener.accept().await.unwrap();
            self.connections.push(stream);
        }
    }

    async fn send<T, MessageTypes>(
        &mut self,
        client_id: usize,
        message_type: MessageTypes,
        message_payload: T,
    ) where
        T: for<'a> MessagePayload<'a>,
        MessageTypes: Serialize,
    {
        let type_json = serde_json::to_string(&message_type).unwrap();
        let payload_json = serde_json::to_string(&message_payload).unwrap();

        let stream = self.connections.get_mut(client_id).unwrap();

        stream
            .write_all(type_json.as_bytes())
            .await
            .unwrap();
        stream
            .write_all(payload_json.as_bytes())
            .await
            .unwrap();
    }

    async fn receive(&mut self, client_id: usize) -> (ClientMessageTypes, String) where {
        let mut buffer = [0; 1024];
        let stream = self.connections.get_mut(client_id).unwrap();

        stream.read(&mut buffer).await.unwrap();
        let message = str::from_utf8(&buffer)
            .unwrap()
            .trim_matches(char::from(0))
            .trim();
        let message_type: ClientMessageTypes = serde_json::from_str(&message).unwrap();

        stream.read(&mut buffer).await.unwrap();
        let message = str::from_utf8(&buffer)
            .unwrap()
            .trim_matches(char::from(0))
            .trim()
            .to_owned();

        return (message_type, message);
    }
}
