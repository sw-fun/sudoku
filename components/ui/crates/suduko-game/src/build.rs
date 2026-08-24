//! Game construction from engine puzzles and serialized strings, plus
//! the versioned save codec (one slot: level, stats, optional
//! in-progress game).

use super::{BuildError, Game};
use std::collections::BTreeMap;
use suduko_grid::{Board, CELL_COUNT, Cell, Puzzle};

/// One save slot, decoded: the chosen level, per-level win counts
/// (keyed by level index 0..=4), and the in-progress game if any.
/// Won boards never restore a game - only stats survive them.
#[derive(Default)]
pub struct Save {
    /// Level index 0..=4 (Easy..Hardest) the saved game was started at.
    pub level: u8,
    /// Wins per level index.
    pub stats: BTreeMap<u8, u32>,
    /// The in-progress game, if the slot holds one.
    pub game: Option<Game>,
}

/// Builds a game from 81-char clue/solution strings ('.' or '0' empty).
///
/// # Errors
///
/// Returns `BuildError::BadStrings` when the strings do not parse or do
/// not form a valid puzzle; the UI only feeds it engine output.
pub fn from_strings(clues: &str, solution: &str) -> Result<Game, BuildError> {
    let (clues, solution) = (
        parse(clues).ok_or(BuildError::BadStrings)?,
        parse(solution).ok_or(BuildError::BadStrings)?,
    );
    let puzzle = Puzzle::new(clues, solution).map_err(|_| BuildError::BadStrings)?;
    Ok(from_puzzle(&puzzle))
}

/// Builds a fresh, empty game from a validated engine puzzle.
pub fn from_puzzle(puzzle: &Puzzle) -> Game {
    Game {
        clues: digits(puzzle.clues()),
        solution: digits(puzzle.solution()),
        user: [0; CELL_COUNT],
        selected: None,
        bad_inputs: 0,
        won: false,
        elapsed_secs: 0,
        teaching: super::Teaching::default(),
        show_me: false,
        show_me_auto: false,
        show_me_delay_ticks: 0,
        show_me_wait: 0,
        eliminated: Vec::new(),
        notes_mode: false,
        keypad_open: false,
    }
}

fn digits(board: &Board) -> [u8; CELL_COUNT] {
    let mut out = [0u8; CELL_COUNT];
    for (idx, slot) in out.iter_mut().enumerate() {
        if let Cell::Value(v) = board.get(idx) {
            *slot = v;
        }
    }
    out
}

fn parse(text: &str) -> Option<Board> {
    suduko_grid::parse(text).ok()
}

/// Encodes the save slot: `v1|level|clues|solution|user|elapsed|bad|stats`
/// where the three boards are 81-char digit strings (0 = empty) and
/// stats is `idx=count;...`. `game = None` writes empty boards.
pub fn save(level: u8, game: Option<&Game>, stats: &BTreeMap<u8, u32>) -> String {
    let enc = |b: &[u8; CELL_COUNT]| -> String { b.iter().map(|&d| (b'0' + d) as char).collect() };
    let (clues, solution, user, elapsed, bad) = game.map_or(
        (
            "0".repeat(CELL_COUNT),
            "0".repeat(CELL_COUNT),
            "0".repeat(CELL_COUNT),
            0,
            0,
        ),
        |g| {
            (
                enc(&g.clues),
                enc(&g.solution),
                enc(&g.user),
                g.elapsed_secs,
                g.bad_inputs,
            )
        },
    );
    let stats_text = stats
        .iter()
        .map(|(l, n)| format!("{l}={n}"))
        .collect::<Vec<_>>()
        .join(";");
    format!("v1|{level}|{clues}|{solution}|{user}|{elapsed}|{bad}|{stats_text}")
}

/// Decodes a save slot written by [`save`]; anything malformed (bad
/// version, wrong field count, bad lengths or charset, level index
/// out of range) fails closed to `None`. A board already fully and
/// correctly solved restores without a game.
pub fn restore(text: &str) -> Option<Save> {
    let parts: Vec<&str> = text.split('|').collect();
    if parts.len() != 8 || parts[0] != "v1" {
        return None;
    }
    let level = parts[1].parse::<u8>().ok()?;
    if level > 4 {
        return None;
    }
    let cells = |t: &str| -> Option<[u8; CELL_COUNT]> {
        let b = t.as_bytes();
        (b.len() == CELL_COUNT && b.iter().all(u8::is_ascii_digit)).then(|| {
            let mut out = [0u8; CELL_COUNT];
            for (i, &d) in b.iter().enumerate() {
                out[i] = d - b'0';
            }
            out
        })
    };
    let boards = [cells(parts[2])?, cells(parts[3])?, cells(parts[4])?];
    let elapsed = parts[5].parse::<u32>().ok()?;
    let bad = parts[6].parse::<u32>().ok()?;
    let mut stats = BTreeMap::new();
    if !parts[7].is_empty() {
        for pair in parts[7].split(';') {
            let (l, n) = pair.split_once('=')?;
            let (l, n) = (l.parse::<u8>().ok()?, n.parse::<u32>().ok()?);
            if l > 4 {
                return None;
            }
            stats.insert(l, n);
        }
    }
    let has_game = boards[1].iter().any(|&d| d != 0);
    let solved = (0..CELL_COUNT).all(|i| boards[1][i] != 0 && boards[2][i] == boards[1][i]);
    let game = (has_game && !solved).then(|| restored_game(&boards, elapsed, bad));
    Some(Save { level, stats, game })
}

/// Rebuilds a fresh-but-populated game from decoded save fields.
fn restored_game(boards: &[[u8; CELL_COUNT]; 3], elapsed: u32, bad: u32) -> Game {
    Game {
        clues: boards[0],
        solution: boards[1],
        user: boards[2],
        selected: None,
        bad_inputs: bad,
        won: false,
        elapsed_secs: elapsed,
        teaching: super::Teaching::default(),
        show_me: false,
        show_me_auto: false,
        show_me_delay_ticks: 0,
        show_me_wait: 0,
        eliminated: Vec::new(),
        notes_mode: false,
        keypad_open: false,
    }
}
