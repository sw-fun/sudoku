use suduko_game::{
    Game, NotesMode, clear_selected, entry, from_strings, keypad_visible, set_value,
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
fn selecting_a_cell_opens_the_keypad_and_reclick_toggles_it() {
    let mut g = game();
    g.select(2); // r0c3, empty player cell
    assert!(keypad_visible(&g), "first tap selects and opens");
    g.select(2);
    assert!(!keypad_visible(&g), "second tap on the same cell closes");
    g.select(2);
    assert!(keypad_visible(&g), "third tap re-opens");
    g.select(5); // different empty player cell (r0c5)
    assert!(keypad_visible(&g), "tapping another cell re-opens on it");
}

#[test]
fn keypad_stays_hidden_for_givens_notes_showme_and_won() {
    let mut g = game();
    g.select(0); // r0c0 is a clue
    assert!(!keypad_visible(&g), "given cells never open the keypad");
    g.select(2);
    g.notes = NotesMode::Auto;
    assert!(!keypad_visible(&g), "notes mode hides the keypad");
    g.notes = NotesMode::Off;
    g.show_me = true;
    assert!(!keypad_visible(&g), "show-me hides the keypad");
    g.show_me = false;
    let won = from_strings(SOLUTION, SOLUTION).expect("solved fixture");
    assert!(!keypad_visible(&won), "a won board hides the keypad");
}

#[test]
fn placing_or_erasing_closes_the_keypad() {
    let mut g = game();
    g.select(2);
    entry(&mut g, 4);
    assert!(!keypad_visible(&g), "digit entry closes the keypad");
    g.select(2);
    clear_selected(&mut g);
    assert!(!keypad_visible(&g), "erase closes the keypad");
    g.select(2);
    g.notes = NotesMode::Auto;
    entry(&mut g, 5);
    g.notes = NotesMode::Off;
    assert!(
        keypad_visible(&g),
        "notes-mode typing never touched the popup state"
    );
}

#[test]
fn a_wrong_value_keeps_the_keypad_usable_for_replacement() {
    let mut g = game();
    g.select(2);
    set_value(&mut g, 2, 9); // red 9; the cell stays selected & open
    g.keypad_open = true;
    assert!(keypad_visible(&g), "a red cell still opens the keypad");
}
