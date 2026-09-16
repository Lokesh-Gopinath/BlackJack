//! Round execution: betting, dealing, player turns, and settlement.

use crate::game::rules::{self, RoundOutcome};
use crate::game::state::BlackjackGame;
use crate::models::card::Card;
use crate::models::hand::Hand;
use crate::players::player::{Player, PlayerAction};
use crate::utils::display;

/// Runs one complete round: bets, deal, insurance, turns, and settlement.
/// An early quit voids the round, refunds the bets, and ends the session.
pub fn play_round(game: &mut BlackjackGame) {
    game.round_number += 1;
    display::print_round_header(game);

    if game.needs_reshuffle() {
        display::print_reshuffle();
        game.reshuffle_deck();
    }

    let coins_before = game.human.player.coins;
    if !take_bets(game) {
        game.round_number = game.round_number.saturating_sub(1);
        finish_round(game, coins_before);
        return;
    }
    deal_initial_cards(game);
    display::print_initial_deal(game);

    let dealer_upcard = game
        .dealer
        .upcard()
        .expect("the dealer is dealt before turns begin");

    maybe_offer_insurance(game, dealer_upcard);
    if game.is_quit() {
        void_round_on_quit(game, coins_before);
        return;
    }

    if rules::check_dealer_blackjack(&game.dealer.player.hand) {
        settle_dealer_blackjack(game);
        finish_round(game, coins_before);
        return;
    }

    display::print_no_dealer_blackjack();

    play_human_turn(game);
    if game.is_quit() {
        void_round_on_quit(game, coins_before);
        return;
    }
    for index in 0..game.bots.len() {
        play_bot_turn(game, index);
    }

    display::print_dealer_reveal(&game.dealer.player);
    if dealer_must_play(game) {
        let drawn = game.dealer.play_turn(&mut game.deck);
        display::print_dealer_turn(&game.dealer.player, &drawn);
    }

    settle_round(game);
    finish_round(game, coins_before);
}

/// Collects the human's bet, then has the bots match it. Returns `false`
/// when the human quits instead of betting.
fn take_bets(game: &mut BlackjackGame) -> bool {
    let human_bet = match game.human.choose_bet() {
        Some(bet) => bet,
        None => {
            game.request_quit();
            return false;
        }
    };
    game.human
        .player
        .place_bet(human_bet)
        .expect("the prompt only offers affordable bets");
    display::print_bet(&game.human.player);

    for bot in &mut game.bots {
        match bot.match_bet(human_bet) {
            Some(_) => display::print_bet(&bot.player),
            None => display::print_sits_out(&bot.player),
        }
    }
    println!();
    true
}

/// Deals two cards to every player with a bet, then two to the dealer.
fn deal_initial_cards(game: &mut BlackjackGame) {
    for _ in 0..2 {
        game.human.player.hand.add_card(game.deck.draw_card());
        for bot in &mut game.bots {
            if bot.player.bet > 0 {
                bot.player.hand.add_card(game.deck.draw_card());
            }
        }
        game.dealer.player.hand.add_card(game.deck.draw_card());
    }
}

/// Offers insurance to the human when the dealer shows an ace; bots decline.
/// An early quit here voids the hand.
fn maybe_offer_insurance(game: &mut BlackjackGame, dealer_upcard: Card) {
    if !dealer_upcard.is_ace() {
        return;
    }
    display::print_dealer_shows_ace();

    let cost = game.human.player.bet / 2;
    if game.human.player.coins < cost {
        display::print_insurance_unaffordable(cost);
    } else {
        match game.human.wants_insurance() {
            Some(true) => {
                game.human
                    .player
                    .buy_insurance()
                    .expect("affordability was checked above");
                display::print_insurance_bought(&game.human.player);
            }
            Some(false) => display::print_insurance_declined(&game.human.player),
            None => game.request_quit(),
        }
    }
    display::print_bots_decline_insurance();
}

