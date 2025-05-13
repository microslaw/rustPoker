use rust_poker::tcp::message_types::{ClientMessageTypes, ServerMessageTypes};
use rust_poker::{game_types::board::Board, tcp::server_messenger::ServerMessenger};
use std::{sync::Arc, time::Duration};
use tokio::io::{self, AsyncBufReadExt};

#[tokio::main]
async fn main() {
    let messenger_arc = Arc::new(ServerMessenger::new("127.0.0.1:7878").await);
    let messenger_clone = Arc::clone(&messenger_arc);
    tokio::spawn(async move {
        messenger_clone.start().await;
    });

    loop {
        println!("Waiting for players");

        let mut input = String::new();
        let mut stdin = io::BufReader::new(io::stdin());
        stdin.read_line(&mut input).await.unwrap();

        println!("Joining closed");

        let mut player_names = Vec::new();
        let stream_count = { messenger_arc.stream_count().await };
        for i in 0..stream_count {
            messenger_arc
                .send(i, ServerMessageTypes::JoinGame, "")
                .await;
            let message = messenger_arc.receive(i).await;
            assert_eq!(
                message.message_type,
                ClientMessageTypes::JoinGameAcknowledgement
            );
            let nickname: String = serde_json::from_str(&message.payload_json).unwrap();
            player_names.push(nickname);
        }

        let mut board = Board::new(player_names, 100, Arc::clone(&messenger_arc));

        board.game_loop().await;
    }
}
