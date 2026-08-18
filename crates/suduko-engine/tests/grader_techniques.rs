use suduko_engine::grader::candidates::Candidates;
use suduko_engine::grader::effect::Effect;
use suduko_engine::grader::techniques;

fn b(digit: u8) -> u16 {
    1 << (digit - 1)
}

fn cands(spec: &[(usize, u16)]) -> Candidates {
    let mut masks = [0x1FFu16; 81];
    for &(idx, mask) in spec {
        masks[idx] = mask;
    }
    Candidates::from_masks(masks, [false; 81])
}

fn without(digit: u8) -> u16 {
    0x1FF & !b(digit)
}

#[test]
fn naked_single_fires_only_on_unplaced_cells() {
    let open = cands(&[(3, b(7))]);
    match techniques::naked_single(&open).expect("unplaced single fires") {
        Effect::Place { idx, digit } => assert_eq!((idx, digit), (3, 7)),
        other => panic!("expected place, got {other:?}"),
    }

    let mut masks = [0x1FFu16; 81];
    masks[3] = b(7);
    let mut placed = [false; 81];
    placed[3] = true;
    let done = Candidates::from_masks(masks, placed);
    assert_eq!(techniques::naked_single(&done), None);
}

#[test]
fn hidden_single_places_digit_confined_to_one_cell() {
    // Row 0 hides digit 3 only in cell 4, which also holds a 7.
    let mut spec: Vec<(usize, u16)> = vec![(4, b(3) | b(7))];
    for col in 0..9 {
        if col != 4 {
            spec.push((col, without(3)));
        }
    }
    let c = cands(&spec);
    match techniques::hidden_single(&c).expect("digit 3 is confined") {
        Effect::Place { idx, digit } => assert_eq!((idx, digit), (4, 3)),
        other => panic!("expected place, got {other:?}"),
    }
}

#[test]
fn pointing_eliminating_outside_block_fires() {
    // Digit 4 in block 0 confined to col 0 (cells 0 and 9).
    let mut spec: Vec<(usize, u16)> = vec![(0, b(4) | b(7)), (9, b(4) | b(8))];
    for &idx in &[1, 2, 10, 11, 18, 19, 20] {
        spec.push((idx, without(4)));
    }
    let c = cands(&spec);
    match techniques::locked_candidates(&c).expect("pointing applies") {
        Effect::Eliminate { removals } => {
            assert!(
                removals.contains(&(27, 4)),
                "removes 4 below the block: {removals:?}"
            );
            assert!(removals.contains(&(72, 4)));
            assert!(!removals.contains(&(0, 4)));
        }
        other => panic!("expected eliminations, got {other:?}"),
    }
}

#[test]
fn claiming_eliminating_outside_line_fires() {
    // Digit 4 in row 0 confined to block 0 (cells 0 and 1).
    let mut spec: Vec<(usize, u16)> = vec![(0, b(4) | b(7)), (1, b(4) | b(8))];
    for col in 2..9 {
        spec.push((col, without(4)));
    }
    let c = cands(&spec);
    match techniques::locked_candidates(&c).expect("claiming applies") {
        Effect::Eliminate { removals } => {
            assert!(
                removals.contains(&(10, 4)),
                "removes 4 in block below row: {removals:?}"
            );
            assert!(removals.contains(&(20, 4)));
            assert!(!removals.contains(&(0, 4)));
        }
        other => panic!("expected eliminations, got {other:?}"),
    }
}

#[test]
fn naked_pair_strips_pair_digits_from_the_rest_of_the_row() {
    // Row 0: cells 0,1 are the pair {2,3}; the rest would consider both.
    let mut spec: Vec<(usize, u16)> = vec![(0, b(2) | b(3)), (1, b(2) | b(3))];
    for col in 2..9 {
        spec.push((col, b(1) | b(2) | b(3)));
    }
    let c = cands(&spec);
    match techniques::naked_set(&c).expect("naked pair applies") {
        Effect::Eliminate { removals } => {
            assert!(removals.contains(&(2, 2)) && removals.contains(&(2, 3)));
            assert!(removals.contains(&(8, 2)) && removals.contains(&(8, 3)));
            assert!(!removals.iter().any(|&(idx, _)| idx < 2));
            assert_eq!(removals.len(), 14, "7 cells x 2 digits: {removals:?}");
        }
        other => panic!("expected eliminations, got {other:?}"),
    }
}

#[test]
fn hidden_pair_strips_intruders_from_the_pair_cells() {
    // Row 0: digits 2,3 live only in cells 0,1, which also hold 5s.
    let mut spec: Vec<(usize, u16)> = vec![(0, b(2) | b(3) | b(5)), (1, b(2) | b(3) | b(5))];
    for col in 2..9 {
        spec.push((col, 0x1FF & !(b(2) | b(3))));
    }
    let c = cands(&spec);
    match techniques::hidden_set(&c).expect("hidden pair applies") {
        Effect::Eliminate { removals } => {
            assert!(removals.contains(&(0, 5)) && removals.contains(&(1, 5)));
            assert_eq!(removals.len(), 2, "only the stray 5s: {removals:?}");
        }
        other => panic!("expected eliminations, got {other:?}"),
    }
}

