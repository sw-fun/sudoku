//! Game-to-tutor glue: shown values, learn-panel toggling,
//! teaching-only pencil marks, and walkthrough step views.

use super::Game;
use suduko_grid::CELL_COUNT;
use suduko_tutor::{Effect, UnitRef};

/// What the effect targets pulse as on the board.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pulse {
    /// Elimination candidates flash red.
    Red,
    /// A placed digit flashes green.
    Green,
}

/// What the board emphasizes for the current teaching step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepView {
    /// Spotlighted pattern cells of this beat.
    pub cells: Vec<usize>,
    /// Tinted units of this beat.
    pub units: Vec<UnitRef>,
    /// Focus digits of this beat.
    pub focus_digits: Vec<u8>,
    /// Cell -> emphasized digits within that cell's pencil marks.
    pub marks: std::collections::BTreeMap<usize, Vec<u8>>,
    /// How the effect targets pulse.
    pub pulse: Pulse,
    /// Cells the effect removes candidates from / places into.
    pub pulse_cells: Vec<usize>,
}

impl Game {
    /// The board emphasis for the current walkthrough step, if a
    /// walkthrough is active.
    #[must_use]
    pub fn step_view(&self) -> Option<StepView> {
        let annotation = self.teaching.current()?;
        let step = annotation.steps.get(self.teaching.step_index)?;
        let pulse_cells: Vec<usize> = match &annotation.effect {
            Effect::Place { idx, .. } => vec![*idx],
            Effect::Eliminate { removals } => {
                let mut cells: Vec<usize> = removals.iter().map(|&(i, _)| i).collect();
                cells.sort_unstable();
                cells.dedup();
                cells
            }
        };
        let pulse = match &annotation.effect {
            Effect::Place { .. } => Pulse::Green,
            Effect::Eliminate { .. } => Pulse::Red,
        };
        Some(StepView {
            cells: step.cells.clone(),
            units: step.units.clone(),
            focus_digits: step.digits.clone(),
            marks: step
                .cells
                .iter()
                .filter_map(|&idx| {
                    let focus: Vec<u8> = step
                        .digits
                        .iter()
                        .copied()
                        .filter(|d| self.shown(idx) == 0 && self.pencil_marks()[idx].contains(d))
                        .collect();
                    (!focus.is_empty()).then_some((idx, focus))
                })
                .collect(),
            pulse,
            pulse_cells,
        })
    }
}

impl Game {
    /// The shown values as tutor input (clues then user entries).
    #[must_use]
    pub fn shown_values(&self) -> [u8; CELL_COUNT] {
        core::array::from_fn(|idx| self.shown(idx))
    }

    /// Opens the learn panel with strategies for the current board.
    pub fn open_learn(&mut self) {
        let cands = suduko_tutor::candidates_with(&self.shown_values(), &self.eliminated);
        self.teaching.refresh(&cands);
        self.teaching.panel_open = true;
    }

    /// Closes the learn panel and clears the walkthrough.
    pub fn close_learn(&mut self) {
        self.teaching.close();
    }

    /// Toggles the learn panel for the current board.
    pub fn toggle_learn(&mut self) {
        if self.teaching.panel_open {
            self.close_learn();
        } else {
            self.open_learn();
        }
    }

    /// Pencil marks for the current board (empty cells only).
    #[must_use]
    pub fn pencil_marks(&self) -> [Vec<u8>; CELL_COUNT] {
        let cands = suduko_tutor::candidates_with(&self.shown_values(), &self.eliminated);
        core::array::from_fn(|idx| {
            if !cands.placed[idx] {
                (1..=9u8)
                    .filter(|d| cands.masks[idx] & (1 << (d - 1)) != 0)
                    .collect()
            } else {
                Vec::new()
            }
        })
    }
}

/// Pencil-note operations on the removal and user layers.
pub enum NoteOp {
    /// Append strategy removals to the layer.
    Extend(Vec<(usize, u8)>),
    /// Fill every empty cell's user notes with computed candidates.
    FillUser,
    /// Wipe the whole user-notes layer.
    ClearUser,
    /// Apply the selected walkthrough annotation's eliminations.
    ApplyCurrent,
    /// Apply every offered elimination strategy at once.
    ApplyAll,
    /// Clear the whole layer.
    Reset,
}

/// One note-layer action; see `NoteOp`.
pub fn note(game: &mut Game, op: NoteOp) {
    match op {
        NoteOp::FillUser => {
            let cands = suduko_tutor::candidates_with(&game.shown_values(), &[]);
            for idx in 0..suduko_grid::CELL_COUNT {
                if !cands.placed[idx] {
                    game.user_marks[idx] = cands.masks[idx] & 0x1ff;
                }
            }
        }
        NoteOp::ClearUser => game.user_marks = [0; suduko_grid::CELL_COUNT],
        NoteOp::Extend(removals) => {
            game.eliminated.extend(removals);
            game.eliminated.sort_unstable();
            game.eliminated.dedup();
        }
        NoteOp::ApplyCurrent => {
            let Some(annotation) = game.teaching.current().cloned() else {
                return;
            };
            if let suduko_tutor::Effect::Eliminate { removals } = annotation.effect {
                note(game, NoteOp::Extend(removals));
                let cands = suduko_tutor::candidates_with(&game.shown_values(), &game.eliminated);
                game.teaching.refresh(&cands);
                game.teaching.panel_open = true;
            }
        }
        NoteOp::ApplyAll => {
            let mut all = Vec::new();
            for a in game.teaching.offers() {
                if let suduko_tutor::Effect::Eliminate { removals } = &a.effect {
                    all.extend_from_slice(removals);
                }
            }
            note(game, NoteOp::Extend(all));
            let cands = suduko_tutor::candidates_with(&game.shown_values(), &game.eliminated);
            game.teaching.refresh(&cands);
            game.teaching.panel_open = true;
        }
        NoteOp::Reset => game.eliminated.clear(),
    }
}
