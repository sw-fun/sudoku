mod app;
mod board;
pub mod keys;
mod learn;
mod menu;

use std::collections::BTreeMap;
use suduko_engine::Level;
use suduko_game::Game;
use yew::html::Scope;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! { <app::Model /> }
}

/// The help dialog rendered above the live board; the game keeps
/// running underneath. Closing returns to the untouched board.
pub(crate) fn help_overlay(link: &yew::html::Scope<app::Model>) -> Html {
    let on_close = link.callback(|_| app::Msg::HelpToggle);
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
                <h2>{ "Notes (pencil marks)" }</h2>
                <ul>
                    <li>{ "The header Notes button is a mode switch: off = digit keys place values, on = digit keys write pencil notes. It stays where you leave it, so you can keep your notes visible while entering values." }</li>
                    <li>{ "Notes: on - select an empty cell and type (or tap) a digit to pencil it in; type it again to remove it. Only digits the rules still allow can be penciled. Erase clears the selected cell's notes." }</li>
                    <li>{ "Your notes are always shown on the board by default. Customize lets you hide them without losing them (Show my notes), fill every empty cell with its computed candidates in one tap (Fill in all notes), or wipe the whole layer (Clear all notes)." }</li>
                    <li>{ "Placing a value - by hand or by show-me - automatically deletes that digit's note from every cell in its row, column, and box." }</li>
                    <li>{ "Notes are saved with the game, so they survive reloads and Resume." }</li>
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

/// Writes the single save slot: stats always, the board only while
/// in progress (won boards clear the game part).
pub(crate) fn persist_slot(level: Level, game: Option<&Game>, stats: &BTreeMap<Level, u32>) {
    let idx = |l: Level| {
        u8::try_from(menu::LEVELS.iter().position(|&x| x == l).unwrap_or(0)).unwrap_or(0)
    };
    let stats = stats
        .iter()
        .map(|(l, n)| (idx(*l), *n))
        .collect::<BTreeMap<_, _>>();
    let live = game.filter(|g| !g.won);
    if let Some(store) = menu::storage() {
        store
            .set_item("sudoku-save", &suduko_game::save(idx(level), live, &stats))
            .ok();
    }
}

/// The abandon-progress confirmation shown when leaving an
/// unfinished board or starting a new one over it. Yes discards the
/// board (stats are kept); No, the X, or clicking outside keeps it.
pub(crate) fn abandon_dialog(link: &Scope<app::Model>) -> Html {
    let answer = link.callback(app::Msg::Abandon);
    html! {
        <div class="overlay" data-testid="abandon-overlay" onclick={ answer.reform(|_| false) }>
            <div class="overlay-card custom-card" onclick={ |e: MouseEvent| e.stop_propagation() }>
                <button class="close-x" data-testid="abandon-close" onclick={ answer.reform(|_| false) }>
                    { "x" }
                </button>
                <h2>{ "Abandon this board?" }</h2>
                <p class="dialog-hint">{ "Your progress on this board will be lost (stats are kept)." }</p>
                <div class="customize-row">
                    <button class="level-btn danger" data-testid="abandon-yes" onclick={ answer.reform(|_| true) }>
                        { "Abandon" }
                    </button>
                    <button class="level-btn" data-testid="abandon-no" onclick={ answer.reform(|_| false) }>
                        { "Keep playing" }
                    </button>
                </div>
            </div>
        </div>
    }
}