/// Runs the human's turn: hit / stand / surrender (first decision only).
fn play_human_turn(game: &mut BlackjackGame) {
    if game.human.player.hand.is_blackjack() {
        display::print_blackjack(&game.human.player);
        return;
    }
    let mut first_decision = true;
    loop {
        if game.human.player.hand.get_value() == 21 {
            display::print_stands_on_21(&game.human.player);
            return;
        }
        match game.human.choose_action(first_decision) {
            Some(PlayerAction::Hit) => {
                let card = game.deck.draw_card();
                game.human.player.hand.add_card(card);
                display::print_player_draws(&game.human.player, &card);
                if game.human.player.hand.is_busted() {
                    display::print_bust(&game.human.player);
                    return;
                }
            }
            Some(PlayerAction::Stand) => {
                display::print_stands(&game.human.player);
                return;
            }
            Some(PlayerAction::Surrender) => {
                game.human.player.has_surrendered = true;
                display::print_surrender(&game.human.player);
                return;
            }
            None => {
                game.request_quit();
                return;
            }
        }
        first_decision = false;
    }
}

/// Runs one bot's turn with its AI decisions.
fn play_bot_turn(game: &mut BlackjackGame, index: usize) {
    {
        let bot = &game.bots[index];
        if bot.player.bet == 0 {
            return; // sat out the round
        }
        if bot.player.hand.is_blackjack() {
            display::print_blackjack(&bot.player);
            return;
        }
    }

    let dealer_upcard = game
        .dealer
        .upcard()
        .expect("the dealer is dealt before turns begin");

    loop {
        if game.bots[index].player.hand.get_value() == 21 {
            display::print_stands_on_21(&game.bots[index].player);
            return;
        }
        match game.bots[index].decide_action(dealer_upcard) {
            PlayerAction::Hit => {
                let card = game.deck.draw_card();
                game.bots[index].player.hand.add_card(card);
                display::print_player_draws(&game.bots[index].player, &card);
                if game.bots[index].player.hand.is_busted() {
                    display::print_bust(&game.bots[index].player);
                    return;
                }
            }
            PlayerAction::Stand => {
                display::print_stands(&game.bots[index].player);
                return;
            }
            PlayerAction::Surrender => {
                game.bots[index].player.has_surrendered = true;
                display::print_surrender(&game.bots[index].player);
                return;
            }
        }
    }
}

/// Whether the hand can still beat the dealer (not busted/surrendered/natural).
fn is_live(player: &Player) -> bool {
    !player.has_surrendered && !player.hand.is_busted() && !player.hand.is_blackjack()
}

/// Whether the dealer must take a turn.
fn dealer_must_play(game: &BlackjackGame) -> bool {
    is_live(&game.human.player) || game.bots.iter().any(|bot| is_live(&bot.player))
}

/// Resolves the round when the dealer peeked a natural blackjack:
/// insurance pays 2:1, naturals push, everything else loses.
fn settle_dealer_blackjack(game: &mut BlackjackGame) {
    display::print_dealer_blackjack(&game.dealer.player);

    let dealer_hand = game.dealer.player.hand.clone();
    let mut dealer_delta: i64 = 0;

    // The human may hold insurance; bots never buy it.
    if game.human.player.has_insurance() {
        let stake = game.human.player.insurance_bet;
        if rules::check_insurance_payout(&dealer_hand) {
            let payout = stake * 3;
            game.human.player.win_insurance();
            dealer_delta -= 2 * stake as i64;
            display::print_insurance_win(&game.human.player, payout);
        } else {
            game.human.player.lose_insurance();
            dealer_delta += stake as i64;
            display::print_insurance_loss();
        }
    }

    dealer_delta += settle_against_dealer_blackjack(&mut game.human.player);
    for bot in &mut game.bots {
        dealer_delta += settle_against_dealer_blackjack(&mut bot.player);
    }

    apply_dealer_delta(game, dealer_delta);
    println!();
    display::print_game_state(game);
}

/// One player's outcome when the dealer has blackjack (naturals push).
/// Returns the dealer's coin delta (positive = the dealer collects).
fn settle_against_dealer_blackjack(player: &mut Player) -> i64 {
    if player.bet == 0 {
        return 0;
    }
    if player.hand.is_blackjack() {
        player.push();
        display::print_push_with(player, "against the dealer's blackjack");
        0
    } else {
        let bet = player.bet;
        display::print_loss_with(player, "to the dealer's blackjack");
        bet as i64
    }
}

