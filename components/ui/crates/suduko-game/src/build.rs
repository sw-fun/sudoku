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
        notes: suduko_uikit::NotesMode::Off,
        user_marks: [0; CELL_COUNT],
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

/// Encodes the save slot:
/// `v2|level|clues|solution|user|elapsed|bad|notes|marks|stats`
/// where the three boards are 81-char digit strings (0 = empty),
/// notes is 0/1/2 (off/user/auto), marks is `idx=hexmask;` pairs for
/// non-empty user marks, and stats is `idx=count;...`. `game = None`
/// writes empty boards with notes off.
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
    let (notes, marks) = game.map_or((0u8, String::new()), |g| {
        let marks = g
            .user_marks
            .iter()
            .enumerate()
            .filter(|&(_, &m)| m != 0)
            .map(|(i, &m)| format!("{i}={m:x};"))
            .collect::<String>();
        (g.notes as u8, marks)
    });
    let stats_text = stats
        .iter()
        .map(|(l, n)| format!("{l}={n}"))
        .collect::<Vec<_>>()
        .join(";");
    format!("v2|{level}|{clues}|{solution}|{user}|{elapsed}|{bad}|{notes}|{marks}|{stats_text}")
}

/// Decodes a save slot written by [`save`]; anything malformed (bad
/// version, wrong field count, bad lengths or charset, level index
/// out of range) fails closed to `None`. v1 slots (no notes fields)
/// restore with notes off and empty marks. A board already fully and
/// correctly solved restores without a game.
pub fn restore(text: &str) -> Option<Save> {
    let parts: Vec<&str> = text.split('|').collect();
    let v2 = parts.len() == 10 && parts[0] == "v2";
    let v1 = parts.len() == 8 && parts[0] == "v1";
    if !v1 && !v2 {
        return None;
    }
    let level = parts[1].parse::<u8>().ok()?;
    (level <= 4).then_some(())?;
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
    let (notes, mut marks) = (suduko_uikit::NotesMode::Off, [0u16; CELL_COUNT]);
    let (notes, marks) = if v2 {
        let notes = suduko_uikit::NotesMode::from_index(parts[7].parse().ok()?)?;
        for p in parts[8].split(';').filter(|p| !p.is_empty()) {
            let (i, m) = p.split_once('=')?;
            *marks.get_mut(i.parse::<usize>().ok()?)? = u16::from_str_radix(m, 16).ok()? & 0x1ff;
        }
        (notes, marks)
    } else {
        (notes, marks)
    };
    let mut stats = BTreeMap::new();
    for pair in parts[if v2 { 9 } else { 7 }]
        .split(';')
        .filter(|p| !p.is_empty())
    {
        let (l, n) = pair.split_once('=')?;
        stats.insert(l.parse::<u8>().ok()?, n.parse::<u32>().ok()?);
    }
    (!stats.keys().any(|&l| l > 4)).then_some(())?;
    let has_game = boards[1].iter().any(|&d| d != 0);
    let solved = (0..CELL_COUNT).all(|i| boards[1][i] != 0 && boards[2][i] == boards[1][i]);
    let game = (has_game && !solved).then(|| restored_game(&boards, elapsed, bad, notes, marks));
    Some(Save { level, stats, game })
}

/// Rebuilds a fresh-but-populated game from decoded save fields.
fn restored_game(
    boards: &[[u8; CELL_COUNT]; 3],
    elapsed: u32,
    bad: u32,
    notes: suduko_uikit::NotesMode,
    marks: [u16; CELL_COUNT],
) -> Game {
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
        notes,
        user_marks: marks,
        keypad_open: false,
    }
}
