//! Top-level application component: screen routing and game lifecycle.
//! Menu rendering lives in `menu`; keyboard wiring in `keys`.

use crate::board::view_board;
use crate::keys::install_key_handler;
use crate::menu::{next_seed, start_game, view_menu};
use gloo_timers::callback::Interval;
use std::collections::BTreeMap;
use suduko_engine::Level;
use suduko_game::{Game, NoteOp, clear_selected, entry, note};
use yew::prelude::*;

pub(crate) enum Msg {
    Start(Level),
    Select(usize),
    Digit(u8),
    Erase,
    Tick,
    Menu,
    NextBoard,
    LearnToggle,
    LearnSelect(usize),
    LearnStep(isize),
    ShowMeToggle,
    ShowMeAuto(bool),
    ShowMeDelay(u32),
    NotesToggle,
    NoteApply,
    NoteApplyAll,
    NoteReset,
    HelpToggle,
    /// Escape/space: closes help when open, otherwise the classic role.
    ContextKey(crate::keys::Key),
}

pub(crate) struct Model {
    screen: Screen,
    game: Option<Game>,
    level: Level,
    stats: BTreeMap<Level, u32>,
    seed: u64,
    help_open: bool,
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
            help_open: false,
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
            Msg::Digit(d) => self.mut_game(|g| entry(g, d)),
            Msg::Erase => self.mut_game(clear_selected),
            Msg::Tick => self.mut_game(|g| {
                if !g.won {
                    g.elapsed_secs += 1;
                }
                suduko_game::showme::tick(g);
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
            Msg::LearnToggle => self.mut_game(Game::toggle_learn),
            Msg::LearnSelect(idx) => self.mut_game(|g| g.teaching.select(idx)),
            Msg::LearnStep(d) => self.mut_game(|g| suduko_game::showme::step_or_apply(g, d)),
            Msg::ShowMeToggle => self.mut_game(suduko_game::showme::toggle),
            Msg::ShowMeAuto(on) => self.mut_game(|g| g.show_me_auto = on),
            Msg::ShowMeDelay(ticks) => self.mut_game(|g| g.show_me_delay_ticks = ticks),
            Msg::NotesToggle => self.mut_game(|g| g.notes_mode = !g.notes_mode),
            Msg::NoteApply => self.mut_game(|g| note(g, NoteOp::ApplyCurrent)),
            Msg::NoteApplyAll => self.mut_game(|g| note(g, NoteOp::ApplyAll)),
            Msg::NoteReset => self.mut_game(|g| note(g, NoteOp::Reset)),
            Msg::HelpToggle => {
                self.help_open = !self.help_open;
                true
            }
            Msg::ContextKey(key) => self.context_key(key),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.screen {
            Screen::Menu => view_menu(ctx.link(), &self.stats),
            Screen::Game => match &self.game {
                Some(game) => {
                    let board = view_board(
                        game,
                        self.level,
                        ctx.link().callback(Msg::Select),
                        ctx.link().callback(Msg::Digit),
                        ctx.link().callback(|_| Msg::Erase),
                        ctx.link().callback(|_| Msg::Menu),
                        ctx.link().callback(|_| Msg::NextBoard),
                        ctx.link().callback(|_| Msg::LearnToggle),
                        ctx.link().callback(Msg::LearnSelect),
                        ctx.link().callback(|d: isize| Msg::LearnStep(d)),
                        ctx.link().callback(|_| Msg::ShowMeToggle),
                        ctx.link().callback(Msg::ShowMeAuto),
                        ctx.link().callback(Msg::ShowMeDelay),
                        ctx.link().callback(|_| Msg::NotesToggle),
                        ctx.link().callback(|_| Msg::NoteApply),
                        ctx.link().callback(|_| Msg::NoteApplyAll),
                        ctx.link().callback(|_| Msg::NoteReset),
                        ctx.link().callback(|_| Msg::HelpToggle),
                    );
                    if self.help_open {
                        html! {
                            <>
                                { board }
                                { crate::menu::help_overlay(
                                    ctx.link().callback(|_| Msg::HelpToggle),
                                ) }
                            </>
                        }
                    } else {
                        board
                    }
                }
                None => view_menu(ctx.link(), &self.stats),
            },
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first: bool) {
        if first {
            install_key_handler(ctx.link().callback(Msg::ContextKey));
        }
    }
}

impl Model {
    /// Context keys: any key closes help; otherwise Escape opens the
    /// menu, space erases, digits place.
    fn context_key(&mut self, key: crate::keys::Key) -> bool {
        if self.help_open {
            self.help_open = false;
            return true;
        }
        match key {
            crate::keys::Key::Escape => self.screen = Screen::Menu,
            crate::keys::Key::Space => return self.mut_game(clear_selected),
            crate::keys::Key::Digit(d) => return self.mut_game(|g| entry(g, d)),
        }
        true
    }

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

#[function_component(App)]
pub fn app() -> Html {
    html! { <Model /> }
}
