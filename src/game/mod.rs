//! Game flow: persistent state, rules, and round execution.

pub mod round;
pub mod rules;
pub mod state;

pub use round::play_round;
pub use rules::RoundOutcome;
pub use state::BlackjackGame;
