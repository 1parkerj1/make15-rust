use super::card::Rank;
use super::card::Suit;
use crate::Card;

use rand::rng;
use rand::seq::SliceRandom;

use strum::IntoEnumIterator;

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

    pub fn create_deck(decknum: i32) -> Vec<Card> {
        let mut i = 0;
        let mut deck = Vec::with_capacity((52 * decknum) as usize);
        while i < decknum {
            for suit in Suit::iter() {
                for rank in Rank::iter() {
                    deck.push(Card::new(rank, suit));
                }
            }
            i += 1;
        }
        return deck;
    }

    pub fn show_deckcards(&self) {
        println!("Deck contains {} cards: ", self.deck_cards.len());
        for card in &self.deck_cards {
            card.to_string();
        }
    }

    pub fn show_topcard(&self) {
        if let Some(top_card) = &self.deck_cards.last() {
            print!("top card: ");
            top_card.to_string();
        } else {
            println!("the deck is empty!");
        }
    }
}
