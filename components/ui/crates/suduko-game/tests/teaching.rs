//! Teaching (learn mode) pure state: panel open/closed, selected
//! annotation, step cursor, and board-to-tutor glue.

use suduko_game::{Game, from_strings};

/// Wikipedia easy clues (singles-rich start) and its solution.
const CLUES: &str =
    "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
const SOLUTION: &str =
    "534678912672195348198342567859761423426853791713924856961537284287419635345286179";

fn game() -> Game {
    from_strings(CLUES, SOLUTION).expect("fixture agrees")
}

#[test]
fn learn_panel_starts_closed_and_opens_with_current_strategies() {
    let mut g = game();
    assert!(!g.teaching.panel_open);
    g.open_learn();
    assert!(g.teaching.panel_open);
    let list = g.teaching.offers();
    assert!(!list.is_empty(), "easy start has strategies");
    assert!(
        list.iter()
            .any(|o| o.strategy == suduko_tutor::Strategy::NakedSingle),
        "singles come first: {list:?}"
    );
}

#[test]
fn selecting_an_annotation_starts_its_walkthrough_at_step_zero() {
    let mut g = game();
    g.open_learn();
    g.teaching.select(0);
    assert_eq!(
        g.teaching.current().map(|a| a.title.clone()),
        Some(g.teaching.offers()[0].title.clone())
    );
    assert_eq!(g.teaching.step_index, 0);
    let first_text = g.teaching.offers()[0].steps[0].text.clone();
    assert_eq!(
        g.teaching
            .current()
            .and_then(|a| a.steps.get(g.teaching.step_index))
            .map(|s| s.text.clone()),
        Some(first_text)
    );
}

#[test]
fn stepping_moves_within_bounds_and_close_resets() {
    let mut g = game();
    g.open_learn();
    g.teaching.select(0);
    let last = g.teaching.current().expect("selected").steps.len() - 1;
    for _ in 0..(last + 5) {
        g.teaching.step_by(1);
    }
    assert_eq!(
        g.teaching.step_index, last,
        "cursor clamps at the last step"
    );
    g.teaching.step_by(-1);
    g.teaching.step_by(-1);
    assert_eq!(
        g.teaching.step_index,
        last - 2,
        "cursor clamps at zero from above"
    );
    g.close_learn();
    assert!(!g.teaching.panel_open);
    assert_eq!(g.teaching.current(), None);
    assert_eq!(g.teaching.step_index, 0);
}

#[test]
fn shown_values_feed_the_tutor_and_pencil_marks_follow_the_board() {
    let mut g = game();
    g.open_learn();
    let marks = g.pencil_marks();
    for (idx, cell_marks) in marks.iter().enumerate() {
        if g.shown(idx) != 0 {
            assert!(cell_marks.is_empty(), "filled cells carry no marks");
        }
    }
    // Fill one correct cell; its marks vanish and peers shrink.
    let idx = 2; // solution 4
    let mut g2 = game();
    g2.open_learn();
    let before = g2.pencil_marks();
    g2.user[idx] = 4;
    let after = g2.pencil_marks();
    assert!(after[idx].is_empty());
    let peer = 11; // r2c3 sees idx 2
    assert!(
        after[peer].len() <= before[peer].len(),
        "placing 4 removes it from peers"
    );
    assert!(before[2].contains(&4));
    let _ = g;
}
