//! Menu screen: difficulty selection and the stats view.

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
