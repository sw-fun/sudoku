//! Pencil-note layer: user-owned candidate removals.

use suduko_game::showme;
use suduko_game::{Game, NoteOp, clear_selected, entry, from_strings, note};

/// Wikipedia easy clues and its solution.
const CLUES: &str =
    "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
const SOLUTION: &str =
    "534678912672195348198342567859761423426853791713924856961537284287419635345286179";

fn game() -> Game {
    from_strings(CLUES, SOLUTION).expect("fixture agrees")
}

#[test]
fn notes_mode_entry_toggles_a_candidate_and_erase_restores_the_cell() {
    let mut g = game();
    g.select(2); // r1c3, empty; solution 4, computed marks contain 4
    assert!(g.pencil_marks()[2].contains(&4));
    g.notes_mode = true;
    entry(&mut g, 4);
    assert!(
        !g.pencil_marks()[2].contains(&4),
        "typing 4 in notes mode removes the 4 mark"
    );
    entry(&mut g, 4);
    assert!(
        g.pencil_marks()[2].contains(&4),
        "typing it again brings the mark back"
    );
    entry(&mut g, 2);
    assert!(!g.pencil_marks()[2].contains(&2));
    clear_selected(&mut g);
    assert!(
        g.pencil_marks()[2].contains(&2),
        "erase in notes mode restores the cell's computed candidates"
    );
}

#[test]
fn notes_mode_entry_never_places_digits() {
    let mut g = game();
    g.select(2);
    g.notes_mode = true;
    entry(&mut g, 4);
    entry(&mut g, 4); // toggle twice
    assert_eq!(g.user[2], 0, "notes mode never writes cell values");
    assert_eq!(g.bad_inputs, 0);
}

#[test]
fn extend_applies_strategy_eliminates_into_the_layer() {
    let mut g = game();
    g.open_learn();
    let pick = g
        .teaching
        .offers()
        .iter()
        .position(|a| matches!(a.effect, suduko_tutor::Effect::Eliminate { .. }))
        .expect("fixture has eliminations");
    g.teaching.select(pick);
    let removals = match &g.teaching.current().expect("selected").effect {
        suduko_tutor::Effect::Eliminate { removals } => removals.clone(),
        _ => unreachable!(),
    };
    note(&mut g, NoteOp::Extend(removals.clone()));
    let marks = g.pencil_marks();
    for &(idx, digit) in &removals {
        assert!(!marks[idx].contains(&digit), "{idx}/{digit} removed");
    }
}

#[test]
fn apply_all_eliminations_and_reset_marks_round_trip() {
    let mut g = game();
    g.open_learn();
    let mut all = Vec::new();
    for a in g.teaching.offers() {
        if let suduko_tutor::Effect::Eliminate { removals } = &a.effect {
            all.extend_from_slice(removals);
        }
    }
    assert!(!all.is_empty(), "fixture offers eliminations");
    note(&mut g, NoteOp::Extend(all.clone()));
    assert!(!g.eliminated.is_empty());
    let marks = g.pencil_marks();
    for &(idx, digit) in &all {
        assert!(!marks[idx].contains(&digit));
    }
    note(&mut g, NoteOp::Reset);
    assert!(g.eliminated.is_empty(), "reset clears the layer");
}

#[test]
fn stopping_show_me_keeps_the_user_marks() {
    let mut g = game();
    g.select(2);
    g.notes_mode = true;
    entry(&mut g, 4); // a user removal
    showme::start(&mut g);
    showme::stop(&mut g);
    assert!(
        !g.pencil_marks()[2].contains(&4),
        "stop no longer wipes the note layer"
    );
}
