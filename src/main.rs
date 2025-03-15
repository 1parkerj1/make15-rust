mod model;

use model::card::Card;
use model::deck::Deck;

fn main() {
    let mut deck = Deck::new(1);
    deck.show_topcard();
}