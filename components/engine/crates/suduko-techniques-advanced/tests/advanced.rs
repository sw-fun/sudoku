use suduko_techniques::{Candidates, Effect};
use suduko_techniques_advanced::{fish, xy_wing};

fn b(digit: u8) -> u16 {
    1 << (digit - 1)
}

fn without(digit: u8) -> u16 {
    0x1FF & !b(digit)
}

fn cands(spec: &[(usize, u16)]) -> Candidates {
    let mut masks = [0x1FFu16; 81];
    for &(idx, mask) in spec {
        masks[idx] = mask;
    }
    Candidates {
        masks,
        placed: [false; 81],
    }
}

#[test]
fn xy_wing_removes_digit_seen_by_both_pincers() {
    let c = cands(&[
        (0, b(1) | b(2)),
        (1, b(1) | b(3)),
        (9, b(2) | b(3)),
        (10, b(3) | b(7)),
    ]);
    match xy_wing(&c).expect("xy-wing applies") {
        Effect::Eliminate { removals } => {
            assert!(
                removals.contains(&(10, 3)),
                "sees both pincers: {removals:?}"
            );
            assert!(!removals.iter().any(|&(idx, _)| idx == 0));
        }
        Effect::Place { .. } => panic!("expected eliminations"),
    }
}

#[test]
fn xy_wing_ignores_pincers_that_cannot_hold_the_z_digit() {
    let c = cands(&[
        (0, b(1) | b(2)),
        (1, b(1) | b(4)),
        (9, b(2) | b(3)),
        (10, b(3)),
    ]);
    assert_eq!(xy_wing(&c), None);
}

#[test]
fn x_wing_removes_digit_from_other_rows_in_the_two_columns() {
    // Digit 5 rectangle: rows 0 and 4, cols 1 and 4 (cells 1, 4, 37, 40).
    let mut spec: Vec<(usize, u16)> = vec![
        (1, b(5) | b(7)),
        (4, b(5) | b(8)),
        (37, b(5) | b(8)),
        (40, b(5) | b(6)),
    ];
    for col in 0..9 {
        if col != 1 && col != 4 {
            spec.push((col, without(5)));
            spec.push((4 * 9 + col, without(5)));
        }
    }
    match fish(&cands(&spec), 2).expect("x-wing applies") {
        Effect::Eliminate { removals } => {
            assert!(removals.contains(&(10, 5)), "col 1 outside: {removals:?}");
            assert!(removals.contains(&(76, 5)), "col 4 outside: {removals:?}");
            assert!(!removals.iter().any(|&(idx, _)| idx == 4));
        }
        Effect::Place { .. } => panic!("expected eliminations"),
    }
}

#[test]
fn swordfish_removes_digit_outside_three_lines() {
    // Digit 5 swordfish over rows 0,3,6 with cols 1,4,7
    // (cells 1, 4, 31, 34, 55, 61).
    let mut spec: Vec<(usize, u16)> = vec![
        (1, b(5) | b(7)),
        (4, b(5) | b(8)),
        (31, b(5) | b(9)),
        (34, b(5) | b(6)),
        (55, b(5) | b(2)),
        (61, b(5) | b(3)),
    ];
    for col in 0..9 {
        if col != 1 && col != 4 && col != 7 {
            spec.push((col, without(5)));
            spec.push((3 * 9 + col, without(5)));
            spec.push((6 * 9 + col, without(5)));
        }
    }
    match fish(&cands(&spec), 3).expect("swordfish applies") {
        Effect::Eliminate { removals } => {
            assert!(removals.contains(&(10, 5)), "col 1 row 1: {removals:?}");
            assert!(removals.contains(&(2 * 9 + 4, 5)), "col 4 row 2");
            assert!(removals.contains(&(8 * 9 + 7, 5)), "col 7 row 8");
            assert!(!removals.iter().any(|&(idx, _)| (0..9).contains(&idx)));
        }
        Effect::Place { .. } => panic!("expected eliminations"),
    }
}
