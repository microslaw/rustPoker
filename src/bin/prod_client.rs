use rust_poker::tcp::client_messenger::ClientMessenger;
use rust_poker::tcp::message_types::ClientMessageTypes;
use rust_poker::tcp::message_types::ServerMessageTypes;

#[tokio::main]
async fn main() {
    let mut messenger = ClientMessenger::new("127.0.0.1:7878").await;

    messenger.send(ClientMessageTypes::JoinGameAcknowledgement, "paylaod").await;


}
