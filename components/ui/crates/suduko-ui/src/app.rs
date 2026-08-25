//! Top-level application component: screen routing, game lifecycle,
//! dialog state, and save-slot persistence. Menu rendering and the
//! dialogs live in `menu`; keyboard wiring in `keys`.

use crate::board::view_board;
use crate::help_overlay;
use crate::keys::install_key_handler;
use crate::menu::{LEVELS, customize_popup, next_seed, start_game, storage, view_menu};
use gloo_timers::callback::Interval;
use std::collections::BTreeMap;
use suduko_engine::Level;
use suduko_game::{Game, NoteOp, clear_selected, entry, note};
use suduko_uikit::{InputMode, mmss};
use yew::html::Scope;
use yew::prelude::*;

pub(crate) enum Msg {
    Start(Level),
    Select(usize),
    Digit(u8),
    Erase,
    InputModeSet(InputMode),
    Tick,
    Menu,
    Resume,
    Abandon(bool),
    ClearStats,
    CustomizeToggle,
    NextBoard,
    LearnToggle,
    LearnSelect(usize),
    LearnStep(isize),
    ShowMeToggle,
    ShowMeAuto(bool),
    ShowMeDelay(u32),
    NotesToggle,
    NotesShow(bool),
    NotesFill,
    NotesClear,
    NoteApply,
    NoteApplyAll,
    NoteReset,
    HelpToggle,
    /// Escape/space/digits, plus any key closing an open dialog.
    ContextKey(crate::keys::Key),
}

pub(crate) struct Model {
    screen: Screen,
    game: Option<Game>,
    level: Level,
    stats: BTreeMap<Level, u32>,
    help_open: bool,
    customize_open: bool,
    confirm_open: bool,
    input_mode: InputMode,
    notes_visible: bool,
    _timer: Interval,
}

enum Screen {
    Menu,
    Game,
}

const SAVE_KEY: &str = "sudoku-save";

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        install_key_handler(ctx.link().callback(Msg::ContextKey));
        let saved = storage()
            .and_then(|s| s.get_item(SAVE_KEY).ok().flatten())
            .and_then(|code| suduko_game::restore(&code));
        let mut model = Self {
            screen: Screen::Menu,
            game: None,
            level: Level::Easy,
            stats: BTreeMap::new(),
            help_open: false,
            customize_open: false,
            confirm_open: false,
            input_mode: InputMode::Below,
            notes_visible: true,
            _timer: Interval::new(1000, {
                let link = ctx.link().clone();
                move || link.send_message(Msg::Tick)
            }),
        };
        if let Some(s) = saved {
            model.stats = s
                .stats
                .iter()
                .map(|(l, n)| (LEVELS[usize::from(*l)], *n))
                .collect();
            if let Some(g) = s.game {
                model.level = LEVELS[usize::from(s.level)];
                model.game = Some(g);
            }
        }
        model
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Msg) -> bool {
        match msg {
            Msg::Start(level) => {
                self.game = Some(start_game(level, next_seed()));
                (self.level, self.screen) = (level, Screen::Game);
            }
            Msg::Select(idx) => self.mut_game(|g| g.select(idx)),
            Msg::Digit(d) => self.mut_game(|g| entry(g, d)),
            Msg::Erase => self.mut_game(clear_selected),
            Msg::InputModeSet(m) => self.input_mode = m,
            Msg::Tick => self.mut_game(suduko_game::showme::tick),
            Msg::Menu | Msg::ContextKey(crate::keys::Key::Escape) => return self.escape(),
            Msg::ContextKey(crate::keys::Key::Space) => self.mut_game(clear_selected),
            Msg::ContextKey(crate::keys::Key::Digit(d)) => self.mut_game(|g| entry(g, d)),
            Msg::Resume => {
                self.screen = Screen::Game;
            }
            Msg::Abandon(true) => {
                (self.confirm_open, self.game, self.screen) = (false, None, Screen::Menu);
            }
            Msg::Abandon(false) => self.confirm_open = false,
            Msg::ClearStats => self.stats.clear(),
            Msg::CustomizeToggle => self.customize_open = !self.customize_open,
            Msg::NextBoard => {
                if self.game.as_ref().is_some_and(|g| g.won) {
                    self.game = Some(start_game(self.level, next_seed()));
                }
            }
            Msg::LearnToggle => self.mut_game(Game::toggle_learn),
            Msg::LearnSelect(idx) => self.mut_game(|g| g.teaching.select(idx)),
            Msg::LearnStep(d) => self.mut_game(|g| suduko_game::showme::step_or_apply(g, d)),
            Msg::ShowMeToggle => self.mut_game(suduko_game::showme::toggle),
            Msg::ShowMeAuto(on) => self.mut_game(|g| g.show_me_auto = on),
            Msg::ShowMeDelay(t) => self.mut_game(|g| g.show_me_delay_ticks = t),
            Msg::NotesToggle => self.mut_game(|g| g.pencil = !g.pencil),
            Msg::NotesShow(on) => self.notes_visible = on,
            Msg::NotesFill => self.mut_game(|g| note(g, NoteOp::FillUser)),
            Msg::NotesClear => self.mut_game(|g| note(g, NoteOp::ClearUser)),
            Msg::NoteApply => self.mut_game(|g| note(g, NoteOp::ApplyCurrent)),
            Msg::NoteApplyAll => self.mut_game(|g| note(g, NoteOp::ApplyAll)),
            Msg::NoteReset => self.mut_game(|g| note(g, NoteOp::Reset)),
            Msg::HelpToggle => self.help_open = !self.help_open,
        }
        self.persist();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        match &self.screen {
            Screen::Menu => {
                let resume = self
                    .game
                    .as_ref()
                    .map(|g| (self.level.label(), mmss(g.elapsed_secs)));
                view_menu(
                    link,
                    &self.stats,
                    resume.as_ref().map(|(l, t)| (*l, t.as_str())),
                )
            }
            Screen::Game => match &self.game {
                Some(game) => html! {
                    <>
                        { view_board(game, self.level, self.input_mode, self.notes_visible, link) }
                        { if self.customize_open { customize_popup(self.input_mode, self.notes_visible, link) } else { html!{} } }
                        { if self.confirm_open { abandon_dialog(link) } else { html!{} } }
                        { if self.help_open { help_overlay(link) } else { html!{} } }
                    </>
                },
                None => view_menu(link, &self.stats, None),
            },
        }
    }
}

