mod model;

use model::card::Card;

fn main() {
    let card1 = Card::new("Ace".to_string(), "Spades".to_string());
    card1.card_name(); 


    println!("rank value of card1: {}", card1.get_rank_val());
}