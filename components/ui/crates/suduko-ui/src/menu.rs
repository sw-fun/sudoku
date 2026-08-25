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
pub fn customize_popup(mode: InputMode, notes_visible: bool, link: &Scope<Model>) -> Html {
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
                { notes_section(notes_visible, link) }
                <button class="level-btn danger" data-testid="clear-stats" onclick={ on_clear_stats.reform(|_| ()) }>
                    { "Clear stats" }
                </button>
            </div>
        </div>
    }
}

/// The notes-management section of the customize dialog: show/hide
/// the player's notes, fill computed candidates everywhere, or wipe
/// the whole layer.
fn notes_section(notes_visible: bool, link: &Scope<Model>) -> Html {
    html! {
        <>
            <h2>{ "My notes" }</h2>
            <label class="auto-label">
                <input
                    type="checkbox"
                    checked={ notes_visible }
                    onchange={ link.callback(|e: Event| {
                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                        Msg::NotesShow(input.checked())
                    }) } />
                { " Show my notes on the board" }
            </label>
            <div class="customize-row">
                <button class="mode-btn" data-testid="fill-notes" onclick={ link.callback(|_| Msg::NotesFill) }>
                    { "Fill in all notes" }
                </button>
                <button class="mode-btn" data-testid="clear-notes" onclick={ link.callback(|_| Msg::NotesClear) }>
                    { "Clear all notes" }
                </button>
            </div>
        </>
    }
}
