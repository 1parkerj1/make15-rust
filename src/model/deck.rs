use crate::Card;
use rand::seq::SliceRandom;
use rand::rng;

pub struct Deck {
    deck_cards: Vec<Card>,
}

impl Deck {

    // construct the deck_cards vec
    pub fn new(decknum: i32) -> Self {
        let mut deck = Self {
            deck_cards: Self::create_deck(decknum),
        };
        deck.shuffle();
        deck
    }

    pub fn shuffle(&mut self) {
        let mut rng = rng();
        self.deck_cards.shuffle(&mut rng);
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.deck_cards.pop()
    }

    pub fn create_deck(i32: decknum) -> Vec<Card> {

        

    }
}



