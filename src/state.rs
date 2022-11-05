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
        self.handle_cell_update(position, grapheme, None);
    }

    /// Update a particular cell's grapheme and styling.
    pub(crate) fn set_styled_text(&mut self, position: Position, grapheme: &str, style: Style) {
        self.handle_cell_update(position, grapheme, Some(style));
    }

    /// Updates state and queues dirtied positions, if they've changed.
    fn handle_cell_update(&mut self, position: Position, grapheme: &str, style: Option<Style>) {
        let new_cell = Cell {
            grapheme: grapheme.to_string(),
            style,
        };

        // If this cell is unchanged, do not mark it dirty
        if Some(&new_cell) == self.cells.get(&position) {
            return;
        }

        self.dirty.insert(position);
        self.cells.insert(position, new_cell);
    }

    /// Clears all cells in the specified line.
    pub(crate) fn clear_line(&mut self, line: u16) {
        self.handle_cell_clears(|position| position.y() == line);
    }

    /// Clears cells in the line from the specified position.
    pub(crate) fn clear_rest_of_line(&mut self, from: Position) {
        self.handle_cell_clears(|position| position.y() == from.y() && position.x() >= from.x());
    }

    /// Clears cells in the interface from the specified position.
    pub(crate) fn clear_rest_of_interface(&mut self, from: Position) {
        self.handle_cell_clears(|position| *position >= &from);
    }

    /// Clears cells matching the specified predicate, marking them dirtied for re-render.
    fn handle_cell_clears<P: FnMut(&&Position) -> bool>(&mut self, filter_predicate: P) {
        let cells = self.cells.keys();
        let deleted_cells = cells.filter(filter_predicate);
        let cell_positions: Vec<Position> = deleted_cells.map(|position| *position).collect();

        for position in cell_positions {
            self.cells.remove(&position);
            self.dirty.insert(position);
        }
    }

    /// Marks any dirty cells as clean.
    pub(crate) fn clear_dirty(&mut self) {
        self.dirty.clear()
    }

    /// Create an iterator for this state's dirty cells.
    pub(crate) fn dirty_iter(&self) -> StateIter {
        StateIter::new(self, self.dirty.clone().into_iter().collect())
    }

    /// Get the last cell's position.
    pub(crate) fn get_last_position(&self) -> Option<Position> {
        self.cells
            .keys()
            .last()
            .and_then(|position| Some(*position))
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
    type Item = (Position, Option<Cell>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.positions.len() {
            let position = self.positions[self.index];
            let cell = self
                .state
                .cells
                .get(&position)
                .and_then(|cell| Some(cell.clone()));

            self.index += 1;
            Some((position, cell))
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

        state.set_styled_text(pos!(0, 0), "X", Style::new().set_bold(true));
        state.set_styled_text(pos!(1, 3), "Y", Style::new().set_italic(true));
        state.set_styled_text(pos!(2, 2), "Z", Style::new().set_foreground(Color::Blue));

        assert_eq!(3, state.cells.len());
        assert_eq!(
            Cell {
                grapheme: "X".to_string(),
                style: Some(Style::new().set_bold(true)),
            },
            state.cells[&pos!(0, 0)],
        );
        assert_eq!(
            Cell {
                grapheme: "Y".to_string(),
                style: Some(Style::new().set_italic(true)),
            },
            state.cells[&pos!(1, 3)],
        );
        assert_eq!(
            Cell {
                grapheme: "Z".to_string(),
                style: Some(Style::new().set_foreground(Color::Blue)),
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
    fn state_clear_line() {
        let mut state = State::new();

        state.set_text(pos!(0, 0), "A");
        state.set_text(pos!(2, 0), "B");
        state.set_text(pos!(1, 1), "C");
        state.set_text(pos!(3, 1), "D");
        state.clear_dirty();

        assert_eq!(4, state.cells.len());
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
        assert_eq!(
            Cell {
                grapheme: "D".to_string(),
                style: None
            },
            state.cells[&pos!(3, 1)]
        );

        state.clear_line(1);

        let dirty_positions: Vec<_> = state.dirty.clone().into_iter().collect();
        assert_eq!(2, dirty_positions.len());
        assert_eq!(pos!(1, 1), dirty_positions[0]);
        assert_eq!(pos!(3, 1), dirty_positions[1]);

        let line_two_cell_count = state.cells.keys().filter(|pos| pos.y() == 1).count();
        assert_eq!(0, line_two_cell_count);
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
    fn state_clear_rest_of_line() {
        let mut state = State::new();

        let content = ["ABC", "DEF", "GHI"];

        for row in 0..content.len() {
            let text = content[row];
            for column in 0..text.len() {
                state.set_text(
                    pos!(column as u16, row as u16),
                    text.get(column..column + 1).unwrap(),
                );
            }
        }

        state.clear_dirty();

        assert_eq!(9, state.cells.len());

        state.clear_rest_of_line(pos!(1, 1));

        assert_eq!(7, state.cells.len());

        let dirty_positions: Vec<_> = state.dirty.clone().into_iter().collect();
        assert_eq!(2, dirty_positions.len());
        assert_eq!(pos!(1, 1), dirty_positions[0]);
        assert_eq!(pos!(2, 1), dirty_positions[1]);

        let line_two_cell_count = state.cells.keys().filter(|pos| pos.y() == 1).count();
        assert_eq!(1, line_two_cell_count);
    }

    #[test]
    fn state_clear_rest_of_interface() {
        let mut state = State::new();

        let content = ["ABC", "DEF", "GHI"];

        for row in 0..content.len() {
            let text = content[row];
            for column in 0..text.len() {
                state.set_text(
                    pos!(column as u16, row as u16),
                    text.get(column..column + 1).unwrap(),
                );
            }
        }

        state.clear_dirty();

        assert_eq!(9, state.cells.len());

        state.clear_rest_of_interface(pos!(1, 1));

        assert_eq!(4, state.cells.len());

        let dirty_positions: Vec<_> = state.dirty.clone().into_iter().collect();
        assert_eq!(5, dirty_positions.len());
        assert_eq!(pos!(1, 1), dirty_positions[0]);
        assert_eq!(pos!(2, 1), dirty_positions[1]);
        assert_eq!(pos!(0, 2), dirty_positions[2]);
        assert_eq!(pos!(1, 2), dirty_positions[3]);
        assert_eq!(pos!(2, 2), dirty_positions[4]);
    }

    #[test]
    fn state_dirty_iter() {
        let mut state = State::new();

        state.set_text(pos!(0, 0), "A");
        state.clear_dirty();

        state.set_text(pos!(2, 0), "B");
        state.set_text(pos!(1, 1), "C");
        state.set_text(pos!(0, 2), "D");
        state.clear_line(1);

        let mut iter = state.dirty_iter();
        assert_eq!(
            Some((
                pos!(2, 0),
                Some(Cell {
                    grapheme: "B".to_string(),
                    style: None
                })
            )),
            iter.next()
        );
        assert_eq!(Some((pos!(1, 1), None,)), iter.next());
        assert_eq!(
            Some((
                pos!(0, 2),
                Some(Cell {
                    grapheme: "D".to_string(),
                    style: None
                })
            )),
            iter.next()
        );
        assert_eq!(None, iter.next());
    }

    #[test]
    fn state_get_last_position() {
        let mut state = State::new();

        state.set_text(pos!(3, 1), "D");
        state.set_text(pos!(1, 1), "C");
        state.set_text(pos!(0, 0), "A");
        state.set_text(pos!(2, 0), "B");

        assert_eq!(pos!(3, 1), state.get_last_position().unwrap());
    }
}
