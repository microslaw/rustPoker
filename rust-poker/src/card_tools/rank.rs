use std::fmt;
use std::slice::Iter;

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
        static RANK: [Rank; Rank::LEN] = [
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
        RANK.iter()
    }
}
