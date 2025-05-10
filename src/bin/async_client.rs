use async_std::task;
use core::str;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::time::Duration;

// Connect to a local server.

fn main() {
    trpl::run(async {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            // Warning: This is not concurrent!
            trpl::race(handle_connection(stream), display_loading()).await;
        }
    })
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // let get = b"GET / HTTP/1.1\r\n";
    // let sleep = b"GET /sleep HTTP/1.1\r\n";

    // let (status_line, filename) = if buffer.starts_with(get) {
    //     ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    // } else if buffer.starts_with(sleep) {
    //     task::sleep(Duration::from_secs(5)).await;
    //     ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    // };
    // // let contents = fs::read_to_string(filename).unwrap();

    let message = str::from_utf8(&buffer).unwrap();

    print!("{}", message);
}

// pub fn main() {
//     trpl::run(async {
//         print!("Hello world");
//         let stream = Async::<TcpStream>::connect(([127, 0, 0, 1], 8000))
//             .await
//             .unwrap();

//         loop {
//             trpl::run(async {
//                 match trpl::race(handle_connection(stream), display_loading()).await {
//                     Either::Left(left) => left,
//                     Either::Right(right) => right,
//                 };

//                 print!("Here");
//             })
//         }
//     });
// }

// async fn handle_connection(mut stream: Async<TcpStream>) {
//     let buf_reader = BufReader::new(&stream);

//     // let json2 = buf_reader.lines().map(|result| result.unwrap()).collect();
//     let mut message = buf_reader.buffer();

//     // let json: Vec<_> = buf_reader.lines().map(|result| result.unwrap())
//     let mut buf: String = String::new();

//     let result = message.read_to_string(&mut buf);
//     print!("{:?}",result);
//     print!("{}",buf);

//     // .await;

//     // let response = "HTTP/1.1 200 OK\r\n\r\n";

//     // stream.wri(response.as_bytes()).unwrap();

//     // let c: Card = serde_json::from_str(&json.join("")).unwrap();

//     // println!("Request: {c}");
// }

async fn display_loading() {
    print!("...");
    trpl::sleep(Duration::from_millis(1000)).await;
}


// use async_std::task;
// use core::str;
// use std::fs;
// use std::io::prelude::*;
// use std::net::TcpListener;
// use std::net::TcpStream;
// use std::time::Duration;
// use async_std::future;

// fn main() {
//     trpl::run(async {
//         let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

//         loop {
//             // Use a timeout to alternate between handling connections and displaying loading
//             let result = future::timeout(Duration::from_secs(1), async {
//                 if let Ok((stream, _)) = listener.accept() {
//                     handle_connection(stream).await;
//                 }
//             })
//             .await;

//             if result.is_err() {
//                 // Timeout occurred, no connection was received
//                 display_loading().await;
//             }
//         }
//     });
// }

// async fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 1024];
//     stream.read(&mut buffer).unwrap();

//     let message = str::from_utf8(&buffer).unwrap();
//     print!("{}", message);
// }

// async fn display_loading() {
//     println!("...");
//     trpl::sleep(Duration::from_millis(1000)).await;
// }
