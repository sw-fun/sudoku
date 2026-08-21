//! Game screen: the board grid, input pad, timer, bad-input counter,
//! the win overlay, and the learn (teaching) panel.

use crate::learn::{marks_html, step_classes};
use suduko_engine::Level;
use suduko_game::{Game, StepView, digit_complete, highlight_set};
use yew::prelude::*;

fn cell(
    game: &Game,
    idx: usize,
    highlights: &[usize],
    on_select: &Callback<usize>,
    marks: Option<&[Vec<u8>]>,
    view: Option<&StepView>,
) -> Html {
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
    let step_classes = step_classes(view, idx);
    if step_classes.is_empty() {
        if highlights.contains(&idx) {
            classes.push("hl");
        }
    } else {
        classes.extend(step_classes);
    }
    let content = if game.shown(idx) != 0 {
        html! { { game.shown(idx).to_string() } }
    } else if let Some(marks) = marks {
        marks_html(idx, marks, view)
    } else {
        html! { { " " } }
    };
    html! {
        <button
            class={ classes.join(" ") }
            data-testid={ format!("cell-{idx}") }
            onclick={ on_select.reform(move |_| idx) }>
            { content }
        </button>
    }
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

#[allow(clippy::too_many_arguments)]
pub fn view_board(
    game: &Game,
    level: Level,
    on_select: Callback<usize>,
    on_digit: Callback<u8>,
    on_erase: Callback<()>,
    on_menu: Callback<()>,
    on_next: Callback<()>,
    on_learn: Callback<()>,
    on_pick: Callback<usize>,
    on_step: Callback<isize>,
) -> Html {
    let highlights = highlight_set(game);
    let teaching = game.teaching.panel_open;
    html! {
        <main class="game" data-testid="game">
            <header class="game-header">
                <span class="level" data-testid="level">{ label(level) }</span>
                <span class="timer" data-testid="timer">{ format_time(game.elapsed_secs) }</span>
                <span class="bad" data-testid="bad-count">{ format!("bad: {}", game.bad_inputs) }</span>
                <button class="menu-btn learn-btn" data-testid="learn-btn" onclick={ on_learn.reform(|_| ()) }>
                    { if teaching { "Hide learning" } else { "Learn" } }
                </button>
                <button class="menu-btn" data-testid="menu-btn" onclick={ on_menu.reform(|_| ()) }>
                    { "Menu" }
                </button>
            </header>
            { grid(game, &highlights, &on_select, teaching) }
            { if teaching { crate::learn::learn_panel(game, &on_learn, &on_pick, &on_step) } else { html! {} } }
            { pad(game, &on_digit, &on_erase) }
            if game.won {
                { overlay(game, &on_next, &on_menu) }
            }
        </main>
    }
}

fn grid(game: &Game, highlights: &[usize], on_select: &Callback<usize>, teaching: bool) -> Html {
    let marks = if teaching {
        Some(game.pencil_marks())
    } else {
        None
    };
    let view = if teaching { game.step_view() } else { None };
    let cells = (0..81).map(|idx| {
        cell(
            game,
            idx,
            highlights,
            on_select,
            marks.as_ref().map(|m| &m[..]),
            view.as_ref(),
        )
    });
    html! {
        <div class="board-wrap" data-testid="board-wrap">
            <span class="coord corner" aria-hidden="true"></span>
            <div class="col-labels" data-testid="col-labels">
                { for (1..=9).map(|n| html! { <span class="coord">{ n }</span> }) }
            </div>
            <div class="row-labels" data-testid="row-labels">
                { for (1..=9).map(|n| html! { <span class="coord">{ n }</span> }) }
            </div>
            <div class="board" data-testid="board">
                { for cells }
            </div>
        </div>
    }
}

fn pad(game: &Game, on_digit: &Callback<u8>, on_erase: &Callback<()>) -> Html {
    html! {
        <div class="pad" data-testid="pad">
            { for (1..=9u8).map(|d| {
                let done = digit_complete(game, d);
                html! {
                    <button
                        class={ if done { "pad-btn done" } else { "pad-btn" } }
                        data-testid={ format!("pad-{d}") }
                        disabled={ done }
                        onclick={ on_digit.reform(move |_| d) }>
                        { d }
                    </button>
                }
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
