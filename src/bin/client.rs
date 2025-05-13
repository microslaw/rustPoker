use rust_poker::game_types::board::{BoardDto, PlayerAction};
use rust_poker::tcp::client_messenger::ClientMessenger;
use rust_poker::tcp::message_types::{ClientMessageTypes, ServerMessageTypes};
use std::io::{self, Write};
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("\nWelcome to No Limit Texas Hold'em Poker!");

    let mut nickname: String = String::new();

    while nickname.is_empty() {
        println!("Enter your nickname:");
        print!(">");
        io::stdout().flush().unwrap();
        nickname = String::new();
        io::stdin().read_line(&mut nickname).unwrap();
        nickname = nickname.trim().to_string();
    }

    let mut messenger = ClientMessenger::new("127.0.0.1:7878").await;
    let mut message = messenger.receive().await;
    assert_eq!(message.message_type, ServerMessageTypes::JoinGame);
    messenger
        .send(ClientMessageTypes::JoinGameAcknowledgement, nickname)
        .await;

    loop {
        message = messenger.receive().await;
        assert_eq!(message.message_type, ServerMessageTypes::NextTurn);
        let board_dto: BoardDto = serde_json::from_str(&message.payload_json).unwrap();

        if board_dto.your_turn {
            println!("Your turn");
            println!("Your cards:\n{}", board_dto.your_cards);
            // if self.community_cards.len() > 0 {
            //     println!("Community cards:\n{}", self.community_cards);
            // }
            println!(
                "Current bet: {} $, your bet: {} $",
                board_dto.current_bet, board_dto.your_bet,
            );

            println!("Your money: {} $", board_dto.your_money);
            println!("Pot: {} $", board_dto.pot);

            println!("Actions: (f)old, (c)heck/call, (r)aise, (a)ll-in");
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();

            let action = match input.chars().next().unwrap_or(' ') {
                'f' => PlayerAction::Fold,
                'c' => {
                    if board_dto.your_bet < board_dto.your_bet {
                        PlayerAction::Call
                    } else {
                        PlayerAction::Check
                    }
                }
                'r' => {
                    println!("Enter raise amount (minimum {} $): ", board_dto.min_raise);
                    print!("> ");
                    io::stdout().flush().unwrap();

                    let mut amount = String::new();
                    io::stdin().read_line(&mut amount).unwrap();
                    let amount = amount.trim().parse::<u16>().unwrap_or(0);

                    PlayerAction::Raise(amount)
                }
                'a' => PlayerAction::AllIn,
                _ => {
                    println!("Invalid action, defaulting to check/call");
                    if board_dto.your_bet < board_dto.current_bet {
                        PlayerAction::Call
                    } else {
                        PlayerAction::Check
                    }
                }
            };

            messenger.send(ClientMessageTypes::PlayCard, action).await;
        } else {
            println!("Waiting for your turn");
            println!("Your cards:\n{}", board_dto.your_cards);
            // if self.community_cards.len() > 0 {
            //     println!("Community cards:\n{}", self.community_cards);
            // }
            println!(
                "Current bet: {} $, your bet: {} $",
                board_dto.current_bet, board_dto.your_bet,
            );

            println!("Your money: {} $", board_dto.your_money);
            println!("Pot: {} $", board_dto.pot);
        }
    }
}
