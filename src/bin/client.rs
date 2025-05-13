use rust_poker::tcp::client_messenger::ClientMessenger;
use rust_poker::tcp::message_types::{ClientMessageTypes, ServerMessageTypes};
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("\nWelcome to No Limit Texas Hold'em Poker!");

    let mut nickname: String = String::new();

    while nickname.is_empty() {
        println!("Enter your nickname:");
        print!(">");
        // io::stdout().flush().unwrap();
        // nickname = String::new();
        // io::stdin().read_line(&mut nickname).unwrap();
        // nickname = nickname.trim().to_string();
        nickname = "microslaw".to_string();
    }

    let mut messenger = ClientMessenger::new("127.0.0.1:7878").await;
    let message = messenger.receive().await;
    assert_eq!(message.message_type, ServerMessageTypes::JoinGame);
    messenger
        .send(ClientMessageTypes::JoinGameAcknowledgement, nickname)
        .await;

    tokio::time::sleep(Duration::from_secs(10)).await;
    messenger
        .send(ClientMessageTypes::JoinGameAcknowledgement, "a")
        .await;
}
