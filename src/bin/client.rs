use rust_poker::card_tools::card::Card;
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    time::Duration,
};
use trpl::Either;

pub fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        trpl::run(async {
            match trpl::race(handle_connection(stream), display_loading()).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

            print!("Here");
        })
    }

    print!("Here2");
}

async fn handle_connection(mut stream: TcpStream) {
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

async fn display_loading() {
    print!("...");
    trpl::sleep(Duration::from_millis(1000)).await;
}
