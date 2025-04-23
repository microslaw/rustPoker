use crate::card_tools::rank::*;
use crate::card_tools::color::*;
use std::fmt;

#[derive(Clone)]
pub struct Card {
    pub rank: Rank,
    pub color: Color,
}

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
