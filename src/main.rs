//! Binary entry point: greets the player and runs rounds until game over.

use std::io::{self, Write};

use blackjack::game::play_round;
use blackjack::game::state::BlackjackGame;
use blackjack::utils::display;

fn main() {
    display::print_banner();

    print!("Enter your name (blank for \"Player\"): ");
    let _ = io::stdout().flush();
    let input = display::read_line().unwrap_or_default();
    let trimmed = input.trim();
    let name = if trimmed.is_empty() { "Player" } else { trimmed };

    let mut game = BlackjackGame::new(name);

    display::print_header(&format!("WELCOME, {}", name.to_uppercase()));
    display::print_game_state(&game);
    println!();
    println!(" Bets: 10 / 50 / 100 coins — the bots always match your bet.");
    println!(" Type q at the bet, insurance, or turn prompt to quit with your winnings.");
    println!(" The game ends when you can no longer afford the minimum bet (10 coins).");
    println!(" If the dealer runs out of coins, it respawns with 5× your current coins.");
    println!(" Your bankroll carries over — only winning hands that wipe out the dealer earn prestige.");

    while !game.is_game_over() && !game.is_quit() {
        play_round(&mut game);
    }

    display::print_final_results(&game);
}
