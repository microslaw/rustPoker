# Rust Poker Game

## Game description

This project implements the poker card game using rust. It uses a client-server architecture with tcp protocol, and gui based on eframe. The game allows multiple players to join a session.

## File structure

- bin - game entrypoints
- card_tools - set of functions and structs for representing game cards
- game_types - more advanced game flow
- tcp - client-server communication

## Concurrent programming

This project uses asynchronous programming with the Tokio runtime to handle multiple clients concurrently. The server can manage multiple game sessions simultaneously, allowing players to join and play without blocking each other.
Methods used in concurrent programming:

- ClientMessenger, ServerMessenger::send, receive - sending and receiving messages between clients and the server
- ServerMessenger::start - begin accepting connections from clients
- Board::notify_everyone - send information about game state changes to all players
- Board::get_player_action - wait for a response with player's action from the client
- Client::update - clients game loop

## External liblaries

- [tokio](https://crates.io/crates/tokio) - Asynchronous runtime for networking and concurrency
- [serde](https://crates.io/crates/serde) - Serialization/deserialization of game messages
- [rand](https://crates.io/crates/rand) - Random number generation for shuffling cards
- [eframe](https://crates.io/crates/eframe) - GUI framework for building the game interface

## Game screenshots

Name input:

![alt text](https://github.com/microslaw/rustPoker/blob/main/screenshots/name_input.jpg)

UI:

![alt text](https://github.com/microslaw/rustPoker/blob/main/screenshots/ui.jpg)

## Contributions

- Miłosz Grunwald (microslaw) - basic datatypes, tcp communication
- Michał Szablewski (mszablewski) - card hand comparison, gui
- Wiktor Nazaruk (wiktornazaruk) - general game logic, gui
