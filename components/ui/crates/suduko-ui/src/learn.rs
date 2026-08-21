//! Learn-mode UI: the strategy picker, the walkthrough shell, and
//! the per-step cell emphasis rendering.

use suduko_game::{Game, StepView};
use yew::prelude::*;

/// The learn panel: picker when nothing is selected, walkthrough
/// shell once a strategy is chosen.
pub fn learn_panel(
    game: &Game,
    on_learn: &Callback<()>,
    on_pick: &Callback<usize>,
    on_step: &Callback<isize>,
    on_auto: &Callback<bool>,
) -> Html {
    match game.teaching.current() {
        None => picker(game, on_pick),
        Some(_) => walkthrough(game, on_learn, on_step, on_auto),
    }
}

fn picker(game: &Game, on_pick: &Callback<usize>) -> Html {
    let offers = game.teaching.offers();
    html! {
        <section class="learn" data-testid="learn-panel">
            <h3>{ format!("Strategies on this board ({})", offers.len()) }</h3>
            <div class="learn-list">
                { for offers.iter().enumerate().map(|(idx, a)| html! {
                    <button
                        class="learn-item"
                        data-testid={ format!("learn-item-{idx}") }
                        onclick={ on_pick.reform(move |_| idx) }>
                        { a.title.clone() }
                    </button>
                }) }
            </div>
        </section>
    }
}

fn walkthrough(
    game: &Game,
    on_learn: &Callback<()>,
    on_step: &Callback<isize>,
    on_auto: &Callback<bool>,
) -> Html {
    let Some(annotation) = game.teaching.current() else {
        return html! {};
    };
    let Some(step) = annotation.steps.get(game.teaching.step_index) else {
        return html! {};
    };
    let index = game.teaching.step_index;
    let last = annotation.steps.len() - 1;
    html! {
        <section class="learn walkthrough" data-testid="walkthrough">
            <h3 class="walkthrough-title">{ annotation.title.clone() }</h3>
            <div class="walkthrough-text" data-testid="walkthrough-text">
                { step.text.clone() }
            </div>
            <div class="walkthrough-progress">{ format!("step {} of {}", index + 1, last + 1) }</div>
            { walkthrough_controls(game, index, last, on_learn, on_step, on_auto) }
        </section>
    }
}

pub(crate) fn step_classes(view: Option<&StepView>, idx: usize) -> Vec<&'static str> {
    let Some(view) = view else {
        return Vec::new();
    };
    let mut out = Vec::new();
    if view.cells.contains(&idx) {
        out.push("tutor-cell");
    }
    if view.units.iter().any(|u| u.cells().contains(&idx)) {
        out.push("tutor-unit");
    }
    if view.pulse_cells.contains(&idx) {
        out.push(match view.pulse {
            suduko_game::Pulse::Red => "tutor-elim",
            suduko_game::Pulse::Green => "tutor-place",
        });
    }
    out
}

pub(crate) fn marks_html(idx: usize, marks: &[Vec<u8>], view: Option<&StepView>) -> Html {
    let emphasis = view.and_then(|v| v.marks.get(&idx));
    let striking =
        view.is_some_and(|v| v.pulse_cells.contains(&idx) && v.pulse == suduko_game::Pulse::Red);
    html! {
        <span class="marks" data-testid={ format!("marks-{idx}") }>
            { for (1..=9u8).map(|d| if marks[idx].contains(&d) {
                let hot = emphasis.is_some_and(|e| e.contains(&d));
                let strike = hot && striking;
                if strike {
                    html! { <span class="mark hot strike">{ d }</span> }
                } else if hot {
                    html! { <span class="mark hot">{ d }</span> }
                } else {
                    html! { <span class="mark">{ d }</span> }
                }
            } else {
                html! { <span class="mark empty">{ " " }</span> }
            }) }
        </span>
    }
}

fn auto_toggle(game: &Game, on_auto: &Callback<bool>) -> Html {
    html! {
        <label class="auto-label">
            <input
                type="checkbox"
                data-testid="showme-auto"
                checked={ game.show_me_auto }
                onclick={ on_auto.reform(|e: MouseEvent| {
                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                    input.checked()
                }) } />
            { "Auto" }
        </label>
    }
}

#[allow(clippy::too_many_arguments)]
fn walkthrough_controls(
    game: &Game,
    index: usize,
    last: usize,
    on_learn: &Callback<()>,
    on_step: &Callback<isize>,
    on_auto: &Callback<bool>,
) -> Html {
    html! {
        <div class="walkthrough-controls">
            <button
                class="menu-btn"
                data-testid="walkthrough-prev"
                disabled={ index == 0 }
                onclick={ on_step.reform(|_| -1) }>
                { "Back" }
            </button>
            if game.show_me {
                <button
                    class="menu-btn"
                    data-testid="walkthrough-next"
                    onclick={ on_step.reform(|_| 1) }>
                    { if index == last { "Apply & continue" } else { "Next" } }
                </button>
                { auto_toggle(game, on_auto) }
            } else {
                <button
                    class="menu-btn"
                    data-testid="walkthrough-next"
                    disabled={ index == last }
                    onclick={ on_step.reform(|_| 1) }>
                    { "Next" }
                </button>
            }
            <button
                class="menu-btn"
                data-testid="walkthrough-close"
                onclick={ on_learn.reform(|_| ()) }>
                { "Close" }
            </button>
        </div>
    }
}
