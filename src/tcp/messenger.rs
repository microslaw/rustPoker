use crate::tcp::message_types::{ClientMessageTypes, ServerMessageTypes};
use async_std::{
    io::{ReadExt, WriteExt},
    net::{TcpListener, TcpStream},
};
use core::str;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};

use super::message_types::MessageTypes;

pub struct Messenger {
    listener: TcpListener,
    sender: TcpStream,
    addr: String,
}

trait MessagePayload<'de>: Serialize + Deserialize<'de> {}

impl Messenger {
    fn new(addr: &String) -> Self {
        trpl::run(async {
            Messenger {
                listener: TcpListener::bind(addr).await.unwrap(),
                sender: TcpStream::connect(addr).await.unwrap(),
                addr: addr.clone(),
            }
        })
    }
    async fn send<T, MessageTypes>(&mut self, message_type: MessageTypes, message_payload: T)
    where
        T: for<'a> MessagePayload<'a>,
        MessageTypes: Serialize,
    {
        let type_json = serde_json::to_string(&message_type).unwrap();
        let payload_json = serde_json::to_string(&message_payload).unwrap();

        self.sender.write_all(type_json.as_bytes()).await.unwrap();
        self.sender
            .write_all(payload_json.as_bytes())
            .await
            .unwrap();
    }


    async fn <T>receive_from_client(&mut self) -> (T, String) where  {
        let mut stream_type;
        (stream_type, _) = self.listener.accept().await.ok().unwrap();
        // (stream, _) = tcp_result.await.ok().unwrap();
        let mut buffer = [0; 1024];
        stream_type.read(&mut buffer).await.unwrap();
        let message = str::from_utf8(&buffer)
            .unwrap()
            .trim_matches(char::from(0))
            .trim();

        let message_type: ClientMessageTypes = serde_json::from_str(&message).unwrap();

        let mut stream_payload;
        (stream_payload, _) = self.listener.accept().await.ok().unwrap();
        let mut buffer = [0; 1024];
        stream_payload.read(&mut buffer).await.unwrap();
        let message = str::from_utf8(&buffer)
            .unwrap()
            .trim_matches(char::from(0))
            .trim()
            .to_owned();

        return (message_type, message);
    }
}
