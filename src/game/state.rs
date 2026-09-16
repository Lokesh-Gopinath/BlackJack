//! Central game state: table composition, deck, and reset logic.

use crate::models::deck::Deck;
use crate::players::bot::BotPlayer;
use crate::players::dealer::Dealer;
use crate::players::human::HumanPlayer;
use crate::players::player::MIN_BET;

/// Reshuffle a fresh deck when fewer cards than this remain.
pub const RESHUFFLE_THRESHOLD: usize = 15;
/// How many bots sit at the table.
pub const BOT_COUNT: usize = 2;
/// When the dealer goes bankrupt it is replenished with this multiple of
/// the human's carried-over coins (e.g. 325 coins → 1625).
pub const DEALER_RESET_MULTIPLIER: u32 = 5;

/// Everything that persists across the rounds of a blackjack session.
#[derive(Debug)]
pub struct BlackjackGame {
    /// The deck cards are drawn from (reshuffled when it runs low).
    pub deck: Deck,
    /// The house dealer.
    pub dealer: Dealer,
    /// The human player.
    pub human: HumanPlayer,
    /// The bot players.
    pub bots: Vec<BotPlayer>,
    /// Number of rounds played in this session.
    pub round_number: u32,
    /// Net coins won by the human this session relative to the initial
    /// 100-coin bankroll (bankrolls persist across dealer resets, so no
    /// coins are ever granted after the session starts).
    pub winnings: i64,
    /// Set when the human quits the session early.
    pub quit_requested: bool,
}

impl BlackjackGame {
    /// Sets up a fresh table: a shuffled deck, a dealer with house coins,
    /// and one human plus [`BOT_COUNT`] bots with starting coins.
    pub fn new(human_name: &str) -> Self {
        BlackjackGame {
            deck: Deck::new(),
            dealer: Dealer::new(),
            human: HumanPlayer::new(human_name),
            bots: (1..=BOT_COUNT).map(BotPlayer::new).collect(),
            round_number: 0,
            winnings: 0,
            quit_requested: false,
        }
    }

    /// Whether the deck has dipped below the reshuffle threshold.
    pub fn needs_reshuffle(&self) -> bool {
        self.deck.remaining() < RESHUFFLE_THRESHOLD
    }

    /// Replaces the deck with a freshly shuffled one.
    pub fn reshuffle_deck(&mut self) {
        self.deck = Deck::new();
    }

    /// Whether the human can no longer afford the minimum bet (game over).
    pub fn is_game_over(&self) -> bool {
        self.human.player.coins < MIN_BET
    }

    /// Marks the session as quit by the human.
    pub fn request_quit(&mut self) {
        self.quit_requested = true;
    }

    /// Whether the human quit the session early.
    pub fn is_quit(&self) -> bool {
        self.quit_requested
    }

    /// Whether the dealer can no longer cover the minimum bet.
    pub fn dealer_is_broke(&self) -> bool {
        self.dealer.player.coins < MIN_BET
    }

    /// Refills the dealer's bankroll after it goes bankrupt. Player
    /// bankrolls and prestige persist untouched; the dealer receives
    /// 5× the human's carried-over coins.
    ///
    /// Never called while the human is broke: the game-over check runs
    /// first, so the dealer always respawns against a funded player.
    pub fn reset_game(&mut self) {
        let dealer_coins = self.human.player.coins * DEALER_RESET_MULTIPLIER;
        self.dealer.player.reset_coins(dealer_coins);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dealer_reset_scales_with_carried_over_bankrolls() {
        let mut game = BlackjackGame::new("Tester");
        game.human.player.coins = 325;
        game.bots[0].player.coins = 40;
        game.bots[1].player.coins = 0;
        game.human.player.add_prestige();
        game.dealer.player.coins = 0; // dealer just went bankrupt

        game.reset_game();

        // Player bankrolls carry over untouched...
        assert_eq!(game.human.player.coins, 325);
        assert_eq!(game.bots[0].player.coins, 40);
        assert_eq!(game.bots[1].player.coins, 0);
        // ...and prestige persists...
        assert_eq!(game.human.player.prestige, 1);
        // ...while the dealer respawns with 5× the human's carried-over coins.
        assert_eq!(game.dealer.player.coins, 325 * DEALER_RESET_MULTIPLIER);
    }
}
