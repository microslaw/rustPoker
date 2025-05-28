use crate::card_tools::card::Card;
use crate::card_tools::hand::Hand;
use crate::card_tools::hand::get_sorted_deck;
use crate::game_types::player::Player;
use crate::tcp::message_types;
use crate::tcp::message_types::{ClientMessageTypes, ServerMessageTypes};
use crate::tcp::server_messenger::ServerMessage;
use crate::tcp::server_messenger::ServerMessenger;
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;
use std::io::{self, Write};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub enum PlayerAction {
    Fold,
    Check,
    Call,
    Raise(u16),
    AllIn,
}

pub enum GameStage {
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BoardDto {
    pub current_bet: u16,
    pub your_bet: u16,
    pub your_money: u16,
    pub pot: u16,
    pub your_cards: Hand,
    pub community_cards: Hand,
    pub your_turn: bool,
    pub current_player_name: String,
    pub min_raise: u16,
    pub blinds: BlindsDto,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BlindsDto {
    pub small_blind: u16,
    pub small_blind_owner: String,
    pub big_blind: u16,
    pub big_blind_owner: String,
}

pub struct Board {
    pub players: Vec<Player>,
    pub community_cards: Hand,
    pub deck: Hand,
    pub pot: u16,
    pub current_bet: u16,
    pub current_player_idx: usize,
    pub dealer_pos: usize,
    pub game_stage: GameStage,
    pub active_players: Vec<bool>,
    pub min_raise: u16,
    pub messenger_arc: Arc<ServerMessenger>,
}

impl Board {
    pub fn new(player_names: Vec<String>, money: u16, messenger_arc: Arc<ServerMessenger>) -> Self {
        let mut board = Board {
            players: Vec::new(),
            community_cards: Hand::default(),
            deck: get_sorted_deck(),
            pot: 0,
            current_bet: 0,
            current_player_idx: 0,
            dealer_pos: 0,
            game_stage: GameStage::PreFlop,
            active_players: Vec::new(),
            min_raise: 2, // Minimum raise is typically the big blind
            messenger_arc: messenger_arc,
        };

        // Initialize players
        for name in player_names {
            board.players.push(Player::new(name, money));
            board.active_players.push(true);
        }

        board
    }

    pub fn shuffle_deck(&mut self) {
        // Reset deck and get a fresh one
        self.deck = get_sorted_deck();

        // Fisher-Yates shuffle
        let mut rng = rand::thread_rng();
        let len = self.deck.len();

        for i in (1..len).rev() {
            let j = rng.gen_range(0..=i);
            self.deck.swap_cards(i, j);
        }
    }

    pub fn deal_hole_cards(&mut self) {
        // Deal 2 cards to each player
        for _ in 0..2 {
            for player_idx in 0..self.players.len() {
                if let Some(card) = self.deck.pop_card() {
                    self.players[player_idx].add_card(card);
                }
            }
        }
    }

    pub fn deal_community_cards(&mut self, count: usize) {
        // Deal specified number of cards to the community
        for _ in 0..count {
            if let Some(card) = self.deck.pop_card() {
                self.community_cards.add_card(card);
            }
        }
    }

    pub fn next_player(&mut self) -> usize {
        // Find the next active player
        let player_count = self.players.len();
        let mut next = (self.current_player_idx + 1) % player_count;

        // Skip players who have folded or are all-in
        while !self.active_players[next] || self.players[next].is_all_in() {
            next = (next + 1) % player_count;

            // If we've gone all the way around, break to avoid infinite loop
            if next == self.current_player_idx {
                break;
            }
        }

        self.current_player_idx = next;
        next
    }

    pub fn post_blinds(&mut self) -> BlindsDto {
        let small_blind_pos = (self.dealer_pos + 1) % self.players.len();
        let big_blind_pos = (self.dealer_pos + 2) % self.players.len();

        let small_blind = 1; // Small blind amount
        let big_blind = 2; // Big blind amount

        // Post small blind
        self.players[small_blind_pos].bet(small_blind);
        self.pot += small_blind;

        // Post big blind
        self.players[big_blind_pos].bet(big_blind);
        self.pot += big_blind;

        self.current_bet = big_blind;
        self.min_raise = big_blind;

        // Start from player after big blind
        self.current_player_idx = (big_blind_pos + 1) % self.players.len();

        BlindsDto {
            small_blind: 1,
            small_blind_owner: self.players[small_blind_pos].get_name().to_string(),
            big_blind: 2,
            big_blind_owner: self.players[big_blind_pos].get_name().to_string(),
        }
    }

    pub fn process_action(&mut self, action: PlayerAction) -> bool {
        let player = &mut self.players[self.current_player_idx];
        let player_bet = player.get_current_bet();

        match action {
            PlayerAction::Fold => {
                self.active_players[self.current_player_idx] = false;
                println!("{} folds", player.get_name());
            }
            PlayerAction::Check => {
                if self.current_bet > player_bet {
                    println!("Cannot check when there's a bet. Must call or raise.");
                    return false;
                }
                println!("{} checks", player.get_name());
            }
            PlayerAction::Call => {
                let call_amount = self.current_bet - player_bet;
                if call_amount > 0 {
                    if !player.bet(call_amount) {
                        println!("Not enough $ to call");
                        return false;
                    }
                    self.pot += call_amount;
                    println!("{} calls {}", player.get_name(), call_amount);
                } else {
                    println!("{} calls", player.get_name());
                }
            }
            PlayerAction::Raise(amount) => {
                if amount < self.min_raise {
                    println!(
                        "Raise must be at least the minimum raise: {} $",
                        self.min_raise
                    );
                    return false;
                }

                let total_bet = self.current_bet + amount;
                let to_call = self.current_bet - player_bet;
                let total_needed = to_call + amount;

                if !player.bet(total_needed) {
                    println!("Not enough $ for this raise");
                    return false;
                }
                self.pot += total_needed;

                self.current_bet = total_bet;
                self.min_raise = amount; // Set new minimum raise
                println!("{} raises to {} $", player.get_name(), total_bet);
            }
            PlayerAction::AllIn => {
                let all_in_amount = player.get_money();
                let _ = player.bet(all_in_amount); // This will put player all-in
                self.pot += all_in_amount;

                // If the all-in amount is greater than current bet, update current bet
                let new_bet = player_bet + all_in_amount;
                if new_bet > self.current_bet {
                    let raise_amount = new_bet - self.current_bet;
                    if raise_amount >= self.min_raise {
                        self.min_raise = raise_amount;
                    }
                    self.current_bet = new_bet;
                }

                println!("{} is ALL IN with {} $", player.get_name(), new_bet);
            }
        }

        true
    }

    async fn notify_everyone(&self, blind_dto: &BlindsDto) {
        for (i, _) in self.players.iter().enumerate() {
            if i == self.current_player_idx {
                continue; // Skip the current player
            }

            let board_dto = self.get_board_dto(blind_dto, i);
            self.messenger_arc
                .send(i, ServerMessageTypes::NextTurn, board_dto)
                .await;
        }
    }

    pub async fn betting_round(&mut self, blind_dto: &BlindsDto) {
        let mut players_acted = 0;
        let player_count = self.players.len() - self.current_player_idx + 1;

        // Continue until all players have had a chance to act and all bets are called
        while players_acted < player_count {
            // Get the current player
            let current_idx = self.current_player_idx;

            // If player is not active (folded), skip
            if !self.active_players[current_idx] {
                self.next_player();
                players_acted += 1;
                continue;
            }

            // If player is all-in, skip
            if self.players[current_idx].is_all_in() {
                self.next_player();
                players_acted += 1;
                continue;
            }

            // Get player action through terminal input
            let board_dto = self.get_board_dto(blind_dto, current_idx);
            self.notify_everyone(&blind_dto).await;
            let action = self.get_player_action(board_dto).await;


            // Process the action
            if self.process_action(action) {
                // If action was valid, move to next player
                self.next_player();
                players_acted += 1;
            }

            // If we've gone all the way around and all bets are matched, break
            let all_bets_called = self.check_all_bets_called();
            if players_acted >= player_count && all_bets_called {
                break;
            }
        }

        // Move all bets to the pot
        self.collect_bets();
    }

    fn check_all_bets_called(&self) -> bool {
        let mut active_players_not_all_in = 0;

        for (idx, active) in self.active_players.iter().enumerate() {
            if *active && !self.players[idx].is_all_in() {
                active_players_not_all_in += 1;
                if self.players[idx].get_current_bet() != self.current_bet {
                    return false;
                }
            }
        }

        // If only one active player remains or all active players have matched bets
        active_players_not_all_in <= 1 || true
    }

    fn collect_bets(&mut self) {
        for player in &mut self.players {
            player.collect_bet();
        }
        self.current_bet = 0;
    }

    fn get_board_dto(&self, blind_dto: &BlindsDto, player_id: usize) -> BoardDto {
        let player = &self.players[player_id];
        BoardDto {
            current_bet: self.current_bet,
            your_bet: player.get_current_bet(),
            your_money: player.get_money(),
            pot: self.pot,
            your_cards: player.get_hand().clone(),
            community_cards: self.community_cards.clone(),
            your_turn: self.current_player_idx == player_id,
            current_player_name: player.get_name().to_string(),
            min_raise: self.min_raise,
            blinds: blind_dto.clone(),
        }
    }

    async fn get_player_action(&self, board_dto: BoardDto) -> PlayerAction {
        let player = &self.players[self.current_player_idx];

        let message = board_dto;

        self.messenger_arc
            .send(
                self.current_player_idx,
                ServerMessageTypes::NextTurn,
                message,
            )
            .await;

        let response = self.messenger_arc.receive(self.current_player_idx).await;
        assert_eq!(response.message_type, ClientMessageTypes::PlayCard);

        let action: PlayerAction = serde_json::from_str(&response.payload_json).unwrap();
        action
    }

    pub fn determine_winners(&self) -> Vec<usize> {
        let mut best_rank = None;
        let mut winners = Vec::new();

        for (idx, active) in self.active_players.iter().enumerate() {
            if *active {
                let player_hand = self.players[idx].get_hand();
                let hand_rank = player_hand.evaluate_best_hand(&self.community_cards);

                match &best_rank {
                    None => {
                        best_rank = Some(hand_rank);
                        winners = vec![idx];
                    }
                    Some(current_best) => {
                        if hand_rank > *current_best {
                            best_rank = Some(hand_rank);
                            winners = vec![idx];
                        } else if hand_rank == *current_best {
                            winners.push(idx);
                        }
                    }
                }
            }
        }

        winners
    }

    pub async fn play_round(&mut self) {
        // Reset for new round
        self.community_cards = Hand::default();
        self.active_players = vec![true; self.players.len()];
        self.pot = 0;
        self.current_bet = 0;
        for player in &mut self.players {
            player.clear_hand();
        }

        // Shuffle deck
        self.shuffle_deck();

        // Pre-flop betting round
        self.game_stage = GameStage::PreFlop;
        let blind_dto = self.post_blinds();
        // Deal hole cards
        self.deal_hole_cards();
        self.betting_round(&blind_dto).await;

        // Check if only one player remains
        if self.count_active_players() <= 1 {
            self.award_pot();
            return;
        }

        // Flop
        self.game_stage = GameStage::Flop;
        self.deal_community_cards(3);
        println!("\nFLOP:\n{}", self.community_cards);
        self.current_player_idx = (self.dealer_pos + 1) % self.players.len();
        self.betting_round(&blind_dto).await;

        // Check if only one player remains
        if self.count_active_players() <= 1 {
            self.award_pot();
            return;
        }

        // Turn
        self.game_stage = GameStage::Turn;
        self.deal_community_cards(1);
        println!("\nTURN:\n{}", self.community_cards);
        self.current_player_idx = (self.dealer_pos + 1) % self.players.len();
        self.betting_round(&blind_dto).await;

        // Check if only one player remains
        if self.count_active_players() <= 1 {
            self.award_pot();
            return;
        }

        // River
        self.game_stage = GameStage::River;
        self.deal_community_cards(1);
        println!("\nRIVER:\n{}", self.community_cards);
        self.current_player_idx = (self.dealer_pos + 1) % self.players.len();
        self.betting_round(&blind_dto).await;

        // Showdown
        self.game_stage = GameStage::Showdown;
        if self.count_active_players() > 1 {
            self.showdown();
        } else {
            self.award_pot();
        }

        // Move dealer button for next round
        self.dealer_pos = (self.dealer_pos + 1) % self.players.len();
    }

    fn count_active_players(&self) -> usize {
        self.active_players.iter().filter(|&&active| active).count()
    }

    fn showdown(&mut self) {
        println!("\n=== SHOWDOWN ===");

        // Show all active players' cards and their hand rankings
        for (idx, active) in self.active_players.iter().enumerate() {
            if *active {
                let player_hand = self.players[idx].get_hand();
                let hand_rank = player_hand.evaluate_best_hand(&self.community_cards);

                println!(
                    "{}'s hand:\n{}\nHand Rank: {:?}",
                    self.players[idx].get_name(),
                    player_hand,
                    hand_rank
                );
            }
        }

        println!("Community cards:\n{}", self.community_cards);

        // Determine winners
        let winners = self.determine_winners();

        // Award pot to winners
        let pot_share = self.pot / winners.len() as u16;
        for &winner_idx in &winners {
            self.players[winner_idx].add_money(pot_share);
            println!(
                "{} wins {} $",
                self.players[winner_idx].get_name(),
                pot_share
            );
        }
    }

    fn award_pot(&mut self) {
        // Find the single active player and award them the pot
        for (idx, active) in self.active_players.iter().enumerate() {
            if *active {
                self.players[idx].add_money(self.pot);
                println!("{} wins {} $", self.players[idx].get_name(), self.pot);
                break;
            }
        }
    }

    pub async fn game_loop(&mut self) {
        println!("\n=== No Limit Texas Hold'em Poker ===");

        let mut round = 1;
        while self.count_players_with_money() > 1 {
            println!("\n=== Round {} ===", round);
            println!("Dealer: {}", self.players[self.dealer_pos].get_name());

            self.play_round().await;

            // Display each player money
            println!("\nMoney");
            for player in &self.players {
                println!("{}: {} $", player.get_name(), player.get_money());
            }

            // Ask to continue
            println!("\nPress Enter to continue to next round or 'q' to quit");
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if input.trim().to_lowercase() == "q" {
                break;
            }

            round += 1;
        }

        // Announce winner
        self.announce_winner();
    }

    fn count_players_with_money(&self) -> usize {
        self.players.iter().filter(|p| p.get_money() > 0).count()
    }

    fn announce_winner(&self) {
        let mut winner_idx = 0;
        let mut max_money = 0;

        // Find the player with the most money
        for (idx, player) in self.players.iter().enumerate() {
            let money = player.get_money();
            if money > max_money {
                max_money = money;
                winner_idx = idx;
            }
        }

        // Check if we found a winner
        if max_money > 0 {
            println!(
                "\n🏆 {} is the winner with {} $! 🏆",
                self.players[winner_idx].get_name(),
                max_money
            );
        } else {
            println!("\nNo winner found - everyone is out of $!");
        }

        println!("Game Over. Thanks for playing!");
    }
}
