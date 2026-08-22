//! Show-me solver mode: the game plays itself, strategy by strategy,
//! applying each annotation's effect before moving to the next. When
//! the taught techniques run out, the trial fallback keeps it going.

use super::Game;
use suduko_tutor::Effect;

/// Default extra ticks between auto-advances (2 => one step per 3s).
const DEFAULT_DELAY_TICKS: u32 = 2;

/// Starts show-me mode: open the panel on the current board and
/// select the first (cheapest) strategy.
pub fn start(game: &mut Game) {
    game.show_me = true;
    game.show_me_auto = true;
    game.show_me_delay_ticks = DEFAULT_DELAY_TICKS;
    game.show_me_wait = 0;
    let cands = suduko_tutor::candidates_with(&game.shown_values(), &game.eliminated);
    game.teaching.refresh(&cands);
    game.teaching.panel_open = true;
    game.teaching.select(0);
}

/// Toggles show-me mode.
pub fn toggle(game: &mut Game) {
    if game.show_me {
        stop(game);
    } else {
        start(game);
    }
}

/// Stops show-me mode and clears solver state (the board survives).
pub fn stop(game: &mut Game) {
    game.show_me = false;
    game.show_me_auto = false;
    // The elimination layer survives: it is the user's pencil-note
    // state too (Reset is the explicit clear).
    game.teaching.close();
}

/// Advances the walkthrough; applying at the last step of a strategy
/// moves on to the next strategy automatically.
pub fn advance(game: &mut Game) {
    let Some(annotation) = game.teaching.current().cloned() else {
        return;
    };
    if game.teaching.step_index + 1 < annotation.steps.len() {
        game.teaching.step_by(1);
    } else {
        apply(game);
    }
}

/// One walkthrough beat: step forward, or apply and continue when a
/// show-me walkthrough sits at its last step. A manual forward step
/// pauses Auto so the learner keeps control.
pub fn step_or_apply(game: &mut Game, delta: isize) {
    if delta >= 0 && game.show_me {
        if game.show_me_auto {
            game.show_me_auto = false;
        }
        advance(game);
    } else {
        game.teaching.step_by(delta);
    }
}

/// One timer tick in show-me auto mode: waits out the delay, then
/// advances one beat.
pub fn tick(game: &mut Game) {
    if !game.show_me || !game.show_me_auto || game.won {
        return;
    }
    if game.show_me_wait >= game.show_me_delay_ticks {
        game.show_me_wait = 0;
        advance(game);
    } else {
        game.show_me_wait += 1;
    }
}

/// Applies the selected annotation's effect and refreshes the offers.
pub fn apply(game: &mut Game) {
    let Some(annotation) = game.teaching.current().cloned() else {
        return;
    };
    match annotation.effect {
        Effect::Place { idx, digit } => {
            super::set_value(game, idx, digit);
        }
        Effect::Eliminate { removals } => {
            game.eliminated.extend(removals);
            game.eliminated.sort_unstable();
            game.eliminated.dedup();
        }
    }
    if game.won {
        stop(game);
        return;
    }
    let cands = suduko_tutor::candidates_with(&game.shown_values(), &game.eliminated);
    game.teaching.refresh(&cands);
    if game.teaching.offers().is_empty()
        && !game.won
        && let Some(trial) = super::trial::trial_annotation(game, &cands)
    {
        game.teaching.push_offer(trial);
    }
    game.teaching.select(0);
}
