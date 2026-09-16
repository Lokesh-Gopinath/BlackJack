//! The [`Hand`] type and blackjack value logic.

use std::fmt;

use super::card::Card;

/// The winning hand value; totals above this are busts.
pub const BLACKJACK_VALUE: u32 = 21;

/// A blackjack hand: an ordered collection of cards.
#[derive(Debug, Clone, Default)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    /// Creates an empty hand.
    pub fn new() -> Self {
        Hand { cards: Vec::new() }
    }

    /// Adds a card to the hand.
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// The cards currently held.
    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    /// Number of cards in the hand.
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Whether the hand holds no cards.
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Removes all cards from the hand.
    pub fn clear(&mut self) {
        self.cards.clear();
    }

    /// Computes the best blackjack total, downgrading aces from 11 to 1
    /// while the total would bust.
    pub fn get_value(&self) -> u32 {
        let mut total: u32 = self.cards.iter().map(Card::value).sum();
        let mut aces_left = aces(&self.cards) as u32;
        while total > BLACKJACK_VALUE && aces_left > 0 {
            total -= 10;
            aces_left -= 1;
        }
        total
    }

    /// Whether at least one ace still counts as 11 (a "soft" hand).
    pub fn is_soft(&self) -> bool {
        let mut total: u32 = self.cards.iter().map(Card::value).sum();
        let mut aces_left = aces(&self.cards) as u32;
        while total > BLACKJACK_VALUE && aces_left > 0 {
            total -= 10;
            aces_left -= 1;
        }
        aces_left > 0
    }

    /// Whether the hand is a natural blackjack (exactly 2 cards worth 21).
    pub fn is_blackjack(&self) -> bool {
        self.len() == 2 && self.get_value() == BLACKJACK_VALUE
    }

    /// Whether the hand's total exceeds 21.
    pub fn is_busted(&self) -> bool {
        self.get_value() > BLACKJACK_VALUE
    }
}

/// Counts the aces in a slice of cards.
fn aces(cards: &[Card]) -> usize {
    cards.iter().filter(|card| card.is_ace()).count()
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.cards.is_empty() {
            return write!(f, "(no cards)");
        }
        let cards = self
            .cards
            .iter()
            .map(|card| card.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{cards} = {}", self.get_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::card::{Card, Rank, Suit};

    fn card(rank: Rank) -> Card {
        Card::new(rank, Suit::Spades)
    }

    #[test]
    fn ace_downgrades_to_avoid_bust() {
        let mut hand = Hand::new();
        hand.add_card(card(Rank::Ace));
        hand.add_card(card(Rank::Nine));
        assert_eq!(hand.get_value(), 20);
        assert!(hand.is_soft());
        hand.add_card(card(Rank::Eight));
        assert_eq!(hand.get_value(), 18);
        assert!(!hand.is_soft());
    }

    #[test]
    fn natural_blackjack_requires_two_cards() {
        let mut hand = Hand::new();
        hand.add_card(card(Rank::Ace));
        hand.add_card(card(Rank::King));
        assert!(hand.is_blackjack());
        hand.add_card(card(Rank::Ten)); // 21 with three cards is not a natural
        assert_eq!(hand.get_value(), BLACKJACK_VALUE);
        assert!(!hand.is_blackjack());
    }

    #[test]
    fn busted_hands_are_detected() {
        let mut hand = Hand::new();
        hand.add_card(card(Rank::Ten));
        hand.add_card(card(Rank::Ten));
        assert!(!hand.is_busted());
        hand.add_card(card(Rank::Five));
        assert!(hand.is_busted());
        assert_eq!(hand.get_value(), 25);
    }
}
