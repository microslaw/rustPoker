use crate::card_tools::hand::Hand;
use crate::game_types::player::Player;

struct Board {
    pub players: Vec<Player>,
    pub center_deck: Hand,
    pub drawing_deck: Hand,
    pub turn: usize,
}

impl Board {
    fn next_turn(&mut self) {
        self.turn = (self.turn + 1) % self.players.len();
    }

    fn add_player(&mut self, player: Player){
        self.players.push(player);
    }

    fn draw_card_to_center(){
        // TODO
    }

    fn game_loop(){
        // TODO
    }
}
