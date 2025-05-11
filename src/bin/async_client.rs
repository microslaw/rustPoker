use async_std::io::{ReadExt, WriteExt};
use async_std::net::TcpListener;
use core::str;
use rust_poker::card_tools::card::Card;
use std::time::Duration;

// Connect to a local server.

fn main() {
    trpl::run(async {
        let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
        loop {
            trpl::race(display_loading(), handle_connection(&listener)).await;
        }
    })
}

async fn handle_connection(listener: &TcpListener) {
    let result = listener.accept();
    let mut stream;
    (stream, _) = result.await.ok().unwrap();

    let mut buffer = [0; 1024];
    // stream.read(&mut buffer).unwrap();
    stream.read(&mut buffer).await.unwrap();

    let message = str::from_utf8(&buffer)
        .unwrap()
        .trim_matches(char::from(0))
        .trim();

    // message.push_str("\n");
    print!("{}", message);
    print!("{}", message.len());
    let card: Card = serde_json::from_str(&message).unwrap();
    print!("{}", card);

    stream.write(b"Ok");
}

async fn display_loading() {
    print!("...\n");
    trpl::sleep(Duration::from_millis(1000)).await;
}
