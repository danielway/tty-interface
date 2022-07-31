use crate::device::Device;
use crate::format::{Color, StringAppender, Style};
use crate::layout::PartLayout;
use crate::mode::{CursorMode, RenderMode};
use crate::position::AbsolutePosition;
use crate::result::Result;
use crate::text::{TextParameters, TextParametersWithLayout};
use crate::Error;
use std::cmp::max;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

const ASSUMED_UNKNOWN_WIDTH: u16 = 1;

/// Provides APIs for optimized terminal interactions including cursor movement and efficient
/// text updates.
#[derive(Debug)]
pub(crate) struct Terminal<'a> {
    /// The low-level output device to use for this terminal.
    device: Device<'a>,

    /// How the cursor should be moved in the terminal.
    cursor_mode: CursorMode,

    /// How the terminal should handle rendering content.
    render_mode: RenderMode,

    /// The starting position of the interface.
    interface_origin: AbsolutePosition,

    /// The cursor's virtual position.
    cursor_position: AbsolutePosition,

    /// The last color rendered to the terminal.
    current_color: Option<Color>,

    /// The last set of styles rendered to the terminal.
    current_styles: Vec<Style>,

    /// Grapheme measured-width cache.
    cache: HashMap<String, u16>,
}

impl<'a> Terminal<'a> {
    /// Create a new terminal in the specified modes for an output device.
    pub(crate) fn new(
        mut device: Device<'a>,
        cursor_mode: CursorMode,
        render_mode: RenderMode,
    ) -> Result<Self> {
        let mut cursor_position = AbsolutePosition::new_from_tuple(device.position()?);

        let interface_origin: AbsolutePosition;
        match render_mode {
            RenderMode::Full => {
                interface_origin = AbsolutePosition::default();
            }
            RenderMode::Relative => {
                interface_origin = cursor_position;
                cursor_position = AbsolutePosition::default();

                // Save the origin to be restored if the terminal is resized
                device.save_position()?;
            }
        };

        let terminal = Self {
            device,
            cursor_mode,
            render_mode,
            interface_origin,
            cursor_position,
            current_color: None,
            current_styles: Vec::default(),
            cache: HashMap::new(),
        };

        Ok(terminal)
    }

    /// Get the terminal's current (column, row) dimensions.
    pub(crate) fn size(&self) -> Result<(u16, u16)> {
        let mut size = self.device.size()?;
        size.0 -= 1;
        Ok(size)
    }

    /// Refreshes the cursor's virtual position by querying the terminal device.
    pub(crate) fn refresh_cursor(&mut self) -> Result<AbsolutePosition> {
        let position = self.device.position()?;
        let mut position = AbsolutePosition::new_from_tuple(position);

        if self.render_mode == RenderMode::Relative {
            position = position.add_rows(-(self.interface_origin.row() as i16));
        }

        self.cursor_position = position;

        Ok(position)
    }

    /// Returns the virtual cursor position.
    pub(crate) fn cursor(&self) -> AbsolutePosition {
        self.cursor_position
    }

    /// Display the cursor.
    pub(crate) fn show_cursor(&mut self) -> Result<()> {
        self.device.set_visible(true)
    }

    /// Hide the cursor.
    pub(crate) fn hide_cursor(&mut self) -> Result<()> {
        self.device.set_visible(false)
    }

    /// Move the cursor to the specified position.
    pub(crate) fn move_cursor(&mut self, to: AbsolutePosition) -> Result<()> {
        if to == self.cursor() {
            return Ok(());
        }

        match self.cursor_mode {
            CursorMode::Absolute => {
                let adjusted_row = to.row() + self.interface_origin.row();

                let terminal_height = self.size()?.1;
                if adjusted_row + 1 > terminal_height {
                    let adjusted_cur_row = self.cursor_position.row() + self.interface_origin.row();
                    let adjustment = (adjusted_row as i16 - adjusted_cur_row as i16) as u16;
                    self.device.move_down(adjustment)?;
                    self.interface_origin = self.interface_origin.add_rows(-(adjustment as i16))
                }

                self.device.goto(to.column(), adjusted_row)?;
            }
            CursorMode::Relative => {
                let column_diff = to.column() as i16 - self.cursor_position.column() as i16;
                let row_diff = to.row() as i16 - self.cursor_position.row() as i16;

                if column_diff > 0 {
                    self.device.move_right(column_diff as u16)?;
                } else if column_diff < 0 {
                    self.device.move_left(column_diff.abs() as u16)?;
                }

                if row_diff > 0 {
                    self.device.move_down(row_diff as u16)?;

                    let terminal_height = self.size()?.1;
                    let target_row = to.row() + self.interface_origin.row() + 1;
                    if target_row >= terminal_height {
                        self.interface_origin = self
                            .interface_origin
                            .add_rows(terminal_height as i16 - target_row as i16);
                    }
                } else if row_diff < 0 {
                    self.device.move_up(row_diff.abs() as u16)?;
                }
            }
        };

        self.cursor_position = to;

        Ok(())
    }

