//! Pencil-note layer: user-owned candidate removals.

use suduko_game::showme;
use suduko_game::{Game, NoteOp, clear_selected, entry, from_strings, note, set_value};

/// Wikipedia easy clues and its solution.
const CLUES: &str =
    "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
const SOLUTION: &str =
    "534678912672195348198342567859761423426853791713924856961537284287419635345286179";

fn game() -> Game {
    from_strings(CLUES, SOLUTION).expect("fixture agrees")
}

#[test]
fn pencil_toggles_notes_and_erase_clears_them() {
    let mut g = game();
    g.pencil = true;
    g.select(2); // r0c3, empty; solution 4, candidates include 4
    entry(&mut g, 4);
    assert_eq!(g.user_marks[2], 1 << 3, "typing 4 pencils it in");
    entry(&mut g, 4);
    assert_eq!(g.user_marks[2], 0, "typing it again removes it");
    entry(&mut g, 4);
    clear_selected(&mut g);
    assert_eq!(g.user_marks[2], 0, "erase clears the cell's notes");
}

#[test]
fn pencil_entry_never_places_digits() {
    let mut g = game();
    g.pencil = true;
    g.select(2);
    entry(&mut g, 4);
    entry(&mut g, 4); // toggle twice
    assert_eq!(g.user[2], 0, "pencil mode never writes cell values");
    assert_eq!(g.bad_inputs, 0);
}

#[test]
fn placement_prunes_peer_notes_even_from_pencil_layer() {
    let mut g = game();
    g.pencil = true;
    for &cell in &[2usize, 11, 20] {
        // r0 peers of the placed cell
        g.select(cell);
        entry(&mut g, 9);
    }
    g.pencil = false;
    set_value(&mut g, 5, 9); // any r0 placement prunes r0 notes
    assert_eq!(g.user_marks[2] & (1 << 8), 0, "peer lost the 9 note");
    assert_eq!(g.user_marks[11] & (1 << 8), 0);
    assert_eq!(g.user_marks[20] & (1 << 8), 0);
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
    g.pencil = true;
    g.select(2);
    entry(&mut g, 4); // a user note
    showme::start(&mut g);
    showme::stop(&mut g);
    assert_eq!(
        g.user_marks[2],
        1 << 3,
        "stop no longer wipes the user notes"
    );
}
