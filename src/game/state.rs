//! Central game state: table composition, deck, and reset logic.

use crate::models::deck::Deck;
use crate::players::bot::BotPlayer;
use crate::players::dealer::Dealer;
use crate::players::human::HumanPlayer;
use crate::players::player::{MIN_BET, PLAYER_START_COINS};

/// Reshuffle a fresh deck when fewer cards than this remain.
pub const RESHUFFLE_THRESHOLD: usize = 15;
/// How many bots sit at the table.
pub const BOT_COUNT: usize = 2;
/// On a dealer reset the dealer receives this multiple of the player's
/// fresh coins (5 × 100 = 500).
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
    /// Net coins won by the human this session: every round's coin delta
    /// plus reset top-ups, so it always equals coins kept minus coins granted.
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

    /// Resets the table after the dealer runs dry: everyone returns to
    /// their starting coins, the dealer receives 5× the player's fresh
    /// coins, and the scoreboard is cleared.
    pub fn reset_game(&mut self) {
        // The coin top-up counts toward the session's net winnings.
        self.winnings += PLAYER_START_COINS as i64 - self.human.player.coins as i64;
        self.human.player.reset_coins(PLAYER_START_COINS);
        self.human.player.reset_points();
        self.dealer.player.reset_coins(PLAYER_START_COINS * DEALER_RESET_MULTIPLIER);
        self.dealer.player.reset_points();
        for bot in &mut self.bots {
            bot.player.reset_coins(PLAYER_START_COINS);
            bot.player.reset_points();
        }
    }
}
