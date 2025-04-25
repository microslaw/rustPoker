use std::fmt;
use std::slice::Iter;
use serde::{Serialize, Deserialize};


#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Serialize, Deserialize)]
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
