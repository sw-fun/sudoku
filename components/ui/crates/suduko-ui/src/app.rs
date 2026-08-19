//! Top-level application component: screen routing and game lifecycle.
//! Menu rendering lives in `menu`; keyboard wiring in `keys`.

use crate::board::view_board;
use crate::keys::install_key_handler;
use crate::menu::{next_seed, view_menu};
use gloo_timers::callback::Interval;
use std::collections::BTreeMap;
use suduko_engine::Level;
use suduko_game::{Game, erase, set_value};
use yew::prelude::*;

pub(crate) enum Msg {
    Start(Level),
    Select(usize),
    Digit(u8),
    Erase,
    Tick,
    Menu,
    NextBoard,
}

pub(crate) struct Model {
    screen: Screen,
    game: Option<Game>,
    level: Level,
    stats: BTreeMap<Level, u32>,
    seed: u64,
    _timer: Interval,
}

enum Screen {
    Menu,
    Game,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let timer = Interval::new(1000, move || link.send_message(Msg::Tick));
        Self {
            screen: Screen::Menu,
            game: None,
            level: Level::Easy,
            stats: BTreeMap::new(),
            seed: next_seed(),
            _timer: timer,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Msg) -> bool {
        match msg {
            Msg::Start(level) => {
                self.level = level;
                self.seed = next_seed();
                self.game = Some(start_game(level, self.seed));
                self.screen = Screen::Game;
                true
            }
            Msg::Select(idx) => self.mut_game(|g| g.select(idx)),
            Msg::Digit(d) => self.mut_game(|g| {
                if let Some(&sel) = g.selected.as_ref() {
                    set_value(g, sel, d);
                }
            }),
            Msg::Erase => self.mut_game(|g| {
                if let Some(&sel) = g.selected.as_ref() {
                    erase(g, sel);
                }
            }),
            Msg::Tick => self.mut_game(|g| {
                if !g.won {
                    g.elapsed_secs += 1;
                }
            }),
            Msg::Menu => {
                self.screen = Screen::Menu;
                true
            }
            Msg::NextBoard => {
                if self.game.as_ref().is_some_and(|g| g.won) {
                    *self.stats.entry(self.level).or_default() += 1;
                    self.seed = next_seed();
                    self.game = Some(start_game(self.level, self.seed));
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.screen {
            Screen::Menu => view_menu(ctx.link(), &self.stats),
            Screen::Game => match &self.game {
                Some(game) => view_board(
                    game,
                    self.level,
                    ctx.link().callback(Msg::Select),
                    ctx.link().callback(Msg::Digit),
                    ctx.link().callback(|_| Msg::Erase),
                    ctx.link().callback(|_| Msg::Menu),
                    ctx.link().callback(|_| Msg::NextBoard),
                ),
                None => view_menu(ctx.link(), &self.stats),
            },
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first: bool) {
        if first {
            install_key_handler(ctx.link().callback(|k| match k {
                crate::keys::Key::Digit(d) => Msg::Digit(d),
                crate::keys::Key::Space => Msg::Erase,
                crate::keys::Key::Escape => Msg::Menu,
            }));
        }
    }
}

impl Model {
    fn mut_game(&mut self, f: impl FnOnce(&mut Game)) -> bool {
        match self.game.as_mut() {
            Some(g) => {
                f(g);
                true
            }
            None => false,
        }
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

#[function_component(App)]
pub fn app() -> Html {
    html! { <Model /> }
}