    /// Saves the cursor's position to be restored later.
    pub(crate) fn save_cursor(&mut self) -> Result<()> {
        self.device.save_position()
    }

    /// Restores the saved cursor position, moving to the cursor to where it was previously saved.
    pub(crate) fn restore_cursor(&mut self) -> Result<()> {
        self.device.restore_position()
    }

    /// Renders the specified text segment. If previous parameters are available, will perform an
    /// optimized render which avoids re-writing unchanged content.
    pub(crate) fn render_segment(
        &mut self,
        origin: AbsolutePosition,
        parameters: TextParameters,
        previous_parameters: Option<&TextParametersWithLayout>,
    ) -> Result<Vec<PartLayout>> {
        let text_length = parameters.text().graphemes(true).count();

        if text_length == 0 {
            return Ok(vec![PartLayout::new(origin, 0, 0, Vec::new())]);
        }

        if parameters.text().contains('\n') {
            return Err(Error::MidSegmentNewlineInvalid);
        }

        let mut part_layouts = Vec::new();
        let mut chunks = vec![(0, text_length)];
        let mut trim_count = None;

        if let Some(previous_parameters) = previous_parameters {
            if parameters.has_same_styles(previous_parameters.parameters()) {
                chunks = get_chunks(parameters.text(), previous_parameters.parameters().text())?;

                let previous_length = previous_parameters
                    .parameters()
                    .text()
                    .graphemes(true)
                    .count();

                if previous_length > text_length {
                    trim_count = Some(previous_length - text_length);
                }

                part_layouts = previous_parameters.layout().clone();
            }
        }

        let mut cursor = origin;

        loop {
            if chunks.is_empty() {
                break;
            }

            let (chunk_start, chunk_end) = chunks.remove(0);

            let render_parameters = get_render_parameters(&parameters, chunk_start, chunk_end);
            let (previous_start, previous_end) =
                get_previous_bounds(chunk_start, chunk_end, previous_parameters);

            if let Some(previous_start) = previous_start {
                cursor = previous_start
            }

            if let Some(previous_parameters) = previous_parameters {
                if let Some(previous_last_part) = previous_parameters.layout().last() {
                    if chunk_start >= previous_last_part.end() {
                        cursor = previous_last_part.end_position();
                    }
                }
            }

            let parts = self.render_chunk(cursor, render_parameters, chunk_start)?;

            let mut remainder_dirty = true;
            if let Some(previous_end) = previous_end {
                if self.cursor() == previous_end {
                    remainder_dirty = false;
                }
            }

            if chunk_end < text_length && remainder_dirty {
                chunks = vec![(chunk_end, text_length)];
            }

            merge_part_layouts(&mut part_layouts, parts.clone());
        }

        if let Some(count) = trim_count {
            trim_part_layouts(&mut part_layouts, count);
        }

        self.refresh_cursor()?;

        Ok(part_layouts)
    }

    /// Clear the cursor's current line.
    pub(crate) fn clear_line(&mut self) -> Result<()> {
        self.device.clear_line()
    }

    /// Clears from the current cursor position to the end of the line.
    pub(crate) fn clear_rest_of_line(&mut self) -> Result<()> {
        self.device.clear_until_newline()
    }

    /// Completely resets the screen in preparation for a full write.
    pub(crate) fn reset(&mut self) -> Result<()> {
        match self.render_mode {
            RenderMode::Full => {
                self.device.clear()?;
                self.move_cursor(AbsolutePosition::default())?;
            }
            RenderMode::Relative => {
                self.restore_cursor()?;

                self.cursor_position = self.refresh_cursor()?;
                self.interface_origin = self.cursor_position;

                self.save_cursor()?;

                let terminal_size = self.size()?;
                let terminal_height = terminal_size.1;

                for row in self.cursor_position.row()..terminal_height {
                    self.move_cursor(AbsolutePosition::new(0, row))?;
                    self.clear_line()?;
                }
            }
        };

        Ok(())
    }

