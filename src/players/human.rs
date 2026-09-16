//! The human player and its CLI decision prompts.

use std::io::{self, Write};

use crate::players::player::{
    Player, PlayerAction, PlayerType, BET_OPTIONS, MIN_BET, PLAYER_START_COINS,
};
use crate::utils::display;

/// The human participant, wrapping [`Player`] state with stdin prompts.
#[derive(Debug)]
pub struct HumanPlayer {
    /// Shared player state.
    pub player: Player,
}

impl HumanPlayer {
    /// Creates the human player with the starting coins.
    pub fn new(name: &str) -> Self {
        HumanPlayer {
            player: Player::new(name, PlayerType::Human, PLAYER_START_COINS),
        }
    }

    /// Prompts until the player picks an affordable bet (10/50/100).
    /// Entering `q` quits the session (returns `None`). On end of input,
    /// bets the smallest affordable option.
    pub fn choose_bet(&self) -> Option<u32> {
        loop {
            print!(
                "Place your bet — {} (coins: {}) or [q]uit: ",
                options_text(),
                self.player.coins
            );
            let _ = io::stdout().flush();

            let input = match display::read_line() {
                Some(line) => line,
                None => return Some(self.first_affordable_bet()),
            };
            if matches!(input.trim().to_lowercase().as_str(), "q" | "quit") {
                return None;
            }
            let amount = match input.trim().parse::<u32>() {
                Ok(amount) if BET_OPTIONS.contains(&amount) => amount,
                _ => {
                    println!("  Please enter one of: {}, or q to quit.", options_text());
                    continue;
                }
            };
            if amount > self.player.coins {
                println!("  You only have {} coins.", self.player.coins);
                continue;
            }
            return Some(amount);
        }
    }

    /// Asks whether the player buys insurance when the dealer shows an ace.
    /// Entering `q` quits the session (returns `None`). On end of input,
    /// declines.
    pub fn wants_insurance(&self) -> Option<bool> {
        let cost = self.player.bet / 2;
        loop {
            print!(
                "Dealer shows an Ace. Buy insurance for {cost} coins (pays 2:1), or [q]uit? [y/n/q]: "
            );
            let _ = io::stdout().flush();
            let input = match display::read_line() {
                Some(line) => line,
                None => return Some(false),
            };
            match input.trim().to_lowercase().as_str() {
                "y" | "yes" => return Some(true),
                "n" | "no" => return Some(false),
                "q" | "quit" => return None,
                _ => println!("  Please answer y, n, or q."),
            }
        }
    }

    /// Prompts for the next action. Surrender is only offered (and only
    /// accepted) as the first decision of the hand; entering `q` quits the
    /// session (returns `None`). On end of input, stands.
    pub fn choose_action(&self, first_decision: bool) -> Option<PlayerAction> {
        loop {
            let options = if first_decision {
                "[h]it, [s]tand, [r] surrender, [q]uit"
            } else {
                "[h]it, [s]tand, [q]uit"
            };
            print!(
                "{}, you have {} — {} : ",
                self.player.name,
                self.player.get_hand_description(),
                options
            );
            let _ = io::stdout().flush();

            let input = match display::read_line() {
                Some(line) => line,
                None => return Some(PlayerAction::Stand),
            };
            match input.trim().to_lowercase().as_str() {
                "h" | "hit" => return Some(PlayerAction::Hit),
                "s" | "stand" => return Some(PlayerAction::Stand),
                "r" | "surrender" if first_decision => return Some(PlayerAction::Surrender),
                "r" | "surrender" => {
                    println!("  Surrender is only allowed before taking a card.");
                }
                "q" | "quit" => return None,
                _ => {
                    let extra = if first_decision { ", r, or q" } else { ", or q" };
                    println!("  Please enter h, s{extra}.");
                }
            }
        }
    }

    /// The smallest bet option the player can currently afford.
    fn first_affordable_bet(&self) -> u32 {
        BET_OPTIONS
            .iter()
            .copied()
            .find(|option| *option <= self.player.coins)
            .unwrap_or(MIN_BET)
    }
}

/// Formats the accepted bet options, e.g. `10/50/100`.
fn options_text() -> String {
    BET_OPTIONS
        .iter()
        .map(|option| option.to_string())
        .collect::<Vec<_>>()
        .join("/")
}
