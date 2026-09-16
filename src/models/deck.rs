//! The [`Deck`] type: building, shuffling, and drawing cards.

use rand::seq::SliceRandom;
use rand::thread_rng;

use super::card::{Card, Rank, Suit};

/// Number of cards in a standard deck.
pub const DECK_SIZE: usize = 52;

/// A standard 52-card deck.
#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// Creates a fully shuffled deck.
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(DECK_SIZE);
        for suit in Suit::ALL {
            for rank in Rank::ALL {
                cards.push(Card::new(rank, suit));
            }
        }
        let mut deck = Deck { cards };
        deck.shuffle();
        deck
    }

    /// Shuffles the remaining cards into a random order.
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    /// Removes and returns the top card, or `None` if the deck is empty.
    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /// Removes and returns the top card, reshuffling a fresh deck when the
    /// current one runs out. Use this when play must always continue.
    pub fn draw_card(&mut self) -> Card {
        match self.cards.pop() {
            Some(card) => card,
            None => {
                *self = Deck::new();
                self.cards.pop().expect("a fresh deck is never empty")
            }
        }
    }

    /// Number of cards left in the deck.
    pub fn remaining(&self) -> usize {
        self.cards.len()
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}
