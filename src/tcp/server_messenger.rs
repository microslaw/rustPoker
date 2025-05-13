use crate::tcp::message_types::{ClientMessageTypes, ServerMessageTypes};
use core::str;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::time::{Sleep, timeout};
use trpl::Runtime;

use super::client_messenger::ClientMessage;

pub struct ServerMessenger {
    listener: TcpListener,
    addr: String,
    pub streams: Arc<Mutex<Vec<TcpStream>>>,
}

#[derive(Serialize, Deserialize)]
pub struct ServerMessage {
    pub message_type: ServerMessageTypes,
    pub payload_json: String,
}

impl ServerMessenger {
    pub async fn new(addr: &'static str) -> Self {
        ServerMessenger {
            listener: TcpListener::bind(addr).await.unwrap(),
            addr: addr.to_string(),
            streams: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn start(&self) {
        println!("Server started on {}", self.addr);
        loop {
            let (stream, _) = self.listener.accept().await.unwrap();
            println!("Added connection");

            let streams = self.streams.clone();

            tokio::spawn(async move {
                let mut streams = streams.lock().await;
                streams.push(stream);
                println!("Finished adding");
            });
        }
    }

    pub async fn stream_count(&self) -> usize {
        self.streams.lock().await.len()
    }

    pub async fn send<T>(
        &self,
        client_id: usize,
        message_type: ServerMessageTypes,
        message_payload: T,
    ) where
        T: Serialize,
    {
        let payload_json = serde_json::to_string(&message_payload).unwrap();

        let message = ServerMessage {
            message_type,
            payload_json,
        };
        let message_json = serde_json::to_string(&message).unwrap();

        println!("sending {}", message_json);
        let mut streams = self.streams.lock().await;
        let stream = streams.get_mut(client_id).unwrap();
        stream.write_all(message_json.as_bytes()).await.unwrap();
    }

    pub async fn receive(&self, client_id: usize) -> ClientMessage where {
        let mut buffer: [u8; 1024] = [0; 1024];
        // let count = { self.stream_count().await };
        // println!("id {}, size {}", client_id, count);
        let mut streams = self.streams.lock().await;

        let stream = streams.get_mut(client_id).unwrap();

        while buffer[0] == 0 {
            stream.read(&mut buffer).await.unwrap();
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        let message_json = str::from_utf8(&buffer)
            .unwrap()
            .trim_matches(char::from(0))
            .trim();

        println!("received {}", message_json);
        // for byte in message_json.as_bytes() {
        //     print!("{:02X} ", byte); // Print each byte in hexadecimal format
        // }

        let message: ClientMessage = serde_json::from_str(&message_json).unwrap();
        return message;
    }
}
