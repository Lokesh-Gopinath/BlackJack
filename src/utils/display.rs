//! Formatting and display helpers for the CLI.

use std::io::{self, Write};

use crate::game::state::BlackjackGame;
use crate::models::card::Card;
use crate::players::player::Player;

/// Reads one line from stdin; `None` means end of input (EOF).
pub fn read_line() -> Option<String> {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(0) => None,
        Ok(_) => Some(buffer),
        Err(_) => None,
    }
}

/// Waits for the player to press Enter.
pub fn pause() {
    print!("Press Enter to continue...");
    let _ = io::stdout().flush();
    let _ = read_line();
}

/// Prints the welcome banner.
pub fn print_banner() {
    println!();
    println!("╔══════════════════════════════════╗");
    println!("║        ♠  B L A C K J A C K  ♥   ║");
    println!("╚══════════════════════════════════╝");
    println!(" Dealer hits soft 17 | Blackjack pays 3:2 | Insurance pays 2:1");
    println!();
}

/// Prints a full-width separator line.
pub fn print_separator() {
    println!("{}", "─".repeat(50));
}

/// Prints a titled section header.
pub fn print_header(title: &str) {
    print_separator();
    println!(" {title}");
    print_separator();
}

/// Prints the round number and the current table stats.
pub fn print_round_header(game: &BlackjackGame) {
    println!();
    print_header(&format!("ROUND {}", game.round_number));
    print_game_state(game);
    println!();
}

/// Prints every participant's coins and prestige.
pub fn print_game_state(game: &BlackjackGame) {
    println!(" {:<14}{:>7}{:>8}", "Player", "Coins", "Prestige");
    print_stats(&game.human.player);
    for bot in &game.bots {
        print_stats(&bot.player);
    }
    print_stats(&game.dealer.player);
}

/// Prints one participant's coins and prestige row.
fn print_stats(player: &Player) {
    println!(" {:<14}{:>7}{:>8}", player.name, player.coins, player.prestige);
}

/// Announces a placed bet.
pub fn print_bet(player: &Player) {
    println!(" {} bets {} coins.", player.name, player.bet);
}

/// Announces that a broke bot sits out.
pub fn print_sits_out(player: &Player) {
    println!(" {} is out of coins and sits out.", player.name);
}

/// Prints the table right after the deal (dealer hole card hidden).
pub fn print_initial_deal(game: &BlackjackGame) {
    println!();
    println!(" {} has: {}", game.human.player.name, game.human.player.hand);
    for bot in &game.bots {
        if bot.player.bet > 0 {
            println!(" {} has: {}", bot.player.name, bot.player.hand);
        }
    }
    match game.dealer.upcard() {
        Some(upcard) => println!(" Dealer shows: {upcard} + [hidden card]"),
        None => println!(" Dealer has no cards."),
    }
    println!();
}

/// Notes the dealer is showing an ace.
pub fn print_dealer_shows_ace() {
    println!(" The dealer shows an Ace!");
}

/// Notes insurance is unaffordable.
pub fn print_insurance_unaffordable(cost: u32) {
    println!(" You cannot afford insurance ({cost} coins).");
}

/// Confirms an insurance purchase.
pub fn print_insurance_bought(player: &Player) {
    println!(
        " {} buys insurance for {} coins.",
        player.name, player.insurance_bet
    );
}

/// Notes the human declined insurance.
pub fn print_insurance_declined(player: &Player) {
    println!(" {} declines insurance.", player.name);
}

/// Notes that bots never insure.
pub fn print_bots_decline_insurance() {
    println!(" The bots decline insurance.");
}

/// Announces an insurance payout (2:1).
pub fn print_insurance_win(player: &Player, payout: u32) {
    println!(
        " ✓ Insurance pays out! {} recovers {} coins.",
        player.name, payout
    );
}

/// Announces a lost insurance stake.
pub fn print_insurance_loss() {
    println!(" ✗ The insurance stake is lost.");
}

/// Announces that quitting voided the hand and refunded the bets.
pub fn print_quit_bets_refunded() {
    println!();
    println!(" ✋ Quit — the hand is voided and all bets are refunded.");
}

/// Announces the dealer peek did NOT find blackjack.
pub fn print_no_dealer_blackjack() {
    println!(" The dealer does not have blackjack.");
    println!();
}

/// Announces the dealer's natural blackjack.
pub fn print_dealer_blackjack(player: &Player) {
    println!();
    println!(" ☠ Dealer has BLACKJACK: {}", player.hand);
}

/// Reveals the dealer's full hand.
pub fn print_dealer_reveal(player: &Player) {
    println!(" Dealer reveals: {}", player.hand);
}

