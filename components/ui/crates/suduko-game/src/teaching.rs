//! Teaching (learn mode) state: strategy offers for the current
//! board, the selected walkthrough, and its step cursor.

use suduko_tutor::{Annotation, Candidates, find_all_in};

/// Pure learn-mode state owned by the game.
#[derive(Clone, Default)]
pub struct Teaching {
    /// Annotations for the board as of the last refresh.
    offers: Vec<Annotation>,
    /// Index into `offers` of the selected walkthrough.
    selected: Option<usize>,
    /// Panel visibility (listing or walkthrough shell).
    pub panel_open: bool,
    /// Cursor into the selected annotation's steps.
    pub step_index: usize,
}

impl Teaching {
    /// Recomputes the strategy list for `shown`; drops a selection
    /// that no longer exists and clamps the step cursor.
    pub fn refresh(&mut self, cands: &Candidates) {
        self.offers = find_all_in(cands);
        let keep = self
            .selected
            .is_some_and(|i| self.offers.as_slice().get(i).is_some());
        if !keep {
            self.selected = None;
            self.step_index = 0;
        }
        if let Some(idx) = self.selected {
            self.step_index = self
                .step_index
                .min(self.offers[idx].steps.len().saturating_sub(1));
        }
    }

    /// The strategy offers for the picker, ladder-ordered.
    #[must_use]
    pub fn offers(&self) -> &[Annotation] {
        &self.offers
    }

    /// The selected annotation, if any.
    #[must_use]
    pub fn current(&self) -> Option<&Annotation> {
        self.selected.and_then(|i| self.offers.get(i))
    }

    /// Appends an offer (used by the trial fallback) and selects it.
    pub fn push_offer(&mut self, annotation: Annotation) {
        self.offers.push(annotation);
        self.selected = Some(self.offers.len() - 1);
        self.step_index = 0;
    }

    /// Closes the panel and clears the walkthrough and offers.
    pub fn close(&mut self) {
        self.panel_open = false;
        self.selected = None;
        self.step_index = 0;
        self.offers.clear();
    }

    /// Selects an offer by index (from the picker) and resets the
    /// cursor.
    pub fn select(&mut self, idx: usize) {
        if idx < self.offers.len() {
            self.selected = Some(idx);
            self.step_index = 0;
        }
    }

    /// Moves the cursor by `delta` steps, clamped to the walkthrough.
    pub fn step_by(&mut self, delta: isize) {
        let Some(last) = self.current().map(|a| a.steps.len() - 1) else {
            return;
        };
        let target = self.step_index as isize + delta;
        self.step_index = target.clamp(0, last as isize) as usize;
    }
}
