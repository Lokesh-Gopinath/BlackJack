//! The dealer and the house turn logic.

use crate::models::card::Card;
use crate::models::deck::Deck;
use crate::players::player::{Player, PlayerType, DEALER_START_COINS};

/// The house dealer, wrapping [`Player`] state.
#[derive(Debug)]
pub struct Dealer {
    /// Shared player state.
    pub player: Player,
}

impl Dealer {
    /// Creates the dealer with the house starting coins.
    pub fn new() -> Self {
        Dealer {
            player: Player::new("Dealer", PlayerType::Dealer, DEALER_START_COINS),
        }
    }

    /// The dealer's face-up card (the first card dealt).
    pub fn upcard(&self) -> Option<Card> {
        self.player.hand.cards().first().copied()
    }

    /// Plays the dealer's turn: hits while the total is under 17 or the hand
    /// is a soft 17 (house rule), standing on hard 17 or more.
    ///
    /// Returns the cards drawn during the turn so the caller can display them.
    pub fn play_turn(&mut self, deck: &mut Deck) -> Vec<Card> {
        let mut drawn = Vec::new();
        loop {
            let value = self.player.hand.get_value();
            let hits = value < 17 || (value == 17 && self.player.hand.is_soft());
            if !hits {
                break;
            }
            let card = deck.draw_card();
            self.player.hand.add_card(card);
            drawn.push(card);
        }
        drawn
    }
}

impl Default for Dealer {
    fn default() -> Self {
        Self::new()
    }
}
