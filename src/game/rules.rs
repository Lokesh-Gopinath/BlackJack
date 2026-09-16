//! Blackjack rules: hand comparison and insurance resolution.

use std::cmp::Ordering;

use crate::models::hand::Hand;

/// How a settled hand turned out for the player.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundOutcome {
    /// Player natural blackjack (dealer does not have one) — pays 3:2.
    PlayerBlackjack,
    /// Player total beats the dealer's (or the dealer busted).
    PlayerWin,
    /// Equal totals — the stake is returned.
    Push,
    /// Player total loses (including a bust).
    DealerWin,
}

/// Compares a finished player hand against the dealer's hand.
///
/// A player bust loses immediately; a natural blackjack beats any
/// non-blackjack dealer hand; a dealer bust beats any live player hand;
/// otherwise the higher total wins and equal totals push.
pub fn determine_winner(player_hand: &Hand, dealer_hand: &Hand) -> RoundOutcome {
    if player_hand.is_busted() {
        return RoundOutcome::DealerWin;
    }
    if player_hand.is_blackjack() && !dealer_hand.is_blackjack() {
        return RoundOutcome::PlayerBlackjack;
    }
    if dealer_hand.is_blackjack() {
        return RoundOutcome::DealerWin;
    }
    if dealer_hand.is_busted() {
        return RoundOutcome::PlayerWin;
    }
    match player_hand.get_value().cmp(&dealer_hand.get_value()) {
        Ordering::Greater => RoundOutcome::PlayerWin,
        Ordering::Equal => RoundOutcome::Push,
        Ordering::Less => RoundOutcome::DealerWin,
    }
}

/// Whether the hand is a natural blackjack.
pub fn check_player_blackjack(hand: &Hand) -> bool {
    hand.is_blackjack()
}

/// Whether the dealer was dealt a natural blackjack.
pub fn check_dealer_blackjack(hand: &Hand) -> bool {
    hand.is_blackjack()
}

/// Whether outstanding insurance bets pay out (only on a dealer blackjack).
pub fn check_insurance_payout(dealer_hand: &Hand) -> bool {
    dealer_hand.is_blackjack()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::card::{Card, Rank, Suit};
    use crate::models::hand::Hand;

    fn card(rank: Rank) -> Card {
        Card::new(rank, Suit::Hearts)
    }

    fn hand_of(ranks: &[Rank]) -> Hand {
        let mut hand = Hand::new();
        for rank in ranks {
            hand.add_card(card(*rank));
        }
        hand
    }

    #[test]
    fn player_bust_loses_even_vs_dealer_bust() {
        let player = hand_of(&[Rank::Ten, Rank::Ten, Rank::Five]);
        let dealer = hand_of(&[Rank::Ten, Rank::Ten, Rank::Ten]);
        assert_eq!(determine_winner(&player, &dealer), RoundOutcome::DealerWin);
    }

    #[test]
    fn natural_beats_plain_twenty_one() {
        let player = hand_of(&[Rank::Ace, Rank::King]);
        let dealer = hand_of(&[Rank::Seven, Rank::Seven, Rank::Seven]);
        assert_eq!(
            determine_winner(&player, &dealer),
            RoundOutcome::PlayerBlackjack
        );
    }

    #[test]
    fn dealer_bust_wins_and_equal_totals_push() {
        let player = hand_of(&[Rank::Ten, Rank::Nine]);
        let dealer = hand_of(&[Rank::Ten, Rank::Ten, Rank::Five]);
        assert_eq!(determine_winner(&player, &dealer), RoundOutcome::PlayerWin);
        let tie = hand_of(&[Rank::Ten, Rank::Nine]);
        assert_eq!(determine_winner(&player, &tie), RoundOutcome::Push);
    }

    #[test]
    fn insurance_pays_only_on_dealer_blackjack() {
        let natural = hand_of(&[Rank::Ace, Rank::Ten]);
        let plain = hand_of(&[Rank::Ten, Rank::Nine]);
        assert!(check_insurance_payout(&natural));
        assert!(!check_insurance_payout(&plain));
        assert!(check_dealer_blackjack(&natural));
        assert!(check_player_blackjack(&natural));
    }
}
