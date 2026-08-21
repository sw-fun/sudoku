//! Teaching-annotation tests on one verified mid-game board: the
//! Wikipedia easy puzzle with its first five empty cells solved.
//! Expected geometry was established independently (candidate
//! computation by hand-run script) before implementation.

use suduko_grid::CELL_COUNT;
use suduko_tutor::{
    Effect, Strategy, UnitRef, candidates, claiming_all, find_all_in, hidden_pairs, hidden_singles,
    naked_pairs, naked_singles, pointing_all, x_wings, xy_wings,
};

/// Wikipedia easy clues plus solution fills at idx 2,3,5,6,7.
const BOARD: &str =
    "53467891.6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";

fn shown(board: &str) -> [u8; CELL_COUNT] {
    let mut out = [0; CELL_COUNT];
    for (idx, ch) in board.bytes().enumerate() {
        out[idx] = ch.saturating_sub(b'0');
    }
    out
}

fn removals(effect: &Effect) -> &[(usize, u8)] {
    match effect {
        Effect::Eliminate { removals } => removals,
        Effect::Place { .. } => &[],
    }
}

#[test]
fn naked_singles_place_the_last_candidate() {
    let anns = naked_singles(&candidates(&shown(BOARD)));
    assert!(!anns.is_empty(), "mid-game board has naked singles");
    for a in &anns {
        let Effect::Place { idx, digit } = a.effect else {
            panic!("singles place, got {:?}", a.effect);
        };
        assert_eq!(a.pattern, vec![idx]);
        assert_eq!(a.digits, vec![digit]);
        assert!(!a.steps.is_empty(), "every annotation teaches in steps");
    }
}

#[test]
fn hidden_singles_confine_a_digit_to_one_cell_of_a_unit() {
    let anns = hidden_singles(&candidates(&shown(BOARD)));
    assert!(!anns.is_empty());
    for a in &anns {
        let Effect::Place { idx, digit } = a.effect else {
            panic!("hidden singles place, got {:?}", a.effect);
        };
        assert_eq!(a.pattern, vec![idx]);
        assert_eq!(a.digits, vec![digit]);
        assert!(
            a.units.len() == 1
                && a.steps
                    .iter()
                    .any(|s| s.text.contains("only") && s.digits.contains(&digit)),
            "steps explain the confinement: {a:?}"
        );
    }
}

#[test]
fn pointing_confines_a_block_digit_to_one_line() {
    let anns = pointing_all(&candidates(&shown(BOARD)));
    let a = anns
        .iter()
        .find(|a| {
            a.strategy == Strategy::Pointing && a.digits == vec![7] && a.pattern == vec![10, 11]
        })
        .expect("7s of block 1 sit only in row 2");
    assert!(a.units.contains(&UnitRef::Block(0)));
    assert!(a.units.contains(&UnitRef::Row(1)));
    let r = removals(&a.effect);
    assert!(r.contains(&(15, 7)) && r.contains(&(17, 7)), "{r:?}");
}

#[test]
fn claiming_confines_a_line_digit_to_one_block() {
    let anns = claiming_all(&candidates(&shown(BOARD)));
    let a = anns
        .iter()
        .find(|a| {
            a.strategy == Strategy::Claiming && a.digits == vec![3] && a.pattern == vec![54, 63, 72]
        })
        .expect("3s of column 1 sit only in block 7");
    assert!(a.units.contains(&UnitRef::Col(0)));
    assert!(a.units.contains(&UnitRef::Block(6)));
    let r = removals(&a.effect);
    assert!(
        r.contains(&(56, 3)) && r.contains(&(65, 3)) && r.contains(&(74, 3)),
        "{r:?}"
    );
}

#[test]
fn naked_pairs_strip_pair_digits_from_shared_units() {
    let anns = naked_pairs(&candidates(&shown(BOARD)));
    let row = anns
        .iter()
        .find(|a| a.units.contains(&UnitRef::Row(1)) && a.pattern == vec![10, 11])
        .expect("row 2 holds the {2,7} pair at r2c2/r2c3");
    assert_eq!(row.digits, vec![2, 7]);
    let r = removals(&row.effect);
    assert!(
        r.contains(&(15, 7)) && r.contains(&(16, 2)) && r.contains(&(17, 7)),
        "{r:?}"
    );
    let block = anns
        .iter()
        .find(|a| a.units.contains(&UnitRef::Block(0)) && a.pattern == vec![10, 11])
        .expect("block 1 shares the same pair");
    assert!(removals(&block.effect).contains(&(18, 2)));
}

#[test]
fn hidden_pairs_strip_extras_from_the_pair_cells() {
    let anns = hidden_pairs(&candidates(&shown(BOARD)));
    let a = anns
        .iter()
        .find(|a| a.digits == vec![1, 9] && a.pattern == vec![54, 56])
        .expect("1s and 9s of row 7 confined to r7c1/r7c3");
    assert!(a.units.contains(&UnitRef::Row(6)));
    let r = removals(&a.effect);
    assert!(
        r.contains(&(54, 3)) && r.contains(&(56, 5)) && r.contains(&(56, 7)),
        "{r:?}"
    );
}

#[test]
fn x_wing_corners_confine_a_digit_to_two_lines() {
    let anns = x_wings(&candidates(&shown(BOARD)));
    let a = anns
        .iter()
        .find(|a| a.digits == vec![2] && a.pattern == vec![21, 23, 75, 77])
        .expect("2-x-wing on columns 4/6 across rows 3/9");
    assert!(a.units.contains(&UnitRef::Col(3)) && a.units.contains(&UnitRef::Col(5)));
    let r = removals(&a.effect);
    for &(cell, digit) in &[(18, 2), (26, 2), (72, 2), (73, 2), (74, 2)] {
        assert!(
            r.contains(&(cell, digit)),
            "missing ({cell},{digit}): {r:?}"
        );
    }
}

#[test]
fn xy_wing_pincers_share_z_which_leaves_their_common_peers() {
    let anns = xy_wings(&candidates(&shown(BOARD)));
    let a = anns
        .iter()
        .find(|a| a.pattern == vec![21, 22, 23] && a.digits == vec![2, 3, 4])
        .expect("row 3 holds the {2,3,4} wing");
    let r = removals(&a.effect);
    assert!(r.contains(&(24, 4)) && r.contains(&(26, 4)), "{r:?}");
}

#[test]
fn find_all_orders_by_ladder_rung_and_dedupes() {
    let anns = find_all_in(&candidates(&shown(BOARD)));
    assert!(anns.len() > 10, "this board is rich: {}", anns.len());
    let rungs: Vec<u32> = anns.iter().map(|a| a.strategy.rung()).collect();
    let mut sorted = rungs.clone();
    sorted.sort_unstable();
    assert_eq!(rungs, sorted, "ladder-ordered");
    let mut seen = std::collections::HashSet::new();
    for a in &anns {
        assert!(
            seen.insert((a.strategy, a.pattern.clone(), a.digits.clone(), &a.effect)),
            "exact duplicates collapse: {} {a:?}",
            a.strategy.name()
        );
    }
    for strategy in [
        Strategy::NakedSingle,
        Strategy::HiddenSingle,
        Strategy::Pointing,
        Strategy::Claiming,
        Strategy::NakedPair,
        Strategy::HiddenPair,
        Strategy::XWing,
        Strategy::XYWing,
    ] {
        assert!(
            anns.iter().any(|a| a.strategy == strategy),
            "{} present in find_all",
            strategy.name()
        );
    }
}
