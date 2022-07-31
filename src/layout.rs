use crate::line::LineId;
use crate::position::{AbsolutePosition, RelativePosition};
use crate::segment::SegmentId;

/// A rendered layout including constituent line and segment arrangement.
#[derive(Debug, Clone)]
pub struct InterfaceLayout {
    terminal_size: (u16, u16),
    lines: Vec<LineLayout>,
}

impl InterfaceLayout {
    /// A layout from the specified lines.
    pub(crate) fn new(lines: Vec<LineLayout>, terminal_size: (u16, u16)) -> Self {
        Self {
            lines,
            terminal_size,
        }
    }

    /// The terminal size when this layout was created.
    pub fn terminal_size(&self) -> (u16, u16) {
        self.terminal_size
    }

    /// This layout's constituent lines.
    pub fn lines(&self) -> &Vec<LineLayout> {
        &self.lines
    }

    /// Get the specified line's layout.
    pub fn get_line(&self, line_id: LineId) -> Option<&LineLayout> {
        self.lines
            .iter()
            .find(|layout| layout.line_id == Some(line_id))
    }

    /// The total number of rendered terminal lines in this layout, after wrapping.
    pub fn rendered_line_count(&self) -> u16 {
        self.lines
            .iter()
            .map(|line| line.wrapped_line_count())
            .sum::<u16>()
    }

    /// The exclusive position on screen where this interface ends.
    pub fn end_position(&self) -> Option<AbsolutePosition> {
        Some(
            self.lines()
                .iter()
                .filter(|line| !line.segments().is_empty())
                .last()?
                .end_position()?,
        )
    }

    /// Given a relative position, derive its absolute position in this layout. Returns `None` if
    /// the relative position is invalid for this layout.
    pub fn get_absolute_position(&self, relative: RelativePosition) -> Option<AbsolutePosition> {
        let index = relative.position() as usize;
        let mut matched_segment = false;
        for line_layout in self.lines.iter().rev() {
            let mut matching_line = false;
            if let Some(line_id) = line_layout.line_id {
                if line_id == relative.line_id() {
                    matching_line = true;
                }
            }

            for segment_layout in line_layout.segments.iter().rev() {
                let mut matching_segment = false;
                if let Some(segment_id) = segment_layout.segment_id {
                    if matching_line && segment_id == relative.segment_id() {
                        matched_segment = true;
                        matching_segment = true;
                    }
                }

                if matched_segment && !segment_layout.parts.is_empty() {
                    if matching_segment {
                        return segment_layout.get_absolute_position(index);
                    } else if index == 0 {
                        return segment_layout.end_position();
                    } else {
                        return None;
                    }
                }
            }
        }

        None
    }
}

/// A rendered line's layout once arranged on the screen.
#[derive(Debug, Clone)]
pub struct LineLayout {
    line_id: Option<LineId>,
    segments: Vec<SegmentLayout>,
}

impl LineLayout {
    /// A layout from the specified segments.
    pub(crate) fn new(line_id: LineId, segments: Vec<SegmentLayout>) -> Self {
        Self {
            line_id: Some(line_id),
            segments,
        }
    }

    /// This layout's line's ID.
    pub fn line_id(&self) -> Option<LineId> {
        self.line_id
    }

    /// This layout's constituent text segments.
    pub fn segments(&self) -> &Vec<SegmentLayout> {
        &self.segments
    }

    /// Get the specified segment's layout.
    pub fn get_segment(&self, segment_id: SegmentId) -> Option<&SegmentLayout> {
        self.segments
            .iter()
            .find(|segment| segment.segment_id == Some(segment_id))
    }

    /// Get the segment's layout preceding the specified segment.
    pub fn get_segment_before(&self, segment_id: SegmentId) -> Option<&SegmentLayout> {
        let mut previous_segment = None;

        for segment in &self.segments {
            if segment.segment_id == Some(segment_id) {
                return previous_segment;
            }

            previous_segment = Some(segment);
        }

        None
    }