/// Pays out every hand against the dealer's final hand. If the round drains
/// the dealer's bankroll below the minimum bet, every player with a winning
/// hand earns +1 prestige.
fn settle_round(game: &mut BlackjackGame) {
    display::print_results_header();

    let dealer_hand = game.dealer.player.hand.clone();
    let (human_delta, human_won) = settle_hand(&mut game.human.player, &dealer_hand);
    let mut dealer_delta = human_delta;

    // Any unclaimed insurance stake (dealer has no blackjack) goes to the house.
    if game.human.player.has_insurance() {
        display::print_insurance_loss();
        dealer_delta += game.human.player.insurance_bet as i64;
        game.human.player.lose_insurance();
    }

    let mut bot_won = Vec::with_capacity(game.bots.len());
    for bot in &mut game.bots {
        let (delta, won) = settle_hand(&mut bot.player, &dealer_hand);
        dealer_delta += delta;
        bot_won.push(won);
    }

    apply_dealer_delta(game, dealer_delta);

    // The hands that drained the dealer's last coins earn prestige.
    if game.dealer_is_broke() {
        let mut winners: Vec<String> = Vec::new();
        if human_won {
            game.human.player.add_prestige();
            winners.push(game.human.player.name.clone());
        }
        for (bot, won) in game.bots.iter_mut().zip(bot_won) {
            if won {
                bot.player.add_prestige();
                winners.push(bot.player.name.clone());
            }
        }
        display::print_prestige_award(&winners);
    }

    println!();
    display::print_game_state(game);
}

/// Resolves one player's hand and returns the dealer's coin delta
/// (negative = the dealer pays, positive = the dealer collects) together
/// with whether the hand won. Winning hands earn prestige if the dealer
/// goes bankrupt this round.
fn settle_hand(player: &mut Player, dealer_hand: &Hand) -> (i64, bool) {
    if player.bet == 0 {
        return (0, false);
    }
    if player.has_surrendered {
        let bet = player.bet;
        player.surrender();
        display::print_surrender_result(player, bet);
        return ((bet / 2) as i64, false);
    }

    let bet = player.bet;
    match rules::determine_winner(&player.hand, dealer_hand) {
        RoundOutcome::PlayerBlackjack => {
            player.win_blackjack();
            display::print_blackjack_win(player);
            (-(bet as i64 * 3 / 2), true)
        }
        RoundOutcome::PlayerWin => {
            player.win();
            display::print_win(player);
            (-(bet as i64), true)
        }
        RoundOutcome::Push => {
            player.push();
            display::print_push(player);
            (0, false)
        }
        RoundOutcome::DealerWin => {
            display::print_loss(player);
            (bet as i64, false)
        }
    }
}

/// Applies the round's net coin flow to the dealer, clamped at zero
/// (a dry dealer triggers the reset check before the next round).
fn apply_dealer_delta(game: &mut BlackjackGame, delta: i64) {
    let dealer = &mut game.dealer.player;
    let coins = dealer.coins as i64 + delta;
    dealer.coins = coins.max(0) as u32;
}

/// Voids the hand in progress: every wagered coin (bets and insurance)
/// goes back to its owner. The dealer has not collected or paid anything
/// before settlement, so it needs no refund.
fn refund_bets(game: &mut BlackjackGame) {
    let human = &mut game.human.player;
    human.coins += human.bet + human.insurance_bet;
    for bot in &mut game.bots {
        bot.player.coins += bot.player.bet;
    }
}

/// Voids the hand in progress after an early quit: prints the notice,
/// refunds every wager, and unwinds the round count.
fn void_round_on_quit(game: &mut BlackjackGame, coins_before: u32) {
    display::print_quit_bets_refunded();
    refund_bets(game);
    game.round_number = game.round_number.saturating_sub(1);
    finish_round(game, coins_before);
}

/// Cleans up hands, banks the round's net winnings, handles the dealer
/// reset, and pauses for the player.
fn finish_round(game: &mut BlackjackGame, coins_before: u32) {
    game.human.player.clear_hand();
    for bot in &mut game.bots {
        bot.player.clear_hand();
    }
    game.dealer.player.clear_hand();

    game.winnings += game.human.player.coins as i64 - coins_before as i64;

    if !game.is_game_over() && game.dealer_is_broke() {
        game.reset_game();
        display::print_dealer_reset(game);
    }

    display::pause();
}



