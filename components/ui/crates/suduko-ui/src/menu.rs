//! Menu screen (difficulty selection, stats, resume, help overlay),
//! the customize and abandon dialogs, and the localStorage handle.

use std::collections::BTreeMap;
use suduko_engine::Level;
use suduko_game::Game;
use suduko_uikit::InputMode;
use yew::html::Scope;
use yew::prelude::*;

use crate::app::{Model, Msg};

/// The five levels in published (band) order; index is the save-slot
/// encoding of a level.
pub(crate) const LEVELS: [Level; 5] = [
    Level::Easy,
    Level::Medium,
    Level::Hard,
    Level::Harder,
    Level::Hardest,
];

/// Fresh time-derived seed so the next board differs from the last.
pub(crate) fn next_seed() -> u64 {
    js_sys::Date::now() as u64
}

/// localStorage handle for the save slot; `None` when storage is
/// unavailable (private mode, etc.) - the game then simply does not
/// persist.
pub(crate) fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?
}

pub fn view_menu(
    link: &Scope<Model>,
    stats: &BTreeMap<Level, u32>,
    resume: Option<(&str, &str)>,
) -> Html {
    let line = LEVELS
        .iter()
        .map(|l| format!("{}: {}", l.label(), stats.get(l).copied().unwrap_or(0)))
        .collect::<Vec<_>>()
        .join("  ");
    html! {
        <main class="menu" data-testid="menu">
            <h1>{ "Learn/Practice Sudoku" }</h1>
            <p class="stats" data-testid="stats">{ line }</p>
            <div class="levels">
                { for LEVELS.iter().map(|&l| {
                    let start = link.callback(move |_| Msg::Start(l));
                    html! {
                        <button
                            class="level-btn"
                            data-testid={ format!("start-{}", l.label().to_lowercase()) }
                            onclick={ start }>
                            { l.label() }
                        </button>
                    }
                }) }
            </div>
            { if let Some((level_label, elapsed)) = resume {
                html! {
                    <button class="level-btn resume" data-testid="resume" onclick={ link.callback(|_| Msg::Resume) }>
                        { format!("Resume {level_label} board ({elapsed})") }
                    </button>
                }
            } else { html! {} } }
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

/// The customize dialog: choose where the digit input lives and clear
/// the saved stats. Clicking outside or the X (or Escape) closes it.
pub fn customize_popup(mode: InputMode, link: &Scope<Model>) -> Html {
    let on_mode = link.callback(Msg::InputModeSet);
    let on_clear_stats = link.callback(|_| Msg::ClearStats);
    let on_close = link.callback(|_| Msg::CustomizeToggle);
    let option = |m: InputMode| {
        let on = mode == m;
        let text = match m {
            InputMode::Above => "Pad above board",
            InputMode::Below => "Pad below board",
            InputMode::Popup => "Popup keypad",
        };
        html! {
            <button
                class={ if on { "mode-btn on" } else { "mode-btn" } }
                data-testid={ format!("customize-{m}") }
                onclick={ on_mode.reform(move |_| m) }>
                { text }
            </button>
        }
    };
    html! {
        <div class="overlay" data-testid="customize-overlay" onclick={ on_close.reform(|_| ()) }>
            <div class="overlay-card custom-card" onclick={ |e: MouseEvent| e.stop_propagation() }>
                <button class="close-x" data-testid="customize-close" onclick={ on_close.reform(|_| ()) }>
                    { "x" }
                </button>
                <h2>{ "Customize" }</h2>
                <p class="dialog-hint">{ "Where should the number buttons live?" }</p>
                <div class="customize-row">
                    { option(InputMode::Above) }
                    { option(InputMode::Below) }
                    { option(InputMode::Popup) }
                </div>
                <button class="level-btn danger" data-testid="clear-stats" onclick={ on_clear_stats.reform(|_| ()) }>
                    { "Clear stats" }
                </button>
            </div>
        </div>
    }
}

/// The abandon-progress confirmation shown when leaving an
/// unfinished board. Yes discards the board (stats are kept); No,
/// the X, or clicking outside keeps playing.
pub fn abandon_dialog(link: &Scope<Model>) -> Html {
    let on_answer = link.callback(Msg::Abandon);
    html! {
        <div class="overlay" data-testid="abandon-overlay" onclick={ on_answer.reform(|_| false) }>
            <div class="overlay-card custom-card" onclick={ |e: MouseEvent| e.stop_propagation() }>
                <button class="close-x" data-testid="abandon-close" onclick={ on_answer.reform(|_| false) }>
                    { "x" }
                </button>
                <h2>{ "Abandon this board?" }</h2>
                <p class="dialog-hint">{ "Your progress on this board will be lost (stats are kept)." }</p>
                <div class="customize-row">
                    <button class="level-btn danger" data-testid="abandon-yes" onclick={ on_answer.reform(|_| true) }>
                        { "Abandon" }
                    </button>
                    <button class="level-btn" data-testid="abandon-no" onclick={ on_answer.reform(|_| false) }>
                        { "Keep playing" }
                    </button>
                </div>
            </div>
        </div>
    }
}

/// The help dialog rendered above the live board; the game keeps
/// running underneath. Closing returns to the untouched board.
pub fn help_overlay(link: &Scope<Model>) -> Html {
    let on_close = link.callback(|_| Msg::HelpToggle);
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
                <h2>{ "Pencil marks" }</h2>
                <ul>
                    <li>{ "Notes shows candidate marks in every empty cell while you play." }</li>
                    <li>{ "With Notes on, typing (or tapping) a digit toggles that candidate in the selected cell; Erase restores the cell's computed candidates." }</li>
                    <li>{ "In Learn, Apply these removals applies one strategy's eliminations to your marks; the picker's Apply all does every listed elimination at once (placements stay yours to make)." }</li>
                    <li>{ "Reset marks clears all removals and returns to computed candidates." }</li>
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
