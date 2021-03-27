use crate::cursor::CursorPosition;
use crate::line::Line;
use crate::update::UpdateBatch;
use termion::raw::RawTerminal;
use std::io::Stdout;

pub(crate) struct InterfaceState {
    pub(crate) cursor: CursorPosition,
    pub(crate) lines: Vec<Line>,
}

pub struct TTYInterface<'s> {
    state: InterfaceState,
    stdout: &'s mut RawTerminal<Stdout>,
}

impl TTYInterface<'_> {
    pub fn new(stdout: &mut RawTerminal<Stdout>) -> TTYInterface {
        TTYInterface {
            state: InterfaceState {
                lines: Vec::new(),
                cursor: CursorPosition(0,0)
            },
            stdout
        }
    }

    pub fn start_update(&self) -> UpdateBatch {
        UpdateBatch { steps: Vec::new() }
    }

    pub fn perform_update(&mut self, batch: UpdateBatch) {
        let mut update_cursor = self.state.cursor;
        for mut step in batch.steps {
            step.do_update(self.stdout, &mut self.state, &mut update_cursor);
        }
        // TODO: return from update_cursor to state.cursor
    }
}