    /// Flushes any outstanding changes.
    pub(crate) fn flush(&mut self) -> Result<()> {
        self.device.flush()
    }

    /// Renders the specified chunk, which is a subset of some segment, to the terminal.
    fn render_chunk(
        &mut self,
        location: AbsolutePosition,
        parameters: TextParameters,
        start_index: usize,
    ) -> Result<Vec<PartLayout>> {
        if parameters.text().is_empty() {
            return Ok(Vec::new());
        }

        self.move_cursor(location)?;

        let (terminal_width, terminal_height) = self.device.size()?;

        let chunk_length = parameters.text().graphemes(true).count();

        let mut part_position = location;
        let mut part_index = start_index;
        let mut part_widths = Vec::new();

        let mut part_layouts = Vec::new();
        let mut chunk_index = start_index;

        loop {
            if chunk_index - start_index == chunk_length {
                break;
            }

            let mut render_wrapped = false;
            loop {
                let mut render_parameters = self.get_render_text(
                    terminal_width,
                    self.cursor(),
                    &parameters,
                    chunk_index - start_index,
                    render_wrapped,
                );

                // TODO: When wrapping, the terminal will render graphemes just-beyond the terminal
                //       width. This cursor position is reported as n (the width) but the grapheme
                //       is actually rendered at n+1, rather than wrapping. Sending a GOTO(n)
                //       instruction will place the cursor at n, before the grapheme rendered at
                //       n+1. We need to resolve this for wrapping to work correctly. As far as I
                //       can tell, this is only an issue for rendering with unknowns, since we're
                //       trying to measure the unknowns' widths.
                //
                //       Ideas:
                //       - Confirm if we can consistently cause this issue, even when rendering
                //          character-by-character outside of tty-interface; read cursor position
                //          with each write to confirm it is updating.
                //       - If the text will end within a range of the terminal width, only render a
                //          single character at a time.

                let cursor_before = self.cursor();
                self.write_text(parameters.set_text(&render_parameters.text))?;
                self.refresh_cursor()?;

                if self.cursor().row() > cursor_before.row()
                    || self.cursor().column() < cursor_before.column()
                {
                    let adjusted_row = cursor_before.row() + self.interface_origin.row() + 1;
                    if adjusted_row == terminal_height {
                        self.interface_origin = self.interface_origin.add_rows(-1);
                        self.cursor_position = self.cursor_position.add_rows(1);
                    }

                    self.clear_line()?;

                    if render_parameters.count == 1 {
                        render_wrapped = true;
                        break;
                    }

                    if render_wrapped {
                        break;
                    }

                    render_wrapped = true;
                } else {
                    if let Some(unknown) = render_parameters.unknown.clone() {
                        let render_width = self.cursor().column() - cursor_before.column();
                        let total_new_width = render_width - render_parameters.known_width;
                        let grapheme_new_width = total_new_width / unknown.indices.len() as u16;

                        if grapheme_new_width == 0 {
                            panic!(
                                "Yay, the out-of-bounds-write scenario. \
                            Render width: {}, Total width: {}, Unknown grapheme width: {}",
                                render_width, total_new_width, grapheme_new_width
                            );
                        }

                        self.cache.insert(unknown.grapheme, grapheme_new_width);

                        for index in unknown.indices {
                            render_parameters
                                .widths
                                .insert(index, grapheme_new_width as usize);
                        }
                    }

                    part_widths.append(&mut render_parameters.widths);
                    chunk_index += render_parameters.count;
                    render_wrapped = false;
                    break;
                }
            }

            if self.cursor().column() == terminal_width || render_wrapped {
                part_layouts.push(PartLayout::new(
                    part_position,
                    part_index,
                    chunk_index,
                    part_widths,
                ));

                let mut next_part_start = self.cursor().set_column(0);
                if part_position.row() == self.cursor().row() {
                    next_part_start = next_part_start.add_rows(1);
                }

                self.move_cursor(next_part_start)?;

                part_position = next_part_start;
                part_index = chunk_index;
                part_widths = Vec::new();
            }
        }

        part_layouts.push(PartLayout::new(
            part_position,
            part_index,
            chunk_index,
            part_widths,
        ));

        Ok(part_layouts)
    }