impl Model {
    /// Escape/Menu: closes any open dialog first, then guards an
    /// in-progress board behind the abandon confirmation before
    /// returning to the menu.
    fn escape(&mut self) -> bool {
        let in_progress = self
            .game
            .as_ref()
            .is_some_and(|g| !g.won && (g.elapsed_secs > 0 || g.user.iter().any(|&v| v != 0)));
        if self.help_open || self.customize_open || self.confirm_open {
            self.help_open = false;
            self.customize_open = false;
            self.confirm_open = false;
        } else if in_progress {
            self.confirm_open = true;
        } else {
            self.game = None;
            self.screen = Screen::Menu;
        }
        self.persist();
        true
    }

    /// Runs a game mutation, tallies a win the moment any mutation
    /// finishes the board (placement, show-me step, or tick), and
    /// persists afterwards.
    fn mut_game(&mut self, f: impl FnOnce(&mut Game)) {
        let was_won = self.game.as_ref().is_some_and(|g| g.won);
        if let Some(g) = self.game.as_mut() {
            f(g);
            if g.won && !was_won {
                *self.stats.entry(self.level).or_default() += 1;
            }
            self.persist();
        }
    }

    /// Writes the single save slot: stats always, the board only
    /// while in progress (won boards clear the game part).
    fn persist(&self) {
        let idx =
            |l: Level| u8::try_from(LEVELS.iter().position(|&x| x == l).unwrap_or(0)).unwrap_or(0);
        let stats = self
            .stats
            .iter()
            .map(|(l, n)| (idx(*l), *n))
            .collect::<BTreeMap<_, _>>();
        let live = self.game.as_ref().filter(|g| !g.won);
        if let Some(store) = storage() {
            store
                .set_item(SAVE_KEY, &suduko_game::save(idx(self.level), live, &stats))
                .ok();
        }
    }
}

/// The abandon-progress confirmation shown when leaving an
/// unfinished board. Yes discards the board (stats are kept); No,
/// the X, or clicking outside keeps playing.
fn abandon_dialog(link: &Scope<Model>) -> Html {
    let answer = link.callback(Msg::Abandon);
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
