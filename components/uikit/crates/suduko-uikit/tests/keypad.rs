use suduko_uikit::{Anchor, CellInput, HAlign, anchor, anchor_style};

#[test]
fn anchor_points_the_keypad_away_from_the_board_edges() {
    assert_eq!(
        anchor(0),
        Anchor {
            below: true,
            h: HAlign::Left
        },
        "top-left cell: keypad below, left-aligned"
    );
    assert_eq!(
        anchor(80),
        Anchor {
            below: false,
            h: HAlign::Right
        },
        "bottom-right cell: keypad above, right-aligned"
    );
    assert_eq!(
        anchor(40),
        Anchor {
            below: true,
            h: HAlign::Center
        },
        "middle cell: keypad below, centered"
    );
}

#[test]
fn anchor_style_places_the_keypad_inside_the_board() {
    let s = anchor_style(0);
    assert!(s.contains("top: calc(1 * var(--cell)"), "below row 0");
    assert!(
        s.contains("left: calc(0 * var(--cell)"),
        "left-aligned at col 0"
    );
    assert!(!s.contains("transform"), "no shift needed at the corner");

    let s = anchor_style(80);
    assert!(s.contains("bottom: calc(1 * var(--cell)"), "above row 8");
    assert!(
        s.contains("right: calc(0 * var(--cell)"),
        "right-aligned at col 8"
    );

    let s = anchor_style(4);
    assert!(s.contains("translateX(-50%)"), "centered column shifts");
    assert!(s.contains("left: calc(4.5 * var(--cell)"));
}

#[test]
fn digit_buttons_mirror_completion_and_the_red_value() {
    let empty = CellInput {
        complete_mask: 0,
        wrong_digit: None,
        value: None,
        given: false,
    };
    for d in 1..=9u8 {
        assert!(empty.digit_enabled(d), "empty cell: every digit enabled");
    }
    let solved = CellInput {
        complete_mask: 0b1_1111_1111,
        ..empty
    };
    for d in 1..=9u8 {
        assert!(!solved.digit_enabled(d), "completed digits stay disabled");
    }
    let red = CellInput {
        wrong_digit: Some(9),
        ..empty
    };
    assert!(!red.digit_enabled(9), "the red digit itself is disabled");
    assert!(red.digit_enabled(4), "the fix is one tap away");
    assert!(red.digit_enabled(1), "any other replacement works too");
}

#[test]
fn erase_follows_the_selected_cell_contents() {
    let empty = CellInput {
        complete_mask: 0,
        wrong_digit: None,
        value: None,
        given: false,
    };
    assert!(!empty.erase_enabled(), "empty cell: nothing to erase");
    let valued = CellInput {
        value: Some(9),
        ..empty
    };
    assert!(valued.erase_enabled(), "a value can be erased");
    let clue = CellInput {
        value: Some(5),
        given: true,
        ..empty
    };
    assert!(!clue.erase_enabled(), "clues cannot be erased");
}
