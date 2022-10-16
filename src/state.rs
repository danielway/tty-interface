use std::collections::{BTreeMap, BTreeSet};

use crate::{Position, Style};

/// A cell in the terminal's column/line grid composed of text and optional style.
#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Cell {
    grapheme: String,
    style: Option<Style>,
}

impl Cell {
    /// This cell's text content.
    pub(crate) fn grapheme(&self) -> &str {
        &self.grapheme
    }

    /// If available, this cell's styling.
    pub(crate) fn style(&self) -> Option<&Style> {
        self.style.as_ref()
    }
}

/// The terminal interface's contents with comparison capabilities.
#[derive(Clone)]
pub(crate) struct State {
    cells: BTreeMap<Position, Cell>,
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
    pub(crate) fn set_text(&mut self, position: Position, grapheme: &str) {
        self.dirty.insert(position);
        self.cells.insert(
            position,
            Cell {
                grapheme: grapheme.to_string(),
                style: None,
            },
        );
    }

    /// Update a particular cell's grapheme and styling.
    pub(crate) fn set_styled_text(&mut self, position: Position, grapheme: &str, style: Style) {
        self.dirty.insert(position);
        self.cells.insert(
            position,
            Cell {
                grapheme: grapheme.to_string(),
                style: Some(style),
            },
        );
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
pub(crate) struct StateIter<'a> {
    state: &'a State,
    positions: Vec<Position>,
    index: usize,
}

impl StateIter<'_> {
    /// Create a new state iterator with the specified positions starting from the first position.
    fn new(state: &State, positions: Vec<Position>) -> StateIter {
        StateIter {
            state,
            positions,
            index: 0,
        }
    }
}

impl<'a> Iterator for StateIter<'_> {
    type Item = (Position, Cell);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.positions.len() {
            let position = self.positions[self.index];
            let cell = &self.state.cells[&position];

            self.index += 1;

            Some((position, cell.clone()))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{pos, Color, Position, Style};

    use super::{Cell, State};

    #[test]
    fn state_set_text() {
        let mut state = State::new();

        state.set_text(pos!(0, 0), "A");
        state.set_text(pos!(2, 0), "B");
        state.set_text(pos!(1, 1), "C");

        assert_eq!(3, state.cells.len());
        assert_eq!(
            Cell {
                grapheme: "A".to_string(),
                style: None
            },
            state.cells[&pos!(0, 0)]
        );
        assert_eq!(
            Cell {
                grapheme: "B".to_string(),
                style: None
            },
            state.cells[&pos!(2, 0)]
        );
        assert_eq!(
            Cell {
                grapheme: "C".to_string(),
                style: None
            },
            state.cells[&pos!(1, 1)]
        );

        let dirty_positions: Vec<_> = state.dirty.clone().into_iter().collect();
        assert_eq!(3, dirty_positions.len());
        assert_eq!(pos!(0, 0), dirty_positions[0]);
        assert_eq!(pos!(2, 0), dirty_positions[1]);
        assert_eq!(pos!(1, 1), dirty_positions[2]);
    }

    #[test]
    fn state_set_styled_text() {
        let mut state = State::new();

        state.set_styled_text(pos!(0, 0), "X", Style::default().set_bold(true));
        state.set_styled_text(pos!(1, 3), "Y", Style::default().set_italic(true));
        state.set_styled_text(
            pos!(2, 2),
            "Z",
            Style::default().set_foreground(Color::Blue),
        );

        assert_eq!(3, state.cells.len());
        assert_eq!(
            Cell {
                grapheme: "X".to_string(),
                style: Some(Style::default().set_bold(true)),
            },
            state.cells[&pos!(0, 0)],
        );
        assert_eq!(
            Cell {
                grapheme: "Y".to_string(),
                style: Some(Style::default().set_italic(true)),
            },
            state.cells[&pos!(1, 3)],
        );
        assert_eq!(
            Cell {
                grapheme: "Z".to_string(),
                style: Some(Style::default().set_foreground(Color::Blue)),
            },
            state.cells[&pos!(2, 2)],
        );

        let dirty_positions: Vec<_> = state.dirty.clone().into_iter().collect();
        assert_eq!(3, dirty_positions.len());
        assert_eq!(pos!(0, 0), dirty_positions[0]);
        assert_eq!(pos!(2, 2), dirty_positions[1]);
        assert_eq!(pos!(1, 3), dirty_positions[2]);
    }

    #[test]
    fn state_clear_dirty() {
        let mut state = State::new();

        state.set_text(pos!(0, 0), "A");
        state.set_text(pos!(2, 0), "B");
        state.set_text(pos!(1, 1), "C");

        assert_eq!(3, state.cells.len());
        assert_eq!(
            Cell {
                grapheme: "A".to_string(),
                style: None
            },
            state.cells[&pos!(0, 0)]
        );
        assert_eq!(
            Cell {
                grapheme: "B".to_string(),
                style: None
            },
            state.cells[&pos!(2, 0)]
        );
        assert_eq!(
            Cell {
                grapheme: "C".to_string(),
                style: None
            },
            state.cells[&pos!(1, 1)]
        );
    }

    #[test]
    fn state_dirty_iter() {
        let mut state = State::new();

        state.set_text(pos!(0, 0), "A");
        state.clear_dirty();

        state.set_text(pos!(2, 0), "B");
        state.set_text(pos!(1, 1), "C");

        let mut iter = state.dirty_iter();
        assert_eq!(
            Some((
                pos!(2, 0),
                Cell {
                    grapheme: "B".to_string(),
                    style: None
                }
            )),
            iter.next()
        );
        assert_eq!(
            Some((
                pos!(1, 1),
                Cell {
                    grapheme: "C".to_string(),
                    style: None
                }
            )),
            iter.next()
        );
        assert_eq!(None, iter.next());
    }
}
