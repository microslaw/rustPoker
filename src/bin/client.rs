
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};
use rust_poker::card_tools::card::Card;

pub fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }

    // client loop
    while true{

    }

}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let json: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();

    let c: Card = serde_json::from_str(&json.join("")).unwrap();

    println!("Request: {c}");
}