    /// Compute the parameters for the next render. Considers a variety of factors to determine the
    /// greatest amount of text that can be rendered.
    fn get_render_text(
        &self,
        terminal_width: u16,
        position: AbsolutePosition,
        parameters: &TextParameters,
        start_index: usize,
        render_wrapped: bool,
    ) -> RenderParameters {
        let mut text = String::new();
        let mut count = 0;
        let mut widths = Vec::new();
        let mut known_width = 0;
        let mut unknown: Option<UnknownParameters> = None;

        let mut grapheme_index = 0;
        for grapheme in parameters.text().graphemes(true).skip(start_index) {
            let potential_width = get_potential_width(position, known_width, unknown.as_ref());

            match self.cache.get(grapheme) {
                Some(width) => {
                    if potential_width + width > terminal_width {
                        break;
                    }

                    known_width += width;
                    widths.push(*width as usize);
                }
                None => {
                    if potential_width + ASSUMED_UNKNOWN_WIDTH > terminal_width {
                        break;
                    }

                    if render_wrapped && count > 0 {
                        break;
                    }

                    match &mut unknown {
                        Some(unknown) => {
                            if unknown.grapheme == grapheme {
                                unknown.indices.push(grapheme_index);
                            } else {
                                break;
                            }
                        }
                        None => {
                            unknown = Some(UnknownParameters {
                                grapheme: grapheme.to_string(),
                                indices: vec![grapheme_index],
                            })
                        }
                    }
                }
            }

            text.push_str(grapheme);
            count += 1;

            if render_wrapped && unknown.is_some() {
                break;
            }

            grapheme_index += 1;
        }

        RenderParameters {
            text,
            count,
            widths,
            known_width,
            unknown: unknown.clone(),
        }
    }

    /// Write the specified text to the output device, including wrapping text in specified formats.
    fn write_text(&mut self, parameters: TextParameters) -> Result<()> {
        let mut output = String::new();

        if parameters.styles() != &self.current_styles {
            output.push_str(&termion::style::Reset.to_string());
            parameters.styles().append_to_string(&mut output);
        }

        if parameters.color() != self.current_color.as_ref() {
            match parameters.color() {
                Some(color) => color.append_to_string(&mut output),
                None => output.push_str(&termion::color::Fg(termion::color::Reset).to_string()),
            }
        }

        output.push_str(parameters.text());

        self.current_color = parameters.color().and_then(|color| Some(*color));
        self.current_styles = parameters.styles().clone();

        self.device.write(&output)?;

        Ok(())
    }
}

/// Identify the chunks of `text` that need to be rendered by comparing against `previous_text`.
fn get_chunks(text: &str, previous_text: &str) -> Result<Vec<(usize, usize)>> {
    let mut graphemes = text.graphemes(true);
    let mut previous_graphemes = previous_text.graphemes(true);

    let mut index = 0;
    let mut slice_start: Option<usize> = None;
    let mut slices = Vec::new();

    loop {
        if let Some(grapheme) = graphemes.next() {
            if Some(grapheme) != previous_graphemes.next() {
                if slice_start.is_none() {
                    slice_start = Some(index);
                }
            } else {
                if let Some(start) = slice_start {
                    slices.push((start, index));
                    slice_start = None;
                }
            }
        } else {
            break;
        }

        index += 1;
    }

    if let Some(slice_start) = slice_start {
        slices.push((slice_start, index));
    }

    Ok(slices)
}

/// Computes the potential render width for the render parameters.
fn get_potential_width(
    position: AbsolutePosition,
    known_width: u16,
    unknown: Option<&UnknownParameters>,
) -> u16 {
    let mut potential_width = position.column() + known_width;

    if let Some(unknown) = unknown {
        potential_width += ASSUMED_UNKNOWN_WIDTH * unknown.indices.len() as u16;
    }

    potential_width
}

// Get the chunk's previous start and end positions in the terminal.
fn get_previous_bounds(
    start_index: usize,
    end_index: usize,
    previous_parameters: Option<&TextParametersWithLayout>,
) -> (Option<AbsolutePosition>, Option<AbsolutePosition>) {
    if let Some(previous_parameters) = previous_parameters {
        let mut start = None;
        let mut end = None;

        for part in previous_parameters.layout() {
            if part.start() <= start_index {
                start = part.get_position(start_index);
            }

            if part.end() >= end_index {
                end = part.get_position(end_index);
                break;
            }
        }

        return (start, end);
    }

    (None, None)
}

