//! Menu screen: difficulty selection, the stats view, and the
//! help overlay dialog.

use std::collections::BTreeMap;
use suduko_engine::Level;
use suduko_game::Game;
use yew::html::Scope;
use yew::prelude::*;

use crate::app::{Model, Msg};

/// Fresh time-derived seed so the next board differs from the last.
pub(crate) fn next_seed() -> u64 {
    js_sys::Date::now() as u64
}

fn label(level: Level) -> &'static str {
    match level {
        Level::Easy => "Easy",
        Level::Medium => "Medium",
        Level::Hard => "Hard",
        Level::Harder => "Harder",
        Level::Hardest => "Hardest",
    }
}

fn levels() -> [Level; 5] {
    [
        Level::Easy,
        Level::Medium,
        Level::Hard,
        Level::Harder,
        Level::Hardest,
    ]
}

fn stats_line(stats: &BTreeMap<Level, u32>) -> String {
    levels()
        .iter()
        .map(|&l| format!("{}: {}", label(l), stats.get(&l).copied().unwrap_or(0)))
        .collect::<Vec<_>>()
        .join("  ")
}

pub fn view_menu(link: &Scope<Model>, stats: &BTreeMap<Level, u32>) -> Html {
    html! {
        <main class="menu" data-testid="menu">
            <h1>{ "Suduko" }</h1>
            <p class="stats" data-testid="stats">{ stats_line(stats) }</p>
            <div class="levels">
                { for levels().iter().map(|&l| {
                    let start = link.callback(move |_| Msg::Start(l));
                    html! {
                        <button
                            class="level-btn"
                            data-testid={ format!("start-{}", label(l).to_lowercase()) }
                            onclick={ start }>
                            { label(l) }
                        </button>
                    }
                }) }
            </div>
            <p class="hint">{ "Pick a difficulty to start." }</p>
        </main>
    }
}

/// Generates (with the documented seed walk) and starts a game.
pub(crate) fn start_game(level: Level, seed: u64) -> Game {
    let mut seed = seed;
    let puzzle = loop {
        match suduko_engine::generate(level, seed) {
            Ok(p) => break p,
            Err(_) => seed += 977,
        }
    };
    Game::from_puzzle(&puzzle)
}

/// The help dialog rendered above the live board; the game keeps
/// running underneath. Closing returns to the untouched board.
pub fn help_overlay(on_close: Callback<()>) -> Html {
    html! {
        <div class="overlay help-overlay" data-testid="help-overlay" onclick={ on_close.reform(|_| ()) }>
            <div
                class="overlay-card help-card"
                data-testid="help-card"
                onclick={ |e: MouseEvent| e.stop_propagation() }>
                <h2>{ "How to play" }</h2>
                <ul>
                    <li>{ "Tap or click a cell to select it." }</li>
                    <li>{ "Enter a digit: type 1-9 or use the pad below the board." }</li>
                    <li>{ "Erase: spacebar, Backspace, or Delete - or the Erase pad button." }</li>
                    <li>{ "Wrong entries turn red and count in the bad counter." }</li>
                    <li>{ "A pad digit grays out once all nine of it are placed correctly." }</li>
                </ul>
                <h2>{ "Learn mode" }</h2>
                <ul>
                    <li>{ "Learn lists every strategy available on the current board - singles, pointing/claiming, pairs, X-Wing, XY-Wing." }</li>
                    <li>{ "Pick one to walk it step by step; pencil marks appear in empty cells while learning." }</li>
                    <li>{ "Pattern cells show outlined blue, involved rows/columns/blocks tint green, eliminations pulse red with strike-through, placements pulse green." }</li>
                </ul>
                <h2>{ "Show me" }</h2>
                <ul>
                    <li>{ "Show me has the game solve the board itself, explaining each strategy as it applies it." }</li>
                    <li>{ "Auto advances at the selected pace (1s/3s/6s); pressing Next pauses Auto for manual stepping." }</li>
                    <li>{ "When the taught techniques run out, a trial placement is explained and applied so the board still solves to the end." }</li>
                </ul>
                <h2>{ "Difficulty" }</h2>
                <ul>
                    <li>{ "Easy through Hardest need progressively rarer techniques (measured by the generator's difficulty bands)." }</li>
                </ul>
                <button
                    class="level-btn"
                    data-testid="help-close"
                    onclick={ on_close.reform(|_| ()) }>
                    { "Back to the board" }
                </button>
            </div>
        </div>
    }
}
