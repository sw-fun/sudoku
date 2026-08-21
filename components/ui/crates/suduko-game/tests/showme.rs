//! Show-me solver mode: the game plays itself, strategy by strategy.

use suduko_game::showme::{advance, apply, start, stop};
use suduko_game::{Game, from_strings};

/// Wikipedia easy clues and its solution (singles-solvable).
const CLUES: &str =
    "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
const SOLUTION: &str =
    "534678912672195348198342567859761423426853791713924856961537284287419635345286179";

/// The mid-game tutor fixture (X-Wing on 2s among others).
const MID_CLUES: &str =
    "53467891.6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";

fn solved_game() -> Game {
    from_strings(CLUES, SOLUTION).expect("fixture agrees")
}

fn mid_game() -> Game {
    from_strings(MID_CLUES, SOLUTION).expect("fixture agrees")
}

#[test]
fn show_me_solves_the_board_strategy_by_strategy() {
    let mut g = solved_game();
    start(&mut g);
    assert!(g.show_me && g.teaching.panel_open);
    let mut guard = 0;
    while !g.won {
        assert!(g.teaching.current().is_some(), "always a next strategy");
        advance(&mut g);
        guard += 1;
        assert!(guard < 1000, "solver must terminate");
    }
    assert!(g.is_won());
    assert!(!g.show_me, "show-me stops itself on victory");
    let filled = g.user.iter().filter(|&&v| v != 0).count();
    assert_eq!(filled, 81 - g.clues.iter().filter(|&&c| c != 0).count());
}

#[test]
fn eliminations_persist_into_marks_and_future_offers() {
    let mut g = mid_game();
    start(&mut g);
    let pick = g
        .teaching
        .offers()
        .iter()
        .position(|a| matches!(a.effect, suduko_tutor::Effect::Eliminate { .. }))
        .expect("mid-game fixture has eliminations");
    g.teaching.select(pick);
    apply(&mut g);
    assert!(!g.eliminated.is_empty(), "applying removes candidates");
    let &(idx, digit) = &g.eliminated[0];
    let marks = g.pencil_marks();
    assert!(
        !marks[idx].contains(&digit),
        "eliminated {digit} leaves the marks of cell {idx}"
    );
    assert!(
        !g.teaching.offers().is_empty(),
        "offers refresh after applying"
    );
}

#[test]
fn stopping_clears_solver_state_but_keeps_the_board() {
    let mut g = mid_game();
    start(&mut g);
    apply(&mut g);
    assert!(!g.eliminated.is_empty() || g.user.iter().any(|&v| v != 0));
    stop(&mut g);
    assert!(!g.show_me && !g.teaching.panel_open && g.eliminated.is_empty());
    assert!(g.teaching.offers().is_empty());
}
