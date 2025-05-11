use async_std::io::ReadExt;
use async_std::net::TcpStream;
use core::str;
use rust_poker::card_tools::card::Card;
use rust_poker::card_tools::color::Color;
use rust_poker::card_tools::rank::Rank;
use serde_json;
use std::io::Write;
use std::time::Duration;

fn main() {
    let message = Card {
        rank: Rank::Queen,
        color: Color::Hearts,
    };
    let json = serde_json::to_string(&message);
    let mut stream = TcpStream::connect("127.0.0.1:7878");

    trpl::run(async {
            .await
            .expect("could not connect");
        loop {
            trpl::race(display_loading(), handle_connection(&listener)).await;
        }
    })
}

pub fn main() {
    // let message = "Hello world";




    stream
}

async fn handle_connection(stream: &mut TcpStream) {

    stream
        .write_all(json.unwrap().as_bytes())
        .expect("failed to write to stream");
    // let result = listener.accept();
    // (stream, _) = result.await.ok().unwrap();
    let mut buffer = [0; 1024];
    // stream.read(&mut buffer).unwrap();
    stream.read(&mut buffer).await.unwrap();

    // stream.read(&mut buffer).await.unwrap();

    let message = str::from_utf8(&buffer).unwrap();

    // message.push_str("\n");
    print!("{}", message);
}

async fn display_loading() {
    print!("...\n");
    trpl::sleep(Duration::from_millis(1000)).await;
}
