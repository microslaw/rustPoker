mod card_tools;
mod game_types;

use std::io::{self, Write};
use game_types::board::Board;

fn main() {
    println!("\nWelcome to No Limit Texas Hold'em Poker!");
    println!("How many players? (2-10)");
    
    print!("> ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num_players = input.trim().parse::<usize>().unwrap_or(2);
    
    let num_players = num_players.clamp(2, 10);
    
    let mut player_names = Vec::new();
    for i in 1..=num_players {
        println!("Enter name for Player {}: ", i);
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        let name = name.trim().to_string();
        
        if name.is_empty() {
            player_names.push(format!("Player {}", i));
        } else {
            player_names.push(name);
        }
    }
    
    println!("Enter starting money amount (default: 100 $): ");
    print!("> ");
    io::stdout().flush().unwrap();
    
    let mut money = String::new();
    io::stdin().read_line(&mut money).unwrap();
    let money = money.trim().parse::<u16>().unwrap_or(100);
    
    // Create board and start game
    let mut board = Board::new(player_names, money);
    board.game_loop();
}