use rand::Rng;
use std::slice::Iter;
use std::{fmt, usize};

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum Color {
    Spades,
    Hearts,
    Club,
    Diamond,
}
impl Color {
    pub const LEN: usize = 4;
    pub fn iterator() -> Iter<'static, Color> {
        static COLOR: [Color; Color::LEN] =
            [Color::Spades, Color::Hearts, Color::Club, Color::Diamond];
        COLOR.iter()
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let txt = match self {
            Color::Spades => "♠".to_string(),
            Color::Hearts => "♥".to_string(),
            Color::Club => "♣".to_string(),
            Color::Diamond => "♦".to_string(),
        };
        write!(f, "{}", txt)
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let txt = match self {
            Rank::Two => "2".to_string(),
            Rank::Three => "3".to_string(),
            Rank::Four => "4".to_string(),
            Rank::Five => "5".to_string(),
            Rank::Six => "6".to_string(),
            Rank::Seven => "7".to_string(),
            Rank::Eight => "8".to_string(),
            Rank::Nine => "9".to_string(),
            Rank::Ten => "0".to_string(),
            Rank::Jack => "J".to_string(),
            Rank::Queen => "Q".to_string(),
            Rank::King => "K".to_string(),
            Rank::Ace => "A".to_string(),
        };
        write!(f, "{}", txt)
    }
}

impl Rank {
    pub const LEN: usize = 13;

    pub fn iterator() -> Iter<'static, Rank> {
        static COLOR: [Rank; Rank::LEN] = [
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ];
        COLOR.iter()
    }
}

#[derive(Clone)]
pub struct Card {
    pub rank: Rank,
    color: Color,
}

pub static C1: Card = Card {
    color: Color::Hearts,
    rank: Rank::Eight,
};
pub static C2: Card = Card {
    color: Color::Hearts,
    rank: Rank::Seven,
};

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        (&self.rank, &self.color).eq(&(&other.rank, &other.color))
    }
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (&self.rank, &self.color).partial_cmp(&(&other.rank, &other.color))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (&self.rank, &self.color).cmp(&(&other.rank, &other.color))
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}{}]", self.rank, self.color)
    }
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
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "----");
        for (i, card) in self.cards.iter().enumerate() {
            if i % 5 == 0 {
                write!(f, "\n");
            }
            write!(f, "{}", card);
        }
        write!(f, "\n----\n")
    }
}

pub fn getSortedDeck() -> Hand {
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
