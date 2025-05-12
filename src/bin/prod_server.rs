use async_std::task::sleep;
use rust_poker::tcp::server_messenger::ServerMessenger;
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;
use trpl::Runtime;

#[tokio::main]
async fn main() {
    let messenger = Arc::new(ServerMessenger::new("127.0.0.1:7878").await);
    let messenger_clone = Arc::clone(&messenger);
    tokio::spawn(async move {
        messenger_clone.start().await;
    });

    loop {
        tokio::time::sleep(Duration::from_secs(5)).await;
        println!("Start tick");

        let stream_count: usize;
        {
            stream_count = messenger.streams.lock().await.len();
        }

        // {
        //     if let Ok(mut streams) = messenger.streams.try_lock() {
        //         println!("Mutex is not locked. Acquired lock successfully.");
        //         // Perform operations on the streams
        //     } else {
        //         println!("Mutex is locked. Could not acquire lock.");
        //     }
        // }

        // get number of streams

        for i in 0..stream_count {
            let message = messenger.receive(i).await;
            println!("{}, {}", message.message_type, message.payload_json);
        }

        println!("Tick")
    }
}
