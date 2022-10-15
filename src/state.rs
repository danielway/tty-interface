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
    pub(crate) fn set(&mut self, position: Position, grapheme: &str) {
        self.cells.insert(position, grapheme.to_string());
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

#[cfg(test)]
mod tests {
    use crate::{Position, pos};

    use super::State;

    #[test]
    fn state_set() {
        let mut state = State::new();

        state.set(pos!(0, 0), "A");
        state.set(pos!(2, 0), "B");
        state.set(pos!(1, 1), "C");

        assert_eq!(3, state.cells.len());
        assert_eq!("A", state.cells[&pos!(0, 0)]);
        assert_eq!("B", state.cells[&pos!(2, 0)]);
        assert_eq!("C", state.cells[&pos!(1, 1)]);

        let dirty_positions: Vec<_> = state.dirty.clone().into_iter().collect();
        assert_eq!(3, dirty_positions.len());
        assert_eq!(pos!(0, 0), dirty_positions[0]);
        assert_eq!(pos!(2, 0), dirty_positions[1]);
        assert_eq!(pos!(1, 1), dirty_positions[2]);
    }

    #[test]
    fn state_clear_dirty() {
        let mut state = State::new();

        state.set(pos!(0, 0), "A");
        state.set(pos!(2, 0), "B");
        state.set(pos!(1, 1), "C");

        assert_eq!(3, state.cells.len());
        assert_eq!("A", state.cells[&pos!(0, 0)]);
        assert_eq!("B", state.cells[&pos!(2, 0)]);
        assert_eq!("C", state.cells[&pos!(1, 1)]);
    }
}