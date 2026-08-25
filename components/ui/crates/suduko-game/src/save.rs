//! Versioned save-slot codec: one slot holding the level, per-level
//! stats, and an optional in-progress game. Pure string <-> data both
//! ways; every malformed input fails closed to `None`.

use super::Game;
use std::collections::BTreeMap;
use suduko_grid::CELL_COUNT;

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

/// Encodes the save slot:
/// `v3|level|clues|solution|user|elapsed|bad|pencil|marks|stats`
/// where the three boards are 81-char digit strings (0 = empty),
/// pencil is 0/1, marks is `idx=hexmask;` pairs for non-empty user
/// notes, and stats is `idx=count;...`. `game = None` writes empty
/// boards with pencil off.
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
    let (pencil, marks) = game.map_or((0u8, String::new()), |g| {
        let marks = g
            .user_marks
            .iter()
            .enumerate()
            .filter(|&(_, &m)| m != 0)
            .map(|(i, &m)| format!("{i}={m:x};"))
            .collect::<String>();
        (u8::from(g.pencil), marks)
    });
    let stats_text = stats
        .iter()
        .map(|(l, n)| format!("{l}={n}"))
        .collect::<Vec<_>>()
        .join(";");
    format!("v3|{level}|{clues}|{solution}|{user}|{elapsed}|{bad}|{pencil}|{marks}|{stats_text}")
}

/// Decodes a slot written by [`save`]; anything malformed fails
/// closed to `None`. v1 slots restore with defaults; v2 User slots
/// map to pencil on, v2 Auto slots to pencil off plus a computed-
/// candidate fill. A solved board restores without a game.
pub fn restore(text: &str) -> Option<Save> {
    let parts: Vec<&str> = text.split('|').collect();
    let ver = match (parts.len(), parts[0]) {
        (8, "v1") => 1,
        (10, "v2") => 2,
        (10, "v3") => 3,
        _ => return None,
    };
    let level = parts[1].parse::<u8>().ok()?;
    (level <= 4).then_some(())?;
    let boards = [cells(parts[2])?, cells(parts[3])?, cells(parts[4])?];
    let (elapsed, bad) = (parts[5].parse::<u32>().ok()?, parts[6].parse::<u32>().ok()?);
    let (pencil, auto_fill) = pencil_state(ver, parts[7])?;
    let marks = if ver == 1 {
        [0u16; CELL_COUNT]
    } else {
        marks(parts[8])?
    };
    let stats = kv_pairs(parts[if ver == 1 { 7 } else { 9 }])?;
    let has_game = boards[1].iter().any(|&d| d != 0);
    let solved = (0..CELL_COUNT).all(|i| boards[1][i] != 0 && boards[2][i] == boards[1][i]);
    let game = (has_game && !solved)
        .then(|| restored_game(&boards, elapsed, bad, pencil, auto_fill, marks));
    Some(Save { level, stats, game })
}

/// Parses an 81-char digit board (0 = empty).
fn cells(text: &str) -> Option<[u8; CELL_COUNT]> {
    let b = text.as_bytes();
    (b.len() == CELL_COUNT && b.iter().all(u8::is_ascii_digit)).then(|| {
        let mut out = [0u8; CELL_COUNT];
        for (slot, &d) in out.iter_mut().zip(b) {
            *slot = d - b'0';
        }
        out
    })
}

/// `idx=hexmask;` pairs into the 81-slot user-notes layer.
fn marks(text: &str) -> Option<[u16; CELL_COUNT]> {
    let mut out = [0u16; CELL_COUNT];
    for pair in text.split(';').filter(|p| !p.is_empty()) {
        let (i, m) = pair.split_once('=')?;
        *out.get_mut(i.parse::<usize>().ok()?)? = u16::from_str_radix(m, 16).ok()? & 0x1ff;
    }
    Some(out)
}

/// `k=v;` pairs into a map (stats or the shared parser shape).
fn kv_pairs(text: &str) -> Option<BTreeMap<u8, u32>> {
    let mut out = BTreeMap::new();
    for pair in text.split(';').filter(|p| !p.is_empty()) {
        let (k, v) = pair.split_once('=')?;
        out.insert(k.parse::<u8>().ok()?, v.parse::<u32>().ok()?);
    }
    (!out.keys().any(|&k| k > 4)).then_some(out)
}

/// (pencil, auto_fill) from the notes field of a v2/v3 slot.
fn pencil_state(ver: u8, field: &str) -> Option<(bool, bool)> {
    match (ver, field) {
        (1, _) => Some((false, false)),
        (3, "0") | (2, "0") => Some((false, false)),
        (3, "1") | (2, "1") => Some((true, false)),
        (2, "2") => Some((false, true)),
        _ => None,
    }
}

/// Rebuilds a fresh-but-populated game from decoded save fields.
fn restored_game(
    boards: &[[u8; CELL_COUNT]; 3],
    elapsed: u32,
    bad: u32,
    pencil: bool,
    auto_fill: bool,
    mut marks: [u16; CELL_COUNT],
) -> Game {
    if auto_fill {
        let vals = core::array::from_fn::<u8, CELL_COUNT, _>(|i| {
            if boards[0][i] != 0 {
                boards[0][i]
            } else {
                boards[2][i]
            }
        });
        let cands = suduko_tutor::candidates_with(&vals, &[]);
        for (m, mask) in marks.iter_mut().zip(cands.masks) {
            *m = mask & 0x1ff;
        }
    }
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
        pencil,
        user_marks: marks,
        keypad_open: false,
    }
}
