mod model;

use std::io::{self, Write};

use model::card::Card;
use model::deck::Deck;

fn main() {
    initialise_game();
}

fn initialise_game() {
    print_banner();
    loop {
        print_menu();
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("failed to read input");
        let choice = choice.trim();

        match choice {
            "1" => {start_game();},
            "2" => {print_rules();},
            "3" => {
                println!("\nthanks for playing make15 :)");
                return;
            },
            _ => {println!("\n invalid input, please enter a number between 1-3!");}
        }
    }
}

fn start_game() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    let deck_num = input.trim().parse::<i32>()
        .expect("Please enter a valid number");

    let mut deck = Deck::new(deck_num);
    deck.show_deckcards();
}

fn print_rules() {

}

fn print_banner() {
    println!("                                                             __            .----------. ");
    println!("                                                        ...-'  |`.        /          /  ");
    println!(" __  __   ___                 .           __.....__     |      |  |      /   ______.'   ");
    println!("|  |/  `.'   `.             .'|       .-''         '.   ....   |  |     /   /_          ");
    println!("|   .-.  .-.   '          .'  |      /     .-''\"'-.  `.   -|   |  |    /      '''--.    ");
    println!("|  |  |  |  |  |    __   <    |     /     /________\\   \\   |   |  |   '___          `.  ");
    println!("|  |  |  |  |  | .:--.'.  |   | ____|                  |...'   `--'       `'.         | ");
    println!("|  |  |  |  |  |/ |   \\ | |   | \\ .\\    .-------------'|         |`.        )        | ");
    println!("|  |  |  |  |  |`\" __ | | |   |/  .  \\    '-.____...---.` --------\\ |......-'        /  ");
    println!("|__|  |__|  |__| .'.''| | |    /\\  \\  `.             .'  `---------' \\          _..'`   ");
    println!("                / /   | |_|   |  \\  \\   `''-...... -'                 '------'''        ");
    println!("                \\ \\._,\\ '/'    \\  \\  \\                                                   ");
    println!("                 `--'  `\"'------'  '---'                                                 \n");
}

fn print_menu() {
    print!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    println!("1. Start Game");
    println!("2. How To Play");
    println!("3. View Leaderboard");
    println!("4. Quit Game");
    print!("-> ");
}