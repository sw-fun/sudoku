//! Learn-mode panel: the strategy picker and the walkthrough shell.

use suduko_game::Game;
use yew::prelude::*;

/// The learn panel: picker when nothing is selected, walkthrough
/// shell once a strategy is chosen.
pub fn learn_panel(
    game: &Game,
    on_learn: &Callback<()>,
    on_pick: &Callback<usize>,
    on_step: &Callback<isize>,
) -> Html {
    match game.teaching.current() {
        None => picker(game, on_pick),
        Some(_) => walkthrough(game, on_learn, on_step),
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

fn walkthrough(game: &Game, on_learn: &Callback<()>, on_step: &Callback<isize>) -> Html {
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
            <div class="walkthrough-controls">
                <button
                    class="menu-btn"
                    data-testid="walkthrough-prev"
                    disabled={ index == 0 }
                    onclick={ on_step.reform(|_| -1) }>
                    { "Back" }
                </button>
                <button
                    class="menu-btn"
                    data-testid="walkthrough-next"
                    disabled={ index == last }
                    onclick={ on_step.reform(|_| 1) }>
                    { "Next" }
                </button>
                <button
                    class="menu-btn"
                    data-testid="walkthrough-close"
                    onclick={ on_learn.reform(|_| ()) }>
                    { "Close" }
                </button>
            </div>
        </section>
    }
}
