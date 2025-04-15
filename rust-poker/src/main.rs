mod card_tools;
mod game_types;

fn main() {
    let mut deck = card_tools::hand::get_sorted_deck();

    println!("{}", deck);

    for _ in 0..47{
        let card = deck.pop_random();
        println!("{}", card);
    }

    println!("{}", deck);
}
