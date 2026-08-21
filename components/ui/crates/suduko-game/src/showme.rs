//! Show-me solver mode: the game plays itself, strategy by strategy,
//! applying each annotation's effect before moving to the next.

use super::Game;
use suduko_grid::CELL_COUNT;
use suduko_tutor::Effect;

/// Candidate state for the current board minus the elimination layer.
fn tutor_candidates(game: &Game) -> suduko_tutor::Candidates {
    suduko_tutor::candidates_with(&game.shown_values(), &game.eliminated)
}

/// Starts show-me mode: open the panel on the current board and
/// select the first (cheapest) strategy.
pub fn start(game: &mut Game) {
    game.show_me = true;
    game.show_me_auto = true;
    game.teaching.open(&tutor_candidates(game));
    game.teaching.select(0);
}

/// Stops show-me mode and clears solver state (the board survives).
pub fn stop(game: &mut Game) {
    game.show_me = false;
    game.show_me_auto = false;
    game.eliminated.clear();
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
/// show-me walkthrough sits at its last step (Back stays manual).
pub fn step_or_apply(game: &mut Game, delta: isize) {
    if delta >= 0 && game.show_me {
        advance(game);
    } else {
        game.teaching.step_by(delta);
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
    game.teaching.refresh(&tutor_candidates(game));
    game.teaching.select(0);
}

/// Pencil marks for a board minus an elimination layer.
#[must_use]
pub fn marks(shown: &[u8; CELL_COUNT], eliminated: &[(usize, u8)]) -> [Vec<u8>; CELL_COUNT] {
    let cands = suduko_tutor::candidates_with(shown, eliminated);
    core::array::from_fn(|idx| {
        if !cands.placed[idx] {
            (1..=9u8)
                .filter(|d| cands.masks[idx] & (1 << (d - 1)) != 0)
                .collect()
        } else {
            Vec::new()
        }
    })
}
