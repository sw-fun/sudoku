//! Game screen: the board grid, input pad, timer, bad-input counter,
//! and the win overlay.

use suduko_engine::Level;
use suduko_game::{Game, highlight_set};
use yew::prelude::*;

fn label(level: Level) -> &'static str {
    match level {
        Level::Easy => "Easy",
        Level::Medium => "Medium",
        Level::Hard => "Hard",
        Level::Harder => "Harder",
        Level::Hardest => "Hardest",
    }
}

#[allow(clippy::too_many_arguments)]
pub fn view_board(
    game: &Game,
    level: Level,
    on_select: Callback<usize>,
    on_digit: Callback<u8>,
    on_erase: Callback<()>,
    on_menu: Callback<()>,
    on_next: Callback<()>,
) -> Html {
    let highlights = highlight_set(game);
    html! {
        <main class="game" data-testid="game">
            <header class="game-header">
                <span class="level" data-testid="level">{ label(level) }</span>
                <span class="timer" data-testid="timer">{ format_time(game.elapsed_secs) }</span>
                <span class="bad" data-testid="bad-count">{ format!("bad: {}", game.bad_inputs) }</span>
                <button class="menu-btn" data-testid="menu-btn" onclick={ on_menu.reform(|_| ()) }>
                    { "Menu" }
                </button>
            </header>
            { grid(game, &highlights, &on_select) }
            { pad(&on_digit, &on_erase) }
            if game.won {
                { overlay(game, &on_next, &on_menu) }
            }
        </main>
    }
}

fn grid(game: &Game, highlights: &[usize], on_select: &Callback<usize>) -> Html {
    html! {
        <div class="board" data-testid="board">
            { for (0..81).map(|idx| cell(game, idx, highlights, on_select)) }
        </div>
    }
}

fn cell(game: &Game, idx: usize, highlights: &[usize], on_select: &Callback<usize>) -> Html {
    let mut classes = vec!["cell"];
    if game.is_given(idx) {
        classes.push("given");
    } else if game.shown(idx) != 0 {
        classes.push("user");
    }
    if game.is_wrong(idx) {
        classes.push("wrong");
    }
    if game.selected == Some(idx) {
        classes.push("selected");
    }
    if highlights.contains(&idx) {
        classes.push("hl");
    }
    let shown = game.shown(idx);
    let text = if shown == 0 {
        " ".to_string()
    } else {
        shown.to_string()
    };
    html! {
        <button
            class={ classes.join(" ") }
            data-testid={ format!("cell-{idx}") }
            onclick={ on_select.reform(move |_| idx) }>
            { text }
        </button>
    }
}

fn pad(on_digit: &Callback<u8>, on_erase: &Callback<()>) -> Html {
    html! {
        <div class="pad" data-testid="pad">
            { for (1..=9u8).map(|d| html! {
                <button
                    class="pad-btn"
                    data-testid={ format!("pad-{d}") }
                    onclick={ on_digit.reform(move |_| d) }>
                    { d }
                </button>
            }) }
            <button class="pad-btn erase" data-testid="pad-erase" onclick={ on_erase.reform(|_| ()) }>
                { "Erase" }
            </button>
        </div>
    }
}

fn overlay(game: &Game, on_next: &Callback<()>, on_menu: &Callback<()>) -> Html {
    html! {
        <div class="overlay win-flash" data-testid="win-overlay">
            <div class="overlay-card">
                <h2>{ "Solved!" }</h2>
                <p data-testid="final-time">{ format!("time: {}", format_time(game.elapsed_secs)) }</p>
                <p>{ format!("bad inputs: {}", game.bad_inputs) }</p>
                <button class="level-btn" data-testid="next-board" onclick={ on_next.reform(|_| ()) }>
                    { "Next board" }
                </button>
                <button class="level-btn" data-testid="overlay-menu" onclick={ on_menu.reform(|_| ()) }>
                    { "Menu" }
                </button>
            </div>
        </div>
    }
}

fn format_time(secs: u32) -> String {
    format!("{:02}:{:02}", secs / 60, secs % 60)
}
