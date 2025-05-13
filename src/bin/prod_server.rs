use async_std::task::sleep;
use rust_poker::tcp::message_types::ServerMessageTypes;
use rust_poker::{game_types::board::Board, tcp::server_messenger::ServerMessenger};
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;
use trpl::Runtime;

#[tokio::main]
async fn main() {
    let messenger_arc = Arc::new(ServerMessenger::new("127.0.0.1:7878").await);
    let messenger_clone = Arc::clone(&messenger_arc);
    tokio::spawn(async move {
        messenger_clone.start().await;
    });

    loop {
        // 5 sec to join the game
        println!("Players ready");
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("Table closed");

        let stream_count = { messenger_arc.stream_count().await };
        for i in 0..stream_count {
            messenger_arc
                .send(i, ServerMessageTypes::JoinGame, "")
                .await;
            let message = messenger_arc.receive(i).await;
        }

        // let mut board = Board::new(player_names, money);

        // get number of streams

        println!("Tick")
    }
}
