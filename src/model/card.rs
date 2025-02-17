pub struct Card {
    rank: String,
    suit: String,
}

impl Card {
    pub fn new (rank: String, suit: String) -> Self {
        Self { rank, suit }
    }

    pub fn card_name(&self) {
        println!("{} of {}", self.rank, self.suit);
    }


    pub fn get_rank_val(&self) -> i32 {
        match self.rank.as_str() {
            "Jack" | "Queen" | "King" => 11,
            "Ace" => 12,
            _ => self.rank.parse().unwrap_or(0),
        }
    }
}

