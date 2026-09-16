//! The shared [`Player`] state used by the human, bots, and dealer.

use crate::models::hand::Hand;

/// Coins a human or bot player starts with.
pub const PLAYER_START_COINS: u32 = 100;
/// Coins the dealer starts with (5× the player's fresh coins).
pub const DEALER_START_COINS: u32 = 500;
/// The smallest allowed bet; a player below this cannot keep playing.
pub const MIN_BET: u32 = 10;
/// The bet amounts the CLI accepts.
pub const BET_OPTIONS: [u32; 3] = [10, 50, 100];

/// Which kind of participant a [`Player`] represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerType {
    /// The human at the keyboard.
    Human,
    /// An AI opponent.
    Bot,
    /// The house dealer.
    Dealer,
}

/// A decision a player can make on their turn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerAction {
    /// Take another card.
    Hit,
    /// Stop taking cards.
    Stand,
    /// Forfeit the hand for half the bet back (first decision only).
    Surrender,
}

/// Shared state and bookkeeping for one participant of the table.
#[derive(Debug)]
pub struct Player {
    /// Display name ("You", "Bot 1", "Dealer", ...).
    pub name: String,
    /// What kind of participant this is.
    pub player_type: PlayerType,
    /// Current hand.
    pub hand: Hand,
    /// Coins left.
    pub coins: u32,
    /// Coins wagered on the current hand (already deducted from the coins).
    pub bet: u32,
    /// Insurance stake for the current hand (half the bet; already deducted).
    pub insurance_bet: u32,
    /// Whether the player gave this hand up.
    pub has_surrendered: bool,
    /// Win/loss score: +1 per win, −1 per loss, never below 0.
    pub points: u32,
}

impl Player {
    /// Creates a player with a fresh, empty hand.
    pub fn new(name: &str, player_type: PlayerType, coins: u32) -> Self {
        Player {
            name: name.to_string(),
            player_type,
            hand: Hand::new(),
            coins,
            bet: 0,
            insurance_bet: 0,
            has_surrendered: false,
            points: 0,
        }
    }

    /// Wagers `amount`, deducting it from the coins.
    pub fn place_bet(&mut self, amount: u32) -> Result<(), String> {
        if amount == 0 {
            return Err("the bet must be greater than zero".to_string());
        }
        if amount > self.coins {
            return Err(format!("not enough coins: needs {amount}, has {}", self.coins));
        }
        self.bet = amount;
        self.coins -= amount;
        Ok(())
    }

    /// Buys insurance for half the current bet (only offered on a dealer ace).
    pub fn buy_insurance(&mut self) -> Result<(), String> {
        if self.bet == 0 {
            return Err("insurance requires a placed bet".to_string());
        }
        let cost = self.bet / 2;
        if self.coins < cost {
            return Err(format!("not enough coins for insurance (needs {cost})"));
        }
        self.coins -= cost;
        self.insurance_bet = cost;
        Ok(())
    }

    /// Wins the hand at even money: stake back plus the dealer's match.
    /// Counts as +1 point.
    pub fn win(&mut self) {
        self.coins += self.bet * 2;
        self.add_point();
    }

    /// Wins with a natural blackjack, paid 3:2. Counts as +1 point.
    pub fn win_blackjack(&mut self) {
        self.coins += self.bet + self.bet * 3 / 2;
        self.add_point();
    }

    /// Cashes out a winning insurance bet, paid 2:1 (stake back plus twice
    /// the stake), and clears the insurance stake.
    pub fn win_insurance(&mut self) {
        self.coins += self.insurance_bet * 3;
        self.insurance_bet = 0;
    }

    /// Forfeits a losing insurance bet (the stake was already deducted).
    pub fn lose_insurance(&mut self) {
        self.insurance_bet = 0;
    }

    /// Ties the hand: the stake is returned and no points change.
    pub fn push(&mut self) {
        self.coins += self.bet;
    }

    /// Surrenders the hand: half the stake back and a lost point.
    pub fn surrender(&mut self) {
        self.has_surrendered = true;
        self.coins += self.bet / 2;
        self.subtract_point();
    }

    /// Loses the hand (the stake is already deducted). Counts as −1 point.
    pub fn lose(&mut self) {
        self.subtract_point();
    }

    /// Adds one point.
    pub fn add_point(&mut self) {
        self.points += 1;
    }

    /// Removes one point, never dropping below zero.
    pub fn subtract_point(&mut self) {
        self.points = self.points.saturating_sub(1);
    }

    /// Sets the points back to zero.
    pub fn reset_points(&mut self) {
        self.points = 0;
    }

    /// Replaces the player's coins (used by the game reset logic).
    pub fn reset_coins(&mut self, coins: u32) {
        self.coins = coins;
    }

    /// Clears the hand and all per-round state (bet, insurance, surrender).
    pub fn clear_hand(&mut self) {
        self.hand.clear();
        self.bet = 0;
        self.insurance_bet = 0;
        self.has_surrendered = false;
    }

    /// Whether an insurance stake is riding on this hand.
    pub fn has_insurance(&self) -> bool {
        self.insurance_bet > 0
    }

    /// A human-readable description of the hand, e.g. `A♠, K♥ = 21`.
    pub fn get_hand_description(&self) -> String {
        format!("{}", self.hand)
    }
}
