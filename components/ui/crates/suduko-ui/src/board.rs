//! Game screen: the board grid, input surfaces (fixed pad above or
//! below the board, or the per-cell popup keypad), the customize bar,
//! the win overlay, and the learn (teaching) panel.

use crate::app::{Model, Msg};
use crate::learn::{marks_html, step_classes};
use suduko_engine::Level;
use suduko_game::{Game, StepView, digit_complete, highlight_set, keypad_visible};
use suduko_uikit::{CellInput, InputMode, anchor_style, mmss};
use yew::html::Scope;
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

#[allow(clippy::too_many_arguments)]
pub fn view_board(
    game: &Game,
    level: Level,
    mode: InputMode,
    notes_visible: bool,
    link: &Scope<Model>,
) -> Html {
    let highlights = highlight_set(game);
    let teaching = game.teaching.panel_open;
    let pencil = game.pencil;
    html! {
        <main class="game" data-testid="game">
            { header_view(game, level, pencil, link) }
            { if mode == InputMode::Above { pad(game, link) } else { html! {} } }
            { grid(game, &highlights, mode, notes_visible, link) }
            { if mode == InputMode::Below { pad(game, link) } else { html! {} } }
            { if teaching {
                crate::learn::learn_panel(game, &link.callback(|_| Msg::LearnToggle), &link.callback(Msg::LearnSelect), &link.callback(|d: isize| Msg::LearnStep(d)), &link.callback(Msg::ShowMeAuto), &link.callback(Msg::ShowMeDelay), &link.callback(|_| Msg::NoteApply), &link.callback(|_| Msg::NoteApplyAll), &link.callback(|_| Msg::NoteReset))
            } else { html! {} } }
            <button class="menu-btn customize-trigger" data-testid="customize-open" onclick={ link.callback(|_| Msg::CustomizeToggle) }>
                { "Customize" }
            </button>
            if game.won {
                { overlay(game, link) }
            }
        </main>
    }
}

/// Game header: menu/learn/showme/notes/help buttons and stats.
fn header_view(game: &Game, level: Level, pencil: bool, link: &Scope<Model>) -> Html {
    let teaching = game.teaching.panel_open;
    html! {
        <header class="game-header">
            <div class="header-btns">
                <button class="menu-btn" data-testid="menu-btn" onclick={ link.callback(|_| Msg::Menu) }>
                    { "Menu" }
                </button>
                <button class="menu-btn learn-btn" data-testid="learn-btn" onclick={ link.callback(|_| Msg::LearnToggle) }>
                    { if teaching { "Hide learning" } else { "Learn" } }
                </button>
                <button class="menu-btn showme-btn" data-testid="showme-btn" onclick={ link.callback(|_| Msg::ShowMeToggle) }>
                    { if game.show_me { "Stop show-me" } else { "Show me" } }
                </button>
                <button
                    class={ if pencil { "menu-btn notes-btn on" } else { "menu-btn notes-btn" } }
                    data-testid="notes-btn"
                    onclick={ link.callback(|_| Msg::NotesToggle) }>
                    { if pencil { "Notes: on" } else { "Notes: off" } }
                </button>
                <button class="menu-btn help-btn" data-testid="help-btn" onclick={ link.callback(|_| Msg::HelpToggle) }>
                    { "?" }
                </button>
            </div>
            <div class="header-stats">
                <span class="level" data-testid="level">{ level.label() }</span>
                <span class="timer" data-testid="timer">{ mmss(game.elapsed_secs) }</span>
                <span class="bad" data-testid="bad-count">{ format!("bad: {}", game.bad_inputs) }</span>
            </div>
        </header>
    }
}

fn grid(
    game: &Game,
    highlights: &[usize],
    mode: InputMode,
    notes_visible: bool,
    link: &Scope<Model>,
) -> Html {
    let on_select = link.callback(Msg::Select);
    let on_digit = link.callback(Msg::Digit);
    let on_erase = link.callback(|_| Msg::Erase);
    let marks = if game.teaching.panel_open {
        Some(game.pencil_marks())
    } else if notes_visible {
        Some(game.user_marks_view())
    } else {
        None
    };
    let view = game.step_view();
    let cells = (0..81).map(|idx| {
        cell(
            game,
            idx,
            highlights,
            &on_select,
            marks.as_ref().map(|m| &m[..]),
            view.as_ref(),
        )
    });
    let keypad = if mode == InputMode::Popup && keypad_visible(game) {
        keypad_view(game, &on_digit, &on_erase)
    } else {
        html! {}
    };
    html! {
        <div class="board-wrap" data-testid="board-wrap">
            <span class="coord corner" aria-hidden="true"></span>
            <div class="row-labels" data-testid="row-labels">
                { for (1..=9).map(|n| html! { <span class="coord">{ n }</span> }) }
            </div>
            <div class="board" data-testid="board">
                { for cells }
                { keypad }
            </div>
            <div class="col-labels" data-testid="col-labels">
                { for (1..=9).map(|n| html! { <span class="coord">{ n }</span> }) }
            </div>
        </div>
    }
}

/// Mini keypad anchored beside the selected cell toward the board
/// center (geometry and button rules from suduko-uikit).
fn keypad_view(game: &Game, on_digit: &Callback<u8>, on_erase: &Callback<()>) -> Html {
    let sel = game.selected.expect("keypad visible implies selection");
    let mut complete_mask = 0u16;
    for d in 1..=9u8 {
        if digit_complete(game, d) {
            complete_mask |= 1 << (d - 1);
        }
    }
    let input = CellInput {
        complete_mask,
        wrong_digit: game.is_wrong(sel).then_some(game.user[sel]),
        value: (game.shown(sel) != 0).then_some(game.shown(sel)),
        given: game.is_given(sel),
    };
    html! {
        <div class="cell-keypad" data-testid="cell-keypad" style={ anchor_style(sel) }>
            { for (1..=9u8).map(|d| {
                let enabled = input.digit_enabled(d);
                html! {
                    <button
                        class="ck-btn"
                        data-testid={ format!("keypad-{d}") }
                        disabled={ !enabled }
                        onclick={ on_digit.reform(move |_| d) }>
                        { d }
                    </button>
                }
            }) }
            <button
                class="ck-btn erase"
                data-testid="keypad-erase"
                disabled={ !input.erase_enabled() }
                onclick={ on_erase.reform(|_| ()) }>
                { "Erase" }
            </button>
        </div>
    }
}

fn pad(game: &Game, link: &Scope<Model>) -> Html {
    let on_digit = link.callback(Msg::Digit);
    let on_erase = link.callback(|_| Msg::Erase);
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

fn overlay(game: &Game, link: &Scope<Model>) -> Html {
    let on_next = link.callback(|_| Msg::NextBoard);
    html! {
        <div class="overlay win-flash" data-testid="win-overlay">
            <div class="overlay-card">
                <h2>{ "Solved!" }</h2>
                <p data-testid="final-time">{ format!("time: {}", mmss(game.elapsed_secs)) }</p>
                <p>{ format!("bad inputs: {}", game.bad_inputs) }</p>
                <button class="level-btn" data-testid="next-board" onclick={ on_next.reform(|_| ()) }>
                    { "Next board" }
                </button>
                <button class="level-btn" data-testid="overlay-menu" onclick={ link.callback(|_| Msg::Menu) }>
                    { "Menu" }
                </button>
            </div>
        </div>
    }
}
