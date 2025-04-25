use std::io::{self, Write};
use std::net::TcpStream;
// filepath: c:\projects\rust-poker\src\bin\server.rs
use rust_poker::card_tools::card::Card;
use rust_poker::card_tools::rank::Rank;
use rust_poker::card_tools::color::Color;
use serde_json;

pub fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("could not connect");

    // let message = "Hello world";

    let message = Card {
        rank: Rank::Queen,
        color: Color::Hearts,
    };

    let json = serde_json::to_string(&message);

    stream
        .write_all(json.unwrap().as_bytes())
        .expect("failed to write to stream");
}
