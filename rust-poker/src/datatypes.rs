use std::fmt;

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub enum Color {
    Spades,
    Hearts,
    Club,
    Diamond,
}


impl fmt::Display for Color {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let txt = match self {
            Color::Spades => "♠".to_string(),
            Color::Hearts => "♥".to_string(),
            Color::Club => "♣".to_string(),
            Color::Diamond => "♦".to_string(),
        };
        write!(f, "{}", txt)
    }
}


#[derive(PartialEq, PartialOrd, Eq, Ord)]
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
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
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
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}{}]", self.rank, self.color)
    }

}
