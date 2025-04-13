use crate::card_tools::card::Card;
use crate::card_tools::hand::Hand;

pub struct Player {
    pub hand: Hand,
    money: u16,
    current_bet: u16,
    name: String,
    all_in: bool,
}

impl Player {
    pub fn new(name: String, money: u16) -> Self {
        Player {
            hand: Hand::default(),
            money,
            current_bet: 0,
            name,
            all_in: false,
        }
    }
    
    pub fn add_card(&mut self, card: Card) {
        self.hand.add_card(card);
    }
    
    pub fn get_hand(&self) -> &Hand {
        &self.hand
    }
    
    pub fn get_name(&self) -> &str {
        &self.name
    }
    
    pub fn get_money(&self) -> u16 {
        self.money
    }
    
    pub fn get_current_bet(&self) -> u16 {
        self.current_bet
    }
    
    pub fn is_all_in(&self) -> bool {
        self.all_in
    }
    
    pub fn add_money(&mut self, amount: u16) {
        self.money += amount;
    }
    
    pub fn bet(&mut self, amount: u16) -> bool {
        if amount > self.money {
            return false;
        }
        
        self.money -= amount;
        self.current_bet += amount;
        
        if self.money == 0 {
            self.all_in = true;
        }
        
        true
    }
    
    pub fn collect_bet(&mut self) -> u16 {
        let bet = self.current_bet;
        self.current_bet = 0;
        bet
    }
    
    pub fn clear_hand(&mut self) {
        self.hand = Hand::default();
        self.all_in = false;
    }
}