use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use crate::tcp::message_types::{ClientMessageTypes, ServerMessageTypes};

pub struct Messenger {
    listener: TcpListener,
    sender: TcpStream,
}

trait MessagePayload<'de>: Serialize + Deserialize<'de> {}

impl Messenger {
    fn new(addr: &String) -> Self {
        Messenger {
            listener: TcpListener::bind(addr).unwrap(),
            sender: TcpStream::connect(addr).unwrap(),
        }
    }
    fn send_to_server<T>(&mut self, message_type: ServerMessageTypes, message_payload: T)
    where
        T: for<'a> MessagePayload<'a>,
    {
        let type_json = serde_json::to_string(&message_type).unwrap();
        let payload_json = serde_json::to_string(&message_payload).unwrap();

        self.sender.write_all(type_json.as_bytes()).unwrap();
        self.sender.write_all(payload_json.as_bytes()).unwrap();
    }

    fn send_to_client<T>(&mut self, message_type: ClientMessageTypes, message_payload: T)
    where
        T: for<'a> MessagePayload<'a>,
    {
        let type_json = serde_json::to_string(&message_type).unwrap();
        let payload_json = serde_json::to_string(&message_payload).unwrap();

        self.sender.write_all(type_json.as_bytes()).unwrap();
        self.sender.write_all(payload_json.as_bytes()).unwrap();
    }

    fn parse_json(buf_reader: BufReader<&TcpStream>) -> String {
        let json_vec: Vec<String> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        json_vec.join("")
    }

    fn receive_from_client(&mut self, mut stream: TcpStream) {
        let json_message_type = Self::parse_json(BufReader::new(&stream));
        let message_type: ClientMessageTypes = serde_json::from_str(&json_message_type).unwrap();

        let json_message_payload = Self::parse_json(BufReader::new(&stream));
        // let message_payload :MessagePayload = Self::parse_json(&json_message_payload).unwrap();

    }
}
