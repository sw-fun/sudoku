use suduko_game::{
    Game, NotesMode, clear_selected, entry, from_strings, keypad_visible, restore, save, set_value,
};

/// Wikipedia easy puzzle (singles-solvable) and its solution.
const CLUES: &str =
    "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
const SOLUTION: &str =
    "534678912672195348198342567859761423426853791713924856961537284287419635345286179";

fn game() -> Game {
    from_strings(CLUES, SOLUTION).expect("fixture agrees")
}

#[test]
fn fresh_games_start_with_notes_off_and_empty_user_marks() {
    let g = game();
    assert_eq!(g.notes, NotesMode::Off);
    assert!(g.user_marks.iter().all(|&m| m == 0));
}

#[test]
fn user_mode_digit_presses_toggle_marks_starting_empty() {
    let mut g = game();
    g.notes = NotesMode::User;
    g.select(8); // r0c8, candidates are exactly 2 and 8
    entry(&mut g, 2);
    assert_eq!(g.user_marks[8], 1 << 1, "first press writes the mark");
    entry(&mut g, 8);
    assert_eq!(g.user_marks[8], (1 << 1) | (1 << 7), "second digit adds");
    entry(&mut g, 2);
    assert_eq!(g.user_marks[8], 1 << 7, "repeat press removes");
    assert_eq!(g.shown(2), 0, "user mode never places values");
    assert_eq!(g.bad_inputs, 0, "and never counts bad inputs");
}

#[test]
fn user_mode_marks_only_land_in_empty_cells_and_legal_digits() {
    let mut g = game();
    g.notes = NotesMode::User;
    g.select(0); // r0c0 is a clue
    entry(&mut g, 4);
    assert_eq!(g.user_marks[0], 0, "clue cells never take marks");
    g.select(8);
    entry(&mut g, 5); // 5 already in r0 (peer)
    assert_eq!(
        g.user_marks[8] & (1 << 4),
        0,
        "digits excluded by the rules are ignored"
    );
}

#[test]
fn user_mode_erase_clears_the_cells_marks() {
    let mut g = game();
    g.notes = NotesMode::User;
    g.select(8);
    entry(&mut g, 2);
    entry(&mut g, 8);
    clear_selected(&mut g);
    assert_eq!(g.user_marks[8], 0, "erase wipes the cell's user marks");
}

#[test]
fn auto_mode_keeps_computed_candidate_strike_outs() {
    let mut g = game();
    g.notes = NotesMode::Auto;
    g.select(2);
    let computed = g.pencil_marks()[2].clone();
    assert!(!computed.is_empty(), "auto shows computed candidates");
    entry(&mut g, computed[0]);
    let after = g.pencil_marks()[2].clone();
    assert!(
        !after.contains(&computed[0]),
        "striking a computed candidate removes it"
    );
    entry(&mut g, computed[0]);
    assert!(
        g.pencil_marks()[2].contains(&computed[0]),
        "repeat press restores it"
    );
}

#[test]
fn user_marks_render_view_round_trips_the_bitmask() {
    let mut g = game();
    g.notes = NotesMode::User;
    g.select(8);
    entry(&mut g, 2);
    entry(&mut g, 8);
    assert_eq!(g.user_marks_view()[8], vec![2, 8]);
}

#[test]
fn keypad_only_opens_when_notes_are_off() {
    let mut g = game();
    g.select(2);
    assert!(keypad_visible(&g), "off: popup keypad available");
    g.notes = NotesMode::User;
    assert!(!keypad_visible(&g), "user notes hide the keypad");
    g.notes = NotesMode::Auto;
    assert!(!keypad_visible(&g), "auto notes hide the keypad");
}

#[test]
fn user_marks_and_mode_survive_the_save_slot() {
    let mut g = game();
    g.notes = NotesMode::User;
    g.select(8);
    entry(&mut g, 2);
    set_value(&mut g, 5, 3);
    let code = save(0, Some(&g), &[(0u8, 7u32)].into_iter().collect());
    let back = restore(&code).expect("v2 restores");
    let rg = back.game.expect("game saved");
    assert_eq!(rg.notes, NotesMode::User);
    assert_eq!(rg.user_marks[8], 1 << 1);
    assert_eq!(rg.user[5], 3, "placed values ride along");
    assert_eq!(back.stats[&0], 7);
}

#[test]
fn v1_saves_restore_with_notes_off_and_empty_marks() {
    let clues = "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
    let zeros = "0".repeat(81);
    let v1 = format!("v1|0|{clues}|{SOLUTION}|{zeros}|42|1|0=2");
    let back = restore(&v1).expect("v1 back-compatible");
    let rg = back.game.expect("v1 game restores");
    assert_eq!(rg.notes, NotesMode::Off);
    assert!(rg.user_marks.iter().all(|&m| m == 0));
    assert_eq!(back.stats[&0], 2);
}
