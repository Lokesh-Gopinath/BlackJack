//! # blackjack
//!
//! A fully-featured CLI Blackjack game: you and two bot players against the
//! dealer, with betting (10/50/100), insurance, surrender, a point system,
//! and an automatic table reset when the dealer runs out of coins.
//!
//! The binary entry point lives in `src/main.rs`; this crate holds all of
//! the game logic organized into four public modules:
//!
//! - [`models`] — cards, the deck, and hands
//! - [`players`] — shared player state plus human, bot, and dealer
//! - [`game`] — persistent state, rules, and round execution
//! - [`utils`] — display and formatting helpers

pub mod game;
pub mod models;
pub mod players;
pub mod utils;
