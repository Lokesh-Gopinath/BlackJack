//! Card, [`Rank`], and [`Suit`] types.

use std::fmt;

/// The four suits of a standard 52-card deck.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    /// Hearts, shown as ♥.
    Hearts,
    /// Diamonds, shown as ♦.
    Diamonds,
    /// Clubs, shown as ♣.
    Clubs,
    /// Spades, shown as ♠.
    Spades,
}

impl Suit {
    /// All suits in iteration order.
    pub const ALL: [Suit; 4] = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        };
        write!(f, "{symbol}")
    }
}

/// The thirteen ranks of a standard 52-card deck.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rank {
    /// The two.
    Two,
    /// The three.
    Three,
    /// The four.
    Four,
    /// The five.
    Five,
    /// The six.
    Six,
    /// The seven.
    Seven,
    /// The eight.
    Eight,
    /// The nine.
    Nine,
    /// The ten.
    Ten,
    /// The jack, worth 10.
    Jack,
    /// The queen, worth 10.
    Queen,
    /// The king, worth 10.
    King,
    /// The ace, worth 11 or 1 (soft handling happens in the hand).
    Ace,
}

impl Rank {
    /// All ranks in ascending order.
    pub const ALL: [Rank; 13] = [
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
        Rank::Ace,
    ];

    /// The base blackjack value of the rank. Aces count as 11; a hand
    /// downgrades aces to 1 as needed (see [`crate::models::hand::Hand`]).
    pub fn value(self) -> u32 {
        match self {
            Rank::Ace => 11,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
        }
    }

    /// Whether the rank is an ace.
    pub fn is_ace(self) -> bool {
        self == Rank::Ace
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        };
        write!(f, "{label}")
    }
}

/// A single playing card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    /// Rank of the card.
    pub rank: Rank,
    /// Suit of the card.
    pub suit: Suit,
}

impl Card {
    /// Creates a new card with the given rank and suit.
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Card { rank, suit }
    }

    /// Whether the card is an ace.
    pub fn is_ace(&self) -> bool {
        self.rank.is_ace()
    }

    /// The base blackjack value of the card.
    pub fn value(&self) -> u32 {
        self.rank.value()
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}