/// Prints the dealer's completed turn (drawn cards, final total).
pub fn print_dealer_turn(player: &Player, drawn: &[Card]) {
    if drawn.is_empty() {
        println!(" Dealer stands on {}.", player.hand.get_value());
    } else {
        for card in drawn {
            println!(" Dealer draws: {card}");
        }
    }
    println!(" Dealer finishes with: {}", player.hand);
    if player.hand.is_busted() {
        println!(" ☠ Dealer busts!");
    }
    println!();
}

/// A player drew a card.
pub fn print_player_draws(player: &Player, card: &Card) {
    println!(" {} draws {card} → {}", player.name, player.hand);
}

/// A player reached a natural blackjack.
pub fn print_blackjack(player: &Player) {
    println!(" ★ {} has BLACKJACK!", player.name);
}

/// A player busted.
pub fn print_bust(player: &Player) {
    println!(
        " ✗ {} BUSTS with {}.",
        player.name,
        player.hand.get_value()
    );
}

/// A player stands automatically on 21.
pub fn print_stands_on_21(player: &Player) {
    println!(" {} stands on 21.", player.name);
}

/// A player chose to stand.
pub fn print_stands(player: &Player) {
    println!(" {} stands on {}.", player.name, player.hand.get_value());
}

/// A player announced surrender.
pub fn print_surrender(player: &Player) {
    println!(" {} surrenders.", player.name);
}

/// Opens the results section of a round.
pub fn print_results_header() {
    println!();
    print_header("RESULTS");
}

/// Announces the prestige earned for helping bankrupt the dealer.
pub fn print_prestige_award(names: &[String]) {
    if names.is_empty() {
        return;
    }
    println!(" ★ The dealer is wiped out! Prestige +1 for: {}.", names.join(", "));
}

/// A player wins the hand at even money.
pub fn print_win(player: &Player) {
    println!(
        " ✓ {} wins {} coins ({}).",
        player.name, player.bet, player.hand
    );
}

/// A player wins with a natural blackjack (3:2).
pub fn print_blackjack_win(player: &Player) {
    println!(
        " ★ {} BLACKJACK wins {} coins!",
        player.name,
        player.bet * 3 / 2
    );
}

/// A player ties.
pub fn print_push(player: &Player) {
    println!(" = {} pushes — bet returned ({}).", player.name, player.hand);
}

/// A player ties, with a custom reason.
pub fn print_push_with(player: &Player, reason: &str) {
    println!(" = {} pushes — bet returned ({}).", player.name, reason);
}

/// A player loses, bust-aware.
pub fn print_loss(player: &Player) {
    if player.hand.is_busted() {
        println!(" ✗ {} busts and loses {} coins.", player.name, player.bet);
    } else {
        println!(
            " ✗ {} loses {} coins ({}).",
            player.name, player.bet, player.hand
        );
    }
}

/// A player loses, with a custom reason.
pub fn print_loss_with(player: &Player, reason: &str) {
    println!(
        " ✗ {} loses {} coins ({}).",
        player.name, player.bet, reason
    );
}

/// A player surrenders — half the bet back.
pub fn print_surrender_result(player: &Player, bet: u32) {
    println!(
        " ✗ {} surrendered — recovers {} coins.",
        player.name,
        bet / 2
    );
}

/// Announces the deck was refreshed.
pub fn print_reshuffle() {
    println!(" Deck running low — reshuffling a fresh deck...");
}

/// Announces the dealer reset and shows the fresh table.
pub fn print_dealer_reset(game: &BlackjackGame) {
    println!();
    print_header("DEALER OUT OF COINS");
    println!(
        " Auto-reset: the dealer receives 5× your coins, everyone returns to their starting coins, and points reset."
    );
    print_game_state(game);
    println!();
}

/// Prints the session-ending summary, including the net winnings.
pub fn print_final_results(game: &BlackjackGame) {
    println!();
    print_header("GAME OVER");
    if game.is_quit() {
        println!(
            " {} quit the session after {} round(s).",
            game.human.player.name, game.round_number
        );
    } else {
        println!(
            " {} ran out of coins after {} round(s).",
            game.human.player.name, game.round_number
        );
    }
    println!();
    println!(
        " You leave with {} coins — net winnings: {}.",
        game.human.player.coins,
        format_winnings(game.winnings)
    );
    println!();
    print_game_state(game);
    println!();
    println!(" Thanks for playing! ♠ ♥ ♦ ♣");
    println!();
}

/// Formats a signed coin amount, e.g. `+30` or `-60`.
fn format_winnings(winnings: i64) -> String {
    if winnings >= 0 {
        format!("+{winnings}")
    } else {
        format!("{winnings}")
    }
}

