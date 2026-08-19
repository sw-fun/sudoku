use suduko_game::{Game, erase, from_strings, highlight_set, set_value};

/// Wikipedia easy puzzle (singles-solvable) and its solution.
const CLUES: &str =
    "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
const SOLUTION: &str =
    "534678912672195348198342567859761423426853791713924856961537284287419635345286179";

fn game() -> Game {
    from_strings(CLUES, SOLUTION).expect("fixture agrees")
}

#[test]
fn given_cells_show_clues_and_cannot_be_changed() {
    let mut g = game();
    assert_eq!(g.shown(0), 5, "clue at r0c0");
    assert!(g.is_given(0));
    assert!(!g.is_given(2), "r0c3 is a player cell");
    let out = set_value(&mut g, 0, 9);
    assert!(out.ignored, "given cell ignores input");
    assert_eq!(g.shown(0), 5);
}

#[test]
fn wrong_entry_bumps_bad_inputs_every_time() {
    let mut g = game();
    assert_eq!(g.bad_inputs, 0);
    let out = set_value(&mut g, 2, 9); // solution is 4
    assert!(out.wrong);
    assert_eq!(g.bad_inputs, 1);
    let out = set_value(&mut g, 2, 9);
    assert!(out.wrong);
    assert_eq!(g.bad_inputs, 2, "every wrong entry counts");
    let out = set_value(&mut g, 2, 4);
    assert!(!out.wrong);
    assert_eq!(g.bad_inputs, 2, "correct entries never count");
    assert_eq!(g.shown(2), 4);
}

#[test]
fn erase_clears_user_values_only() {
    let mut g = game();
    set_value(&mut g, 2, 4);
    erase(&mut g, 2);
    assert_eq!(g.shown(2), 0);
    set_value(&mut g, 0, 9); // ignored: given
    erase(&mut g, 0);
    assert_eq!(g.shown(0), 5, "clue survives erase");
}

#[test]
fn empty_selected_cell_highlights_its_twenty_peers() {
    let mut g = game();
    g.select(2); // r0c3 is empty
    let hl = highlight_set(&g);
    assert_eq!(hl.len(), 20);
    assert!(hl.contains(&0) && hl.contains(&8) && hl.contains(&11));
    assert!(!hl.contains(&2));
}

#[test]
fn valued_selected_cell_highlights_same_number_minus_bad_guesses() {
    let mut g = game();
    // Put a wrong 5 in r0c3 and a correct 5 in r1c2 (solution 2? check:
    // SOLUTION r1 = 672195348, so r1c2 = 2). Choose cells whose solution
    // is 5 to make them correct: r0c0 is a clue 5. Select the clue 5 at 0.
    set_value(&mut g, 2, 9); // wrong value 9 nowhere else matters
    set_value(&mut g, 3, 5); // r0c4: solution 6 -> wrong 5, must NOT highlight
    g.select(0); // clue value 5
    let hl = highlight_set(&g);
    assert!(hl.contains(&14), "r1c5 holds a correct 5: {hl:?}");
    assert!(hl.contains(&71), "r7c8 holds a correct 5: {hl:?}");
    assert!(!hl.contains(&3), "wrong 5 at r0c4 must not highlight");
    assert!(!hl.contains(&0), "the selected cell itself is excluded");
}

#[test]
fn win_requires_all_cells_filled_correctly() {
    let mut g = game();
    assert!(!g.is_won());
    for idx in 0..81 {
        let digit = digit_at(SOLUTION, idx);
        set_value(&mut g, idx, digit);
    }
    assert!(g.is_won());
}

#[test]
fn wrong_cells_stay_wrong_until_corrected() {
    let mut g = game();
    set_value(&mut g, 2, 9);
    assert!(g.is_wrong(2));
    set_value(&mut g, 2, 4);
    assert!(!g.is_wrong(2));
}

fn digit_at(s: &str, idx: usize) -> u8 {
    s.as_bytes()[idx] - b'0'
}
