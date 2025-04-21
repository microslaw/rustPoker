use rand::Rng;
use std::fmt;

use crate::card_tools::card::*;
use crate::card_tools::rank::*;
use crate::card_tools::color::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HandRank {
    HighCard(u32),
    OnePair(u32),
    TwoPair(u32, u32),
    ThreeOfAKind(u32),
    Straight(u32),
    Flush(u32),
    FullHouse(u32, u32),
    FourOfAKind(u32),
    StraightFlush(u32),
}

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

    pub fn evaluate_best_hand(&self, community_cards: &Hand) -> HandRank {
        let mut all_cards = self.cards.clone();
        all_cards.extend(community_cards.cards.iter().cloned());

        all_cards.sort_by(|a, b| b.rank.cmp(&a.rank));

        if let Some(rank) = Self::is_straight_flush(&all_cards) {
            return HandRank::StraightFlush(rank);
        }
        if let Some(rank) = Self::is_four_of_a_kind(&all_cards) {
            return HandRank::FourOfAKind(rank);
        }
        if let Some((three, pair)) = Self::is_full_house(&all_cards) {
            return HandRank::FullHouse(three, pair);
        }
        if let Some(rank) = Self::is_flush(&all_cards) {
            return HandRank::Flush(rank);
        }
        if let Some(rank) = Self::is_straight(&all_cards) {
            return HandRank::Straight(rank);
        }
        if let Some(rank) = Self::is_three_of_a_kind(&all_cards) {
            return HandRank::ThreeOfAKind(rank);
        }
        if let Some((high, low)) = Self::is_two_pair(&all_cards) {
            return HandRank::TwoPair(high, low);
        }
        if let Some(rank) = Self::is_one_pair(&all_cards) {
            return HandRank::OnePair(rank);
        }

        HandRank::HighCard(all_cards[0].rank.to_value())
    }

    fn is_straight_flush(cards: &[Card]) -> Option<u32> {
        if let Some(rank) = Self::is_straight(cards) {
            if Self::is_flush(cards).is_some() {
                return Some(rank);
            }
        }
        None
    }

    fn is_four_of_a_kind(cards: &[Card]) -> Option<u32> {
        for rank in Rank::iterator() {
            if cards.iter().filter(|c| c.rank == *rank).count() == 4 {
                return Some(rank.to_value());
            }
        }
        None
    }

    fn is_full_house(cards: &[Card]) -> Option<(u32, u32)> {
        if let Some(three) = Self::is_three_of_a_kind(cards) {
            if let Some(pair) = Self::is_one_pair(cards) {
                return Some((three, pair));
            }
        }
        None
    }

    fn is_flush(cards: &[Card]) -> Option<u32> {
        for color in Color::iterator() {
            let flush_cards: Vec<&Card> = cards.iter().filter(|c| c.color == *color).collect();
            if flush_cards.len() >= 5 {
                return Some(flush_cards[0].rank.to_value());
            }
        }
        None
    }

    fn is_straight(cards: &[Card]) -> Option<u32> {
        let mut unique_ranks: Vec<u32> = cards.iter().map(|c| c.rank.to_value()).collect();
        unique_ranks.sort_unstable();
        unique_ranks.dedup();

        for window in unique_ranks.windows(5) {
            if window[4] - window[0] == 4 {
                return Some(window[4]);
            }
        }

        // Special case: A-2-3-4-5 straight
        if unique_ranks.contains(&14) && unique_ranks.windows(4).any(|w| w == [2, 3, 4, 5]) {
            return Some(5);
        }

        None
    }

    fn is_three_of_a_kind(cards: &[Card]) -> Option<u32> {
        for rank in Rank::iterator() {
            if cards.iter().filter(|c| c.rank == *rank).count() == 3 {
                return Some(rank.to_value());
            }
        }
        None
    }

    fn is_two_pair(cards: &[Card]) -> Option<(u32, u32)> {
        let mut pairs = Vec::new();
        for rank in Rank::iterator() {
            if cards.iter().filter(|c| c.rank == *rank).count() == 2 {
                pairs.push(rank.to_value());
            }
        }

        if pairs.len() >= 2 {
            pairs.sort_unstable();
            return Some((pairs[pairs.len() - 1], pairs[pairs.len() - 2]));
        }
        None
    }

    fn is_one_pair(cards: &[Card]) -> Option<u32> {
        for rank in Rank::iterator() {
            if cards.iter().filter(|c| c.rank == *rank).count() == 2 {
                return Some(rank.to_value());
            }
        }
        None
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
