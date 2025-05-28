use rust_poker::game_types::board::{BoardDto, PlayerAction};
use rust_poker::tcp::client_messenger::ClientMessenger;
use rust_poker::tcp::message_types::{ClientMessageTypes, ServerMessageTypes};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use eframe::{egui, App, NativeOptions};
use serde_json;

struct PokerClientApp {
    nickname: String,
    board: Option<BoardDto>,
    input_nickname: String,
    input_raise: String,
    action_sender: Option<Sender<PlayerAction>>,
    board_receiver: Option<Receiver<BoardDto>>,
    waiting_for_action: bool,
    status: String,
}

impl Default for PokerClientApp {
    fn default() -> Self {
        Self {
            nickname: String::new(),
            board: None,
            input_nickname: String::new(),
            input_raise: String::new(),
            action_sender: None,
            board_receiver: None,
            waiting_for_action: false,
            status: String::new(),
        }
    }
}

impl App for PokerClientApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.nickname.is_empty() {
                ui.heading("Enter your nickname:");
                ui.text_edit_singleline(&mut self.input_nickname);
                if ui.button("Join Game").clicked() && !self.input_nickname.trim().is_empty() {
                    self.nickname = self.input_nickname.trim().to_string();
                    let (action_tx, action_rx) = mpsc::channel();
                    let (board_tx, board_rx) = mpsc::channel();
                    let nickname = self.nickname.clone();
                    thread::spawn(move || {
                        tokio::runtime::Runtime::new().unwrap().block_on(async move {
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
                                board_tx.send(board_dto.clone()).ok();
                                if board_dto.your_turn {
                                    if let Ok(action) = action_rx.recv() {
                                        messenger.send(ClientMessageTypes::PlayCard, action).await;
                                    }
                                }
                            }
                        });
                    });
                    self.action_sender = Some(action_tx);
                    self.board_receiver = Some(board_rx);
                }
                return;
            }

            if let Some(rx) = &self.board_receiver {
                while let Ok(board) = rx.try_recv() {
                    self.board = Some(board);
                    self.waiting_for_action = self.board.as_ref().map_or(false, |b| b.your_turn);
                    self.status.clear();
                }
            }

            if let Some(board) = &self.board {
                let board = board.clone();
                ui.heading(format!("Welcome, {}", self.nickname));
                ui.label(format!("Pot: {} $", board.pot));
                ui.label(format!("Your money: {} $", board.your_money));
                ui.label(format!("Your cards: {}", board.your_cards));
                if !board.community_cards.is_empty() {
                    ui.label(format!("Community cards: {}", board.community_cards));
                }
                ui.label(format!("Current bet: {} $, your bet: {} $", board.current_bet, board.your_bet));
                ui.label(format!("Small blind: {} posted {} $", board.blinds.small_blind_owner, board.blinds.small_blind));
                ui.label(format!("Big blind: {} posted {} $", board.blinds.big_blind_owner, board.blinds.big_blind));

                if board.your_turn {
                    ui.separator();
                    ui.label("Your turn: Choose action");

                    let mut input_raise = std::mem::take(&mut self.input_raise);
                    let mut status = std::mem::take(&mut self.status);

                    ui.horizontal(|ui| {
                        if ui.button("Fold").clicked() {
                            self.send_action(PlayerAction::Fold);
                        }
                        if ui.button("Check/Call").clicked() {
                            let action = if board.your_bet < board.current_bet {
                                PlayerAction::Call
                            } else {
                                PlayerAction::Check
                            };
                            self.send_action(action);
                        }
                        ui.label("Raise:");
                        ui.text_edit_singleline(&mut input_raise);
                        if ui.button("Raise").clicked() {
                            if let Ok(amount) = input_raise.trim().parse::<u16>() {
                                if amount >= board.min_raise {
                                    self.send_action(PlayerAction::Raise(amount));
                                    input_raise.clear();
                                } else {
                                    status = format!("Raise must be at least {}", board.min_raise);
                                }
                            } else {
                                status = "Invalid raise amount".to_string();
                            }
                        }
                        if ui.button("All-in").clicked() {
                            self.send_action(PlayerAction::AllIn);
                        }
                    });

                    self.input_raise = input_raise;
                    self.status = status;
                } else {
                    ui.separator();
                    ui.label(format!("Waiting for {} to play...", board.current_player_name));
                }

            }
            if !self.status.is_empty() {
                ui.colored_label(egui::Color32::RED, &self.status);
            }
        });
    }
}

impl PokerClientApp {
    fn send_action(&mut self, action: PlayerAction) {
        if let Some(tx) = &self.action_sender {
            tx.send(action).ok();
            self.waiting_for_action = false;
        }
    }
}

fn main() {
    let app = PokerClientApp::default();
    let native_options = NativeOptions::default();
    let _ = eframe::run_native(
        "Rust Poker Client",
        native_options,
        Box::new(|_cc| Ok::<Box<dyn App>, Box<dyn std::error::Error + Send + Sync>>(Box::new(app))),
    );
}