use std::collections::{BTreeMap, BTreeSet};

use crate::Position;

/// The terminal interface's contents with comparison capabilities.
#[derive(Clone)]
pub(crate) struct State {
    cells: BTreeMap<Position, String>,
    dirty: BTreeSet<Position>,
}

impl State {
    /// Initialize a new, empty terminal state.
    pub(crate) fn new() -> State {
        State {
            cells: BTreeMap::new(),
            dirty: BTreeSet::new(),
        }
    }

    /// Update a particular cell's grapheme.
    pub(crate) fn set(&mut self, position: Position, grapheme: String) {
        self.cells.insert(position, grapheme);
        self.dirty.insert(position);
    }

    /// Marks any dirty cells as clean.
    pub(crate) fn clear_dirty(&mut self) {
        self.dirty.clear()
    }

    /// Create an iterator for this state's dirty cells.
    pub(crate) fn dirty_iter(&self) -> StateIter {
        StateIter::new(self, self.dirty.clone().into_iter().collect())
    }
}

/// Iterates through a subset of cells in the state.
pub(crate) struct StateIter<'a>{
    state: &'a State,
    positions: Vec<Position>,
    index: usize,
}

impl StateIter<'_> {
    /// Create a new state iterator with the specified positions starting from the first position.
    fn new(state: &State, positions: Vec<Position>) -> StateIter {
        StateIter { state, positions, index: 0 }
    }
}

impl<'a> Iterator for StateIter<'a> {
    type Item = (Position, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.positions.len() {
            let position = self.positions[self.index];
            let text = &self.state.cells[&position];
            
            self.index += 1;
            
            Some((position, text))
        } else {
            None
        }
    }
}