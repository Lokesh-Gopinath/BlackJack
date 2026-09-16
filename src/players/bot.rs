//! The bot players and their ("average") AI decisions.

use crate::models::card::{Card, Rank};
use crate::players::player::{Player, PlayerAction, PlayerType, PLAYER_START_COINS};

/// An AI participant, wrapping [`Player`] state with bot decisions.
#[derive(Debug)]
pub struct BotPlayer {
    /// Shared player state.
    pub player: Player,
}

impl BotPlayer {
    /// Creates a bot named `Bot {id}` with the starting coins.
    pub fn new(id: usize) -> Self {
        BotPlayer {
            player: Player::new(&format!("Bot {id}"), PlayerType::Bot, PLAYER_START_COINS),
        }
    }

    /// Matches the human's bet. A bot short on coins goes all-in; a broke
    /// bot sits the round out (returns `None`).
    pub fn match_bet(&mut self, human_bet: u32) -> Option<u32> {
        if self.player.coins == 0 {
            return None;
        }
        let amount = human_bet.min(self.player.coins);
        match self.player.place_bet(amount) {
            Ok(()) => Some(amount),
            Err(_) => None,
        }
    }

    /// Whether the bot buys insurance. The average AI knows insurance is a
    /// losing bet, so bots always decline.
    pub fn wants_insurance(&self) -> bool {
        false
    }

    /// Picks the next action ("average" AI, roughly basic strategy):
    ///
    /// - surrenders hard 16 against a dealer 9/10/A,
    /// - stands on any hard 17+ and any soft 18+,
    /// - hits 11 and below, and soft hands below 18,
    /// - otherwise (hard 12–16) hits only against a strong dealer upcard
    ///   (7 through ace).
    pub fn decide_action(&self, dealer_upcard: Card) -> PlayerAction {
        if self.player.hand.is_blackjack() {
            return PlayerAction::Stand;
        }
        let value = self.player.hand.get_value();

        if value == 16
            && !self.player.hand.is_soft()
            && matches!(
                dealer_upcard.rank,
                Rank::Nine | Rank::Ten | Rank::Jack | Rank::Queen | Rank::King | Rank::Ace
            )
        {
            return PlayerAction::Surrender;
        }
        if self.player.hand.is_soft() {
            return if value >= 18 {
                PlayerAction::Stand
            } else {
                PlayerAction::Hit
            };
        }
        if value >= 17 {
            return PlayerAction::Stand;
        }
        if value <= 11 {
            return PlayerAction::Hit;
        }
        if dealer_upcard.value() >= 7 {
            PlayerAction::Hit
        } else {
            PlayerAction::Stand
        }
    }
}
