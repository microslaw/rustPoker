mod datatypes;
use datatypes::*;

fn main() {
    let mut deck = getSortedDeck();

    println!("{}", deck);

    for _ in 0..10{
        let card = deck.pop_random();
        println!("{}", card);
    }
}
