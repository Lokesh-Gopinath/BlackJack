# Rust CLI Blackjack Game

A fully-featured Blackjack game built in Rust with a CLI interface.

## Features

- Single deck (52 cards), reshuffled when it runs low
- Multiplayer: You + 2 Bot Players vs Dealer
- Starting coins: Players = 100, Dealer = 500
- Betting options: 10, 50, 100 coins
- Bots automatically match your bet (all-in if short on coins)
- Insurance and Surrender options
- Average AI for bots (basic-strategy style decisions)
- Point system: Win = +1, Lose = -1 (min 0)
- Game ends when you can no longer afford the minimum bet (10 coins)
- Auto-reset with the dealer getting 5x your coins when the dealer runs out
- Quit any time: type `q` at the bet, insurance, or turn prompt — the session
  ends and shows your net winnings (coins kept vs. coins granted)

## Rules

- Dealer hits on soft 17
- Blackjack pays 3:2
- Insurance pays 2:1 (when dealer shows an Ace; costs half your bet)
- Surrender (first decision only) returns half your bet
- Standard Blackjack rules apply otherwise

## Installation

```bash
git clone https://github.com/yourusername/blackjack.git
cd blackjack
cargo build
```

## Running

```bash
cargo run
```

## Project Structure

```
blackjack/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs          # Entry point, CLI handling
    ├── lib.rs           # Module exports
    ├── models/
    │   ├── mod.rs       # Model module exports
    │   ├── card.rs      # Card, Rank, Suit types
    │   ├── deck.rs      # Deck struct and methods
    │   └── hand.rs      # Hand struct and methods
    ├── players/
    │   ├── mod.rs       # Player module exports
    │   ├── player.rs    # Player struct, types, and point/bet bookkeeping
    │   ├── human.rs     # HumanPlayer (CLI prompts)
    │   ├── bot.rs       # BotPlayer (average AI)
    │   └── dealer.rs    # Dealer (house turn logic)
    ├── game/
    │   ├── mod.rs       # Game module exports
    │   ├── state.rs     # Game state management and reset logic
    │   ├── rules.rs     # Blackjack rules logic
    │   └── round.rs     # Round execution logic
    └── utils/
        ├── mod.rs       # Utility exports
        └── display.rs   # Formatting and display helpers
```

## Notes

- Card suits are printed as Unicode symbols (♥ ♦ ♣ ♠). If your console shows
  garbled characters, switch it to UTF-8 (e.g. `chcp 65001` on Windows).

## License

MIT
