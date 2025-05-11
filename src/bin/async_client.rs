use async_std::io::ReadExt;
use async_std::net::TcpListener;
use core::str;
use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Duration;

// Connect to a local server.

fn main() {
    trpl::run(async {
        let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
        print!("entering loop\n");
        loop {
            print!("begining race\n");
            trpl::race(display_loading(), handle_connection(&listener)).await;
            print!("finished race\n")
        }
    })
}

async fn handle_connection(listener: &TcpListener) {
    print!("begun handling conection\n");
    let result = listener.accept();
    let mut stream;
    (stream, _) = result.await.ok().unwrap();
    let mut buffer = [0; 1024];
    // stream.read(&mut buffer).unwrap();
    stream.read(&mut buffer).await.unwrap();

    let message = str::from_utf8(&buffer).unwrap();
    print!("{}", message);
    print!("finished handling connection\n")
}

async fn display_loading() {
    print!("...\n");
    trpl::sleep(Duration::from_millis(1000)).await;
}
