use crate::tcp::message_types::{ClientMessageTypes, ServerMessageTypes};
use core::str;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use trpl::Runtime;

use super::client_messenger::ClientMessage;

pub struct ServerMessenger {
    listener: TcpListener,
    addr: String,
    pub streams: Arc<Mutex<Vec<TcpStream>>>,
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

            let streams = self.streams.clone(); // Clone the Arc<Mutex<Vec<TcpStream>>>
            tokio::spawn(async move {
                let mut streams = streams.lock().await;
                streams.push(stream);
                println!("Finished adding");
            });
        }
    }

    pub async fn send<T>(
        &mut self,
        client_id: usize,
        message_type: ServerMessageTypes,
        message_payload: T,
    ) where
        T: Serialize,
    {
        let type_json = serde_json::to_string(&message_type).unwrap();
        let payload_json = serde_json::to_string(&message_payload).unwrap();

        let mut streams = self.streams.lock().await;
        let stream = streams.get_mut(client_id).unwrap();

        stream.write_all(type_json.as_bytes()).await.unwrap();
        stream.write_all(payload_json.as_bytes()).await.unwrap();
    }

    pub async fn receive(&self, client_id: usize) -> ClientMessage where {
        let mut buffer: Vec<u8> = Vec::new();
        let mut streams = self.streams.lock().await;
        let stream = streams.get_mut(client_id).unwrap();

        stream.read_to_end(&mut buffer).await.unwrap();
        let message_json = str::from_utf8(&buffer)
            .unwrap()
            .trim_matches(char::from(0))
            .trim_matches(char::from(0))
            .trim();


        println!("{}", message_json);
        for byte in message_json.as_bytes() {
            print!("{:02X} ", byte); // Print each byte in hexadecimal format
        }
        println!();

        let message: ClientMessage = serde_json::from_str(&message_json).unwrap();
        return message;
    }
}