    /// The total number of rendered terminal lines in this layout, after wrapping.
    pub fn wrapped_line_count(&self) -> u16 {
        if self.segments.is_empty() {
            0
        } else {
            self.segments
                .iter()
                .map(|segment| segment.wrapped_line_count())
                .sum::<u16>()
                + 1
        }
    }

    /// The exclusive position on screen where this line ends.
    pub fn end_position(&self) -> Option<AbsolutePosition> {
        for segment in self.segments.iter().rev() {
            if !segment.parts().is_empty() {
                return segment.end_position();
            }
        }
        None
    }
}

impl Default for LineLayout {
    /// A default empty layout.
    fn default() -> Self {
        Self {
            line_id: None,
            segments: Vec::new(),
        }
    }
}

/// A rendered segment's layout once arranged on the screen.
#[derive(Debug, Clone)]
pub struct SegmentLayout {
    segment_id: Option<SegmentId>,
    parts: Vec<PartLayout>,
}

impl SegmentLayout {
    /// A layout from the specified parts.
    pub(crate) fn new(segment_id: SegmentId, parts: Vec<PartLayout>) -> Self {
        Self {
            segment_id: Some(segment_id),
            parts,
        }
    }

    /// A default empty layout.
    pub(crate) fn default() -> Self {
        Self {
            segment_id: None,
            parts: Vec::new(),
        }
    }

    /// This layout's segment identifier.
    pub fn segment_id(&self) -> Option<SegmentId> {
        self.segment_id
    }

    /// This layout's constituent parts.
    pub fn parts(&self) -> &Vec<PartLayout> {
        &self.parts
    }

    /// The total number of rendered terminal lines in this layout, after wrapping.
    pub fn wrapped_line_count(&self) -> u16 {
        if self.parts.is_empty() {
            0
        } else {
            self.parts.len() as u16 - 1
        }
    }

    /// The exclusive position on screen where this segment ends.
    pub fn end_position(&self) -> Option<AbsolutePosition> {
        Some(self.parts.last()?.end_position())
    }

    /// Computes the absolute position of the specified grapheme.
    pub fn get_absolute_position(&self, grapheme_index: usize) -> Option<AbsolutePosition> {
        for part_index in 0..self.parts.len() {
            let part_layout = &self.parts[part_index];

            if part_layout.end > grapheme_index
                || part_layout.end == grapheme_index && part_index + 1 == self.parts.len()
            {
                return part_layout.get_position(grapheme_index);
            }
        }

        None
    }
}

/// A part of a segment's layout once arranged on the screen.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PartLayout {
    position: AbsolutePosition,
    start: usize,
    end: usize,
    widths: Vec<usize>,
}

impl PartLayout {
    /// Part of a segment's layout. Represents the segment text's substring from grapheme indices
    /// `start` to `end`, inclusive and exclusive respectively, starting from `position`.
    pub(crate) fn new(
        position: AbsolutePosition,
        start: usize,
        end: usize,
        widths: Vec<usize>,
    ) -> Self {
        Self {
            position,
            start,
            end,
            widths,
        }
    }

    /// The position on screen where this part of the segment begins.
    pub fn position(&self) -> AbsolutePosition {
        self.position
    }

    /// The inclusive start index of the segment text substring included in this part of the layout.
    pub fn start(&self) -> usize {
        self.start
    }

    /// The exclusive end index of the segment text substring included in this part of the layout.
    pub fn end(&self) -> usize {
        self.end
    }

    /// The grapheme cluster widths for this part of the layout. There should be `length` widths.
    pub fn widths(&self) -> &Vec<usize> {
        &self.widths
    }

    /// The exclusive position on screen where this part of the segment ends.
    pub fn end_position(&self) -> AbsolutePosition {
        self.get_position(self.end).unwrap()
    }

    /// The grapheme length of this part.
    pub fn length(&self) -> usize {
        self.end - self.start
    }

    /// Compute the position of a specific index in this layout.
    pub fn get_position(&self, index: usize) -> Option<AbsolutePosition> {
        if index >= self.start && index <= self.end {
            let preceding_width = self.widths.iter().take(index - self.start).sum::<usize>() as i16;
            let position = self.position.add_columns(preceding_width);
            Some(position)
        } else {
            None
        }
    }
}