/// Produces a specified chunk's text parameters from the segment parameters.
fn get_render_parameters<'a>(
    parameters: &'a TextParameters<'a>,
    chunk_start: usize,
    chunk_end: usize,
) -> TextParameters<'a> {
    let mut graphemes = parameters.text().grapheme_indices(true).peekable();

    let mut grapheme_index = 0;
    let mut start_index = None;
    let end_index;

    loop {
        if let Some(grapheme) = graphemes.next() {
            if grapheme_index == chunk_start {
                start_index = Some(grapheme.0);
            }

            if grapheme_index == chunk_end {
                end_index = Some(grapheme.0);
                break;
            }

            grapheme_index += 1;
        } else {
            end_index = Some(parameters.text().len());
            break;
        }
    }

    match (start_index, end_index) {
        (Some(start_index), Some(end_index)) => {
            parameters.set_text(&parameters.text()[start_index..end_index])
        }
        _ => panic!(
            "Invalid grapheme indices for chunk: {:?}-{:?} for {:?}",
            chunk_start, chunk_end, parameters
        ),
    }
}

/// Merges existing part layouts with newer layouts.
fn merge_part_layouts(parts: &mut Vec<PartLayout>, updates: Vec<PartLayout>) {
    if let (Some(first), Some(last)) = (updates.first(), updates.last()) {
        let start = first.start();
        let end = last.end();

        let mut part_index = 0;
        for update in updates {
            loop {
                if part_index == parts.len() {
                    break;
                }

                let part = &parts[part_index];
                if part.position().row() < update.position().row() {
                    part_index += 1;
                } else {
                    break;
                }
            }

            if part_index == parts.len() {
                parts.push(update);
            } else {
                let part = parts[part_index].clone();

                let merge_widths = |merge_left: bool, merge_right: bool| -> Vec<usize> {
                    let mut merged: Vec<usize> = Vec::new();
                    let mut update_widths = update.widths().clone();
                    let part_widths = part.widths().clone();

                    if merge_left && update.start() > part.start() {
                        merged.append(
                            &mut part_widths
                                .clone()
                                .into_iter()
                                .take(update.start() - part.start())
                                .collect(),
                        );
                    }

                    merged.append(&mut update_widths);

                    if merge_right && update.end() < part.end() {
                        merged.append(
                            &mut part_widths
                                .into_iter()
                                .skip(update.end() - part.start())
                                .collect(),
                        );
                    }

                    merged
                };

                if part.position().row() == update.position().row() {
                    if update.start() == start && update.end() == end {
                        parts[part_index] = PartLayout::new(
                            part.position(),
                            part.start(),
                            max(part.end(), update.end()),
                            merge_widths(true, true),
                        );
                    } else if update.start() == start {
                        parts[part_index] = PartLayout::new(
                            part.position(),
                            part.start(),
                            update.end(),
                            merge_widths(true, false),
                        );
                    } else if update.end() != end {
                        parts[part_index] = PartLayout::new(
                            update.position(),
                            update.start(),
                            part.end(),
                            merge_widths(false, true),
                        );
                    } else {
                        parts[part_index] = update;
                    }
                } else {
                    parts.insert(part_index, update);
                }
            }
        }
    }
}

/// If a segment shrank, trims the specified number of graphemes.
fn trim_part_layouts(part_layouts: &mut Vec<PartLayout>, mut grapheme_count: usize) {
    loop {
        if grapheme_count == 0 {
            break;
        }

        if let Some(last_layout) = part_layouts.last() {
            let layout_index = part_layouts.len() - 1;
            if last_layout.length() > grapheme_count {
                part_layouts[layout_index] = PartLayout::new(
                    last_layout.position(),
                    last_layout.start(),
                    last_layout.end() - grapheme_count,
                    last_layout
                        .widths()
                        .clone()
                        .into_iter()
                        .take(last_layout.widths().len() - grapheme_count)
                        .collect(),
                );
                grapheme_count = 0;
            } else {
                grapheme_count -= last_layout.length();
                part_layouts.remove(layout_index);
            }
        } else {
            break;
        }
    }
}

#[derive(Clone, Debug)]
struct RenderParameters {
    text: String,
    count: usize,
    widths: Vec<usize>,
    known_width: u16,
    unknown: Option<UnknownParameters>,
}

#[derive(Clone, Debug)]
struct UnknownParameters {
    grapheme: String,
    indices: Vec<usize>,
}
