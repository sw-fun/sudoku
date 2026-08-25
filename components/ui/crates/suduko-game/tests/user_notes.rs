use suduko_game::{
    Game, NoteOp, clear_selected, entry, from_strings, keypad_visible, note, restore, save,
    set_value,
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
fn fresh_games_start_in_value_mode_with_empty_notes() {
    let g = game();
    assert!(!g.pencil);
    assert!(g.user_marks.iter().all(|&m| m == 0));
}

#[test]
fn pencil_on_digit_presses_toggle_notes_and_never_place() {
    let mut g = game();
    g.pencil = true;
    g.select(8); // r0c8, candidates are exactly 2 and 8
    entry(&mut g, 2);
    assert_eq!(g.user_marks[8], 1 << 1, "first press writes the note");
    entry(&mut g, 8);
    assert_eq!(g.user_marks[8], (1 << 1) | (1 << 7), "second digit adds");
    entry(&mut g, 2);
    assert_eq!(g.user_marks[8], 1 << 7, "repeat press removes");
    assert_eq!(g.shown(8), 0, "pencil mode never places values");
    assert_eq!(g.bad_inputs, 0, "and never counts bad inputs");
}

#[test]
fn pencil_notes_only_land_in_empty_cells_and_legal_digits() {
    let mut g = game();
    g.pencil = true;
    g.select(0); // r0c0 is a clue
    entry(&mut g, 2);
    assert_eq!(g.user_marks[0], 0, "clue cells never take notes");
    g.select(8);
    entry(&mut g, 5); // 5 already in r0 (peer)
    assert_eq!(
        g.user_marks[8] & (1 << 4),
        0,
        "digits excluded by the rules are ignored"
    );
}

#[test]
fn pencil_erase_clears_the_cells_notes_value_erase_clears_the_value() {
    let mut g = game();
    g.pencil = true;
    g.select(8);
    entry(&mut g, 2);
    clear_selected(&mut g);
    assert_eq!(g.user_marks[8], 0, "pencil erase wipes the cell's notes");
    g.pencil = false;
    set_value(&mut g, 8, 2);
    clear_selected(&mut g);
    assert_eq!(g.shown(8), 0, "value erase clears the value");
}

#[test]
fn placing_a_value_prunes_that_digits_notes_from_all_peers() {
    use suduko_grid::peers_of;
    let mut g = game();
    let peers: Vec<usize> = peers_of(2).to_vec();
    // pencil 9 into every cell where it is legal (dynamically verified)
    g.pencil = true;
    let mut nonpeer_targets = 0;
    for cell in 0..81usize {
        g.select(cell);
        entry(&mut g, 9);
        if !peers.contains(&cell) && g.user_marks[cell] & (1 << 8) != 0 {
            nonpeer_targets += 1;
        }
    }
    assert!(nonpeer_targets > 0, "some non-peer holds a 9 note");
    g.pencil = false;
    // place 9 at r0c2 (empty player cell, solution 4): peers lose the 9 note
    let out = set_value(&mut g, 2, 9);
    assert!(out.wrong, "9 is wrong at r0c2 (solution 4)");
    for &p in &peers {
        assert_eq!(g.user_marks[p] & (1 << 8), 0, "peer {p} lost its 9 note");
    }
    let kept = (0..81)
        .filter(|&c| !peers.contains(&c) && g.user_marks[c] & (1 << 8) != 0)
        .count();
    assert_eq!(kept, nonpeer_targets, "every non-peer keeps its 9 note");
}

#[test]
fn user_marks_view_lists_sorted_digits() {
    let mut g = game();
    g.pencil = true;
    g.select(8);
    entry(&mut g, 8);
    entry(&mut g, 2);
    assert_eq!(g.user_marks_view()[8], vec![2, 8]);
}

#[test]
fn keypad_only_opens_in_value_mode() {
    let mut g = game();
    g.select(2);
    assert!(keypad_visible(&g), "value mode: popup keypad available");
    g.pencil = true;
    assert!(!keypad_visible(&g), "pencil mode hides the keypad");
}

#[test]
fn fill_and_clear_notes_actions_manage_the_whole_layer() {
    let mut g = game();
    note(&mut g, NoteOp::FillUser);
    let empty_cells = (0..81).filter(|&i| g.shown(i) == 0).count();
    let filled = g.user_marks.iter().filter(|&&m| m != 0).count();
    assert_eq!(empty_cells, filled, "every empty cell gains its candidates");
    assert!(
        g.shown(8) == 0 && g.user_marks[8] != 0,
        "empty cell carries marks"
    );
    assert_eq!(g.user_marks[0], 0, "clue cells stay unmarked");
    note(&mut g, NoteOp::ClearUser);
    assert!(
        g.user_marks.iter().all(|&m| m == 0),
        "clear wipes all notes"
    );
}

#[test]
fn v3_saves_carry_pencil_and_marks() {
    let mut g = game();
    g.pencil = true;
    g.select(8);
    entry(&mut g, 2);
    set_value(&mut g, 5, 3);
    let code = save(0, Some(&g), &[(0u8, 7u32)].into_iter().collect());
    let back = restore(&code).expect("v3 restores");
    let rg = back.game.expect("game saved");
    assert!(rg.pencil, "pencil state rides along");
    assert_eq!(rg.user_marks[8], 1 << 1);
    assert_eq!(rg.user[5], 3, "placed values ride along");
}

#[test]
fn v2_saves_map_user_and_auto_notes_forward() {
    let clues = "530070000600195000098000060800060003400803001700020006060000280000419005000080079";
    let zeros = "0".repeat(81);
    // v2 notes=1 (User) with a mark at cell 8 -> pencil on, mark kept
    let user = format!("v2|0|{clues}|{SOLUTION}|{zeros}|42|1|1|8=2;|0=2");
    let back = restore(&user).expect("v2 user restores");
    let rg = back.game.expect("game");
    assert!(rg.pencil);
    assert_eq!(rg.user_marks[8], 1 << 1);
    // v2 notes=2 (Auto) -> pencil off, computed candidates filled in
    let auto = format!("v2|0|{clues}|{SOLUTION}|{zeros}|42|1|2||0=2");
    let back = restore(&auto).expect("v2 auto restores");
    let rg = back.game.expect("game");
    assert!(!rg.pencil);
    let expected = {
        let vals = core::array::from_fn(|i: usize| clues.as_bytes()[i] - b'0');
        suduko_tutor::candidates_with(&vals, &[]).masks[8] & 0x1ff
    };
    assert_eq!(
        u32::from(rg.user_marks[8]),
        u32::from(expected),
        "auto fills exactly the computed candidates at r0c8"
    );
}
