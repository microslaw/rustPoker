use rand::Rng;
use std::fmt;

use crate::card_tools::card::*;
use crate::card_tools::rank::*;
use crate::card_tools::color::*;


#[derive(Default)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn concat(&self, other: &Hand) -> Hand {
        let mut new_hand: Hand = Hand::default();
        new_hand.cards.extend(self.cards.iter().cloned());
        new_hand.cards.extend(other.cards.iter().cloned());
        return new_hand;
    }

    pub fn pop_random(&mut self) -> Card {

        let num = rand::thread_rng().gen_range(0..self.cards.len());
        let poped: Card = self.cards[num].clone();
        self.cards.remove(1);

        return poped;
    }

    pub fn pop_card(&mut self) -> Option<Card> {
        if self.cards.is_empty() {
            None
        } else {
            Some(self.cards.remove(0))
        }
    }

    pub fn swap_cards(&mut self, i: usize, j: usize) {
        if i < self.cards.len() && j < self.cards.len() {
            self.cards.swap(i, j);
        }
    }
    
    pub fn len(&self) -> usize {
        self.cards.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
    
    pub fn get_cards(&self) -> &Vec<Card> {
        &self.cards
    }
    
    // This method will be used for hand evaluation
    pub fn evaluate(&self) -> u32 {
        // In a complete implementation, this would determine hand rank
        // (straight flush, four of a kind, etc.)
        // For now, let's return a simple score
        let mut score = 0;
        for card in &self.cards {
            score += match card.rank {
                Rank::Two => 2,
                Rank::Three => 3,
                Rank::Four => 4,
                Rank::Five => 5,
                Rank::Six => 6,
                Rank::Seven => 7,
                Rank::Eight => 8,
                Rank::Nine => 9,
                Rank::Ten => 10,
                Rank::Jack => 11,
                Rank::Queen => 12,
                Rank::King => 13,
                Rank::Ace => 14,
            };
        }
        score
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        _ = write!(f, "----");
        for (i, card) in self.cards.iter().enumerate() {
            if i % 5 == 0 {
                _ = write!(f, "\n");
            }
            _ = write!(f, "{}", card);
        }
        write!(f, "\n----\n")
    }
}

pub fn get_sorted_deck() -> Hand {
    let mut deck: Hand = Hand::default();

    for color in Color::iterator() {
        for rank in Rank::iterator() {
            deck.add_card(Card {
                rank: rank.clone(),
                color: color.clone(),
            });
        }
    }

    return deck;
}
