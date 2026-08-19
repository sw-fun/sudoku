use suduko_techniques::{Candidates, Effect};
use suduko_techniques::{hidden_set, locked_candidates, naked_set, naked_single};

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
fn naked_single_fires_only_on_unplaced_cells() {
    let open = cands(&[(3, b(7))]);
    match naked_single(&open).expect("unplaced single fires") {
        Effect::Place { idx, digit } => assert_eq!((idx, digit), (3, 7)),
        Effect::Eliminate { .. } => panic!("expected place"),
    }
    let mut masks = [0x1FFu16; 81];
    masks[3] = b(7);
    let mut placed = [false; 81];
    placed[3] = true;
    let done = Candidates { masks, placed };
    assert_eq!(naked_single(&done), None);
}

#[test]
fn hidden_single_places_digit_confined_to_one_cell() {
    let mut spec: Vec<(usize, u16)> = vec![(4, b(3) | b(7))];
    for col in 0..9 {
        if col != 4 {
            spec.push((col, without(3)));
        }
    }
    match suduko_techniques::hidden_single(&cands(&spec)).expect("digit 3 confined") {
        Effect::Place { idx, digit } => assert_eq!((idx, digit), (4, 3)),
        Effect::Eliminate { .. } => panic!("expected place"),
    }
}

#[test]
fn pointing_eliminating_outside_block_fires() {
    let mut spec: Vec<(usize, u16)> = vec![(0, b(4) | b(7)), (9, b(4) | b(8))];
    for &idx in &[1, 2, 10, 11, 18, 19, 20] {
        spec.push((idx, without(4)));
    }
    match locked_candidates(&cands(&spec)).expect("pointing applies") {
        Effect::Eliminate { removals } => {
            assert!(removals.contains(&(27, 4)), "below the block: {removals:?}");
            assert!(removals.contains(&(72, 4)));
            assert!(!removals.contains(&(0, 4)));
        }
        Effect::Place { .. } => panic!("expected eliminations"),
    }
}

#[test]
fn claiming_eliminating_outside_line_fires() {
    let mut spec: Vec<(usize, u16)> = vec![(0, b(4) | b(7)), (1, b(4) | b(8))];
    for col in 2..9 {
        spec.push((col, without(4)));
    }
    match locked_candidates(&cands(&spec)).expect("claiming applies") {
        Effect::Eliminate { removals } => {
            assert!(removals.contains(&(10, 4)), "block below row: {removals:?}");
            assert!(removals.contains(&(20, 4)));
            assert!(!removals.contains(&(0, 4)));
        }
        Effect::Place { .. } => panic!("expected eliminations"),
    }
}

#[test]
fn naked_pair_strips_pair_digits_from_the_rest_of_the_row() {
    let mut spec: Vec<(usize, u16)> = vec![(0, b(2) | b(3)), (1, b(2) | b(3))];
    for col in 2..9 {
        spec.push((col, b(1) | b(2) | b(3)));
    }
    match naked_set(&cands(&spec)).expect("naked pair applies") {
        Effect::Eliminate { removals } => {
            assert!(removals.contains(&(2, 2)) && removals.contains(&(2, 3)));
            assert!(removals.contains(&(8, 2)) && removals.contains(&(8, 3)));
            assert_eq!(removals.len(), 14, "7 cells x 2 digits: {removals:?}");
        }
        Effect::Place { .. } => panic!("expected eliminations"),
    }
}

#[test]
fn hidden_pair_strips_intruders_from_the_pair_cells() {
    let mut spec: Vec<(usize, u16)> = vec![(0, b(2) | b(3) | b(5)), (1, b(2) | b(3) | b(5))];
    for col in 2..9 {
        spec.push((col, 0x1FF & !(b(2) | b(3))));
    }
    match hidden_set(&cands(&spec)).expect("hidden pair applies") {
        Effect::Eliminate { removals } => {
            assert!(removals.contains(&(0, 5)) && removals.contains(&(1, 5)));
            assert_eq!(removals.len(), 2, "only the stray 5s: {removals:?}");
        }
        Effect::Place { .. } => panic!("expected eliminations"),
    }
}

#[test]
fn hidden_pair_with_an_absent_digit_does_not_fire() {
    let mut spec: Vec<(usize, u16)> = vec![(0, b(2) | b(5)), (1, b(2) | b(5))];
    for col in 2..9 {
        spec.push((col, 0x1FF & !(b(2) | b(3))));
    }
    assert_eq!(hidden_set(&cands(&spec)), None);
}
