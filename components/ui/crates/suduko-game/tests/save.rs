use std::collections::BTreeMap;
use suduko_game::{Game, restore, save};

/// Wikipedia easy puzzle (singles-solvable) and its solution.
const CLUES: &str =
    "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
const SOLUTION: &str =
    "534678912672195348198342567859761423426853791713924856961537284287419635345286179";

fn game() -> Game {
    suduko_game::from_strings(CLUES, SOLUTION).expect("fixture agrees")
}

fn stats() -> BTreeMap<u8, u32> {
    [(0u8, 3u32), (2, 1)].into_iter().collect()
}

#[test]
fn in_progress_game_round_trips_with_level_and_stats() {
    let mut g = game();
    g.elapsed_secs = 253;
    g.bad_inputs = 4;
    g.user[2] = 9;
    g.user[5] = 1;
    let code = save(1, Some(&g), &stats());
    let back = restore(&code).expect("valid code restores");
    assert_eq!(back.level, 1);
    let rg = back.game.expect("game saved");
    assert_eq!((rg.elapsed_secs, rg.bad_inputs), (253, 4));
    assert_eq!(rg.user[2], 9);
    assert_eq!(rg.clues, g.clues);
    assert_eq!(rg.solution, g.solution);
    assert!(!rg.won, "an in-progress board is not won");
    assert_eq!(back.stats, stats());
}

#[test]
fn stats_only_round_trips_without_a_game() {
    let code = save(4, None, &stats());
    let back = restore(&code).expect("valid code restores");
    assert_eq!(back.level, 4, "level rides along even without a game");
    assert!(back.game.is_none());
    assert_eq!(back.stats, stats());
}

#[test]
fn empty_stats_round_trip() {
    let code = save(0, None, &BTreeMap::new());
    let back = restore(&code).expect("valid code restores");
    assert!(back.game.is_none());
    assert!(back.stats.is_empty());
}

#[test]
fn corrupt_codes_fail_closed_to_none() {
    let good = save(1, Some(&game()), &stats());
    for bad in [
        String::new(),
        good.replace("v2|", "v9|"),
        good.replace('1', "x"),                // bad digit charset
        good[..good.len() - 5].to_string(),    // truncated tail
        String::from("v2|9|1|1|1|0|0|0||0=0"), // level out of range
    ] {
        assert!(
            restore(&bad).is_none(),
            "corrupt code must fail closed: {bad:?}"
        );
    }
}

#[test]
fn a_won_board_restores_as_no_game() {
    let mut g = game();
    for (i, d) in SOLUTION.bytes().enumerate() {
        g.user[i] = d - b'0';
    }
    g.won = g.is_won();
    assert!(g.won);
    let code = save(2, Some(&g), &stats());
    let back = restore(&code).expect("valid code restores");
    assert!(back.game.is_none(), "won boards never offer resume");
    assert_eq!(back.stats, stats(), "stats survive");
}
