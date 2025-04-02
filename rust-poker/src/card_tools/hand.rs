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
