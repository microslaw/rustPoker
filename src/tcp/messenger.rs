// use super::message_types::MessageTypes;
// use crate::tcp::message_types::{ClientMessageTypes, ServerMessageTypes};
// use async_std::io::{ReadExt, WriteExt};
// use core::str;
// use serde::{Deserialize, Serialize};
// use std::io::{BufRead, BufReader, Write};
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::net::{TcpListener, TcpStream};
// use trpl::Runtime;

// pub struct ServerMessenger {
//     listener: TcpListener,
//     addr: String,
//     connections: Vec<TcpStream>,
// }

// trait MessagePayload<'de>: Serialize + Deserialize<'de> {}

// impl ServerMessenger {
//     fn new(addr: &String) -> Self {
//         let rt = Runtime::new().unwrap();
//         let new_server = rt.block_on(async {
//             ServerMessenger {
//                 listener: TcpListener::bind(addr).await.unwrap(),
//                 addr: addr.clone(),
//                 connections: Vec::new(),
//             }
//         });

//         // Accept new connections

//         println!("Server started on {}", addr);

//         return new_server;
//     }

//     async fn start(&mut self) {
//         loop {
//             let mut stream: TcpStream;
//             (stream, _) = self.listener.accept().await.unwrap();
//             self.connections.push(stream);

//             // tokio::spawn(self.handle_connection(stream))
//         }
//     }

//     async fn handle_connection(stream: TcpStream){
//         let mut buffer = [0;1024];
//         // stream.read(buffer)
//     }

//     async fn send<T, MessageTypes>(&mut self, message_type: MessageTypes, message_payload: T)
//     where
//         T: for<'a> MessagePayload<'a>,
//         MessageTypes: Serialize,
//     {
//         let type_json = serde_json::to_string(&message_type).unwrap();
//         let payload_json = serde_json::to_string(&message_payload).unwrap();

//         self.sender.write_all(type_json.as_bytes()).await.unwrap();
//         self.sender
//             .write_all(payload_json.as_bytes())
//             .await
//             .unwrap();
//     }

//     async fn receive_from_client(&mut self) -> (ClientMessageTypes, String) where {
//         let mut stream_type;
//         (stream_type, _) = self.listener.accept().await.ok().unwrap();
//         // (stream, _) = tcp_result.await.ok().unwrap();
//         let mut buffer = [0; 1024];
//         stream_type.read(&mut buffer).await.unwrap();
//         let message = str::from_utf8(&buffer)
//             .unwrap()
//             .trim_matches(char::from(0))
//             .trim();

//         let message_type: ClientMessageTypes = serde_json::from_str(&message).unwrap();

//         let mut stream_payload;
//         (stream_payload, _) = self.listener.accept().await.ok().unwrap();
//         let mut buffer = [0; 1024];
//         stream_payload.read(&mut buffer).await.unwrap();
//         let message = str::from_utf8(&buffer)
//             .unwrap()
//             .trim_matches(char::from(0))
//             .trim()
//             .to_owned();

//         return (message_type, message);
//     }
// }
