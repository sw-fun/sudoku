//! Walkthrough step-view derivation: which cells/units/marks the
//! board should emphasize for the current teaching step, and which
//! effect targets pulse (red eliminations, green placements).

use suduko_game::{Game, from_strings};
use suduko_tutor::UnitRef;

/// Wikipedia easy clues plus solution fills at idx 2,3,5,6,7 (the
/// tutor test fixture: X-Wing on 2s, pairs, pointing, XY-wing...).
const CLUES: &str =
    "53467891.6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
const SOLUTION: &str =
    "534678912672195348198342567859761423426853791713924856961537284287419635345286179";

fn game() -> Game {
    from_strings(CLUES, SOLUTION).expect("fixture agrees")
}

fn xwing_walkthrough() -> Game {
    let mut g = game();
    g.open_learn();
    let idx = g
        .teaching
        .offers()
        .iter()
        .position(|a| a.title.starts_with("X-Wing on 2s"))
        .expect("X-Wing on 2s present");
    g.teaching.select(idx);
    g
}

#[test]
fn step_view_flags_pattern_cells_and_units_per_step() {
    let mut g = xwing_walkthrough();
    // Step 0: first base line - its two corners + the row unit.
    let view = g.step_view().expect("view exists");
    assert!(
        view.cells.contains(&21) && view.cells.contains(&23),
        "{:?}",
        view.cells
    );
    assert!(view.units.contains(&UnitRef::Col(3)), "{:?}", view.units);
    assert_eq!(view.focus_digits, vec![2]);
    // Step 2: all four corners.
    g.teaching.step_by(2);
    let view = g.step_view().expect("view exists");
    for corner in [21, 23, 75, 77] {
        assert!(view.cells.contains(&corner), "{:?}", view.cells);
    }
    // Final step: elimination targets, not corners.
    g.teaching.step_by(1);
    let view = g.step_view().expect("view exists");
    for target in [18, 26, 72, 73, 74] {
        assert!(view.cells.contains(&target), "{:?}", view.cells);
    }
    assert!(!view.cells.contains(&21));
}

#[test]
fn effect_targets_pulse_red_for_eliminations_green_for_placements() {
    let g = xwing_walkthrough();
    let view = g.step_view().expect("view exists");
    assert_eq!(
        view.pulse,
        suduko_game::Pulse::Red,
        "X-Wing ends in eliminations"
    );
    assert_eq!(view.pulse_cells, vec![18, 26, 72, 73, 74]);

    let mut g2 = game();
    g2.open_learn();
    let idx = g2
        .teaching
        .offers()
        .iter()
        .position(|a| a.title.starts_with("Naked Single"))
        .expect("singles present");
    g2.teaching.select(idx);
    let view2 = g2.step_view().expect("view exists");
    assert_eq!(view2.pulse, suduko_game::Pulse::Green);
    assert_eq!(view2.pulse_cells.len(), 1, "one placed cell");
}

#[test]
fn emphasized_marks_carry_the_focus_digit_only_in_relevant_cells() {
    let g = xwing_walkthrough();
    let view = g.step_view().expect("view exists");
    // Corners 21 and 23 hold a 2 candidate: emphasized there.
    assert!(view.marks.get(&21) == Some(&vec![2]) || view.marks.get(&21) == Some(&vec![2, 3]));
    // A cell with no 2 candidate gets no emphasis.
    assert_eq!(view.marks.get(&20), None);
    let _ = view;
}