#[test]
fn hidden_pair_with_an_absent_digit_does_not_fire() {
    // Digit 3 is absent from row 0 entirely, so {2,3} is a degenerate pair:
    // digit 2 alone cannot lock cells 0,1 to two digits.
    let mut spec: Vec<(usize, u16)> = vec![(0, b(2) | b(5)), (1, b(2) | b(5))];
    for col in 2..9 {
        spec.push((col, 0x1FF & !(b(2) | b(3))));
    }
    let c = cands(&spec);
    assert_eq!(techniques::hidden_set(&c), None);
}

#[test]
fn xy_wing_removes_digit_seen_by_both_pincers() {
    // Pivot 0 holds {1,2}; pincers 1 (row peer) holds {1,3} and 9 (column
    // peer) holds {2,3}; cell 10 sees both pincers and must lose 3.
    let c = cands(&[
        (0, b(1) | b(2)),
        (1, b(1) | b(3)),
        (9, b(2) | b(3)),
        (10, b(3) | b(7)),
    ]);
    match techniques::xy_wing(&c).expect("xy-wing applies") {
        Effect::Eliminate { removals } => {
            assert!(
                removals.contains(&(10, 3)),
                "sees both pincers: {removals:?}"
            );
            assert!(!removals.iter().any(|&(idx, _)| idx == 0));
        }
        other => panic!("expected eliminations, got {other:?}"),
    }
}

#[test]
fn xy_wing_ignores_pincers_that_cannot_hold_the_z_digit() {
    // Pivot 0 holds {1,2}; pincer 1 holds {1,4} (no shared z), so no wing.
    let c = cands(&[
        (0, b(1) | b(2)),
        (1, b(1) | b(4)),
        (9, b(2) | b(3)),
        (10, b(3)),
    ]);
    assert_eq!(techniques::xy_wing(&c), None);
}

#[test]
fn x_wing_removes_digit_from_other_rows_in_the_two_columns() {
    // Digit 5 forms a rectangle: rows 0 and 4, cols 1 and 4.
    let mut spec: Vec<(usize, u16)> = vec![
        (0 * 9 + 1, b(5) | b(7)),
        (0 * 9 + 4, b(5) | b(8)),
        (4 * 9 + 1, b(5) | b(8)),
        (4 * 9 + 4, b(5) | b(6)),
    ];
    for col in 0..9 {
        if col != 1 && col != 4 {
            spec.push((col, without(5)));
            spec.push((4 * 9 + col, without(5)));
        }
    }
    let c = cands(&spec);
    match techniques::x_wing(&c).expect("x-wing applies") {
        Effect::Eliminate { removals } => {
            assert!(
                removals.contains(&(10, 5)),
                "col 1 outside the rectangle: {removals:?}"
            );
            assert!(removals.contains(&(76, 5)), "col 4 outside: {removals:?}");
            assert!(!removals.iter().any(|&(idx, _)| idx == 4));
        }
        other => panic!("expected eliminations, got {other:?}"),
    }
}

#[test]
fn swordfish_removes_digit_outside_three_lines() {
    // Digit 5 swordfish over rows 0,3,6 with cols 1,4,7.
    let mut spec: Vec<(usize, u16)> = vec![
        (0 * 9 + 1, b(5) | b(7)),
        (0 * 9 + 4, b(5) | b(8)),
        (3 * 9 + 4, b(5) | b(9)),
        (3 * 9 + 7, b(5) | b(6)),
        (6 * 9 + 1, b(5) | b(2)),
        (6 * 9 + 7, b(5) | b(3)),
    ];
    for col in 0..9 {
        if col != 1 && col != 4 && col != 7 {
            spec.push((col, without(5)));
            spec.push((3 * 9 + col, without(5)));
            spec.push((6 * 9 + col, without(5)));
        }
    }
    let c = cands(&spec);
    match techniques::swordfish(&c).expect("swordfish applies") {
        Effect::Eliminate { removals } => {
            assert!(removals.contains(&(10, 5)), "col 1 row 1: {removals:?}");
            assert!(removals.contains(&(2 * 9 + 4, 5)), "col 4 row 2");
            assert!(removals.contains(&(8 * 9 + 7, 5)), "col 7 row 8");
            assert!(!removals.iter().any(|&(idx, _)| (0..9).contains(&idx)));
        }
        other => panic!("expected eliminations, got {other:?}"),
    }
}

#[test]
fn apply_place_resolves_cell_and_clears_peers() {
    let mut c = cands(&[]);
    effect_apply(&mut c, 0, 3);
    assert_eq!(c.masks[0], b(3));
    assert!(c.placed[0]);
    assert_eq!(c.masks[1], without(3));
    assert_eq!(c.masks[9], without(3));
    assert_eq!(c.masks[10], without(3), "block peer");
    assert_eq!(c.masks[40], 0x1FF, "non-peer untouched");
}

fn effect_apply(c: &mut Candidates, idx: usize, digit: u8) {
    use suduko_engine::grader::effect;
    effect::apply(c, Effect::Place { idx, digit });
}
