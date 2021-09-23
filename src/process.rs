use unicode_segmentation::UnicodeSegmentation;

use crate::{
    grid::{Alignment, Chunk, DividerStrategy},
    out::{Action, Handler},
};

/// Represents a formatting problem. Currently, the only problem that can occur is a lack of space.
/// Your string is given back.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FormatError {
    NoSpace(String),
}

enum InternalFormatError {
    NoSpace(TrimmedText),
}

/// Determines how text too big to fit is handled.
/// TrimStrategy::Cut truncates the value, removing any text that doesn't fit.
/// TrimStrategy::Split moves all extra text to extra lines.
/// TrimStrategy::Ignore ignores this, although you can't guarantee that other regions of the terminal will remain untouched.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrimStrategy {
    Cut,
    Split,
    Ignore,
}
enum TrimResult {
    WrapText(Vec<TrimmedText>),
    Text(TrimmedText),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TrimmedText(String);
/// A structure that can display text inside a chunk.  
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChunkProcess {
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
    divider: usize,
    minus: Vec<TrimmedText>,
    plus: Vec<TrimmedText>,
    example_str: String,
}
impl ChunkProcess {
    /// Creates a new chunk process.
    pub(crate) fn new(val: &Chunk, strategy: DividerStrategy) -> ChunkProcess {
        ChunkProcess {
            start_x: val.start_x(),
            start_y: val.start_y(),
            end_x: val.end_x(),
            end_y: val.end_y(),
            divider: match strategy {
                DividerStrategy::Beginning => 0,
                DividerStrategy::End => val.end_y() - val.start_y(),
                DividerStrategy::Halfway => (val.end_y() - val.start_y()) / 2,
                DividerStrategy::Pos(v) => v,
            },
            minus: Vec::new(),
            plus: Vec::new(),
            example_str: " ".chars().cycle().take(val.end_x() - val.start_x()).collect(),
        }
    }
    /// Trims a string using a trim strategy.
    fn trim(&self, s: String, b: TrimStrategy, a: Alignment) -> Result<TrimResult, InternalFormatError> {
        match b {
            TrimStrategy::Cut => {
                // Adds blank space to make sure that the entire length is spanned and no text from the previous frame remains
                let blank_space = " ".graphemes(true).cycle();
                // Any extra text above the length specified is gone.
                let i = s.graphemes(true).chain(blank_space).take(self.end_x - self.start_x);
                Ok(TrimResult::Text(TrimmedText(i.collect())))
            }
            TrimStrategy::Split => {
                let mut v = s.graphemes(true).collect::<Vec<_>>();
                if v.is_empty() {
                    v.push(" ");
                } // An empty string won't create a line break unless we do this.
                  // Stores the previous value
                let mut storage: &[&str] = &[];
                // The trimmed text result
                let mut res: Vec<TrimmedText> = Vec::new();
                for line in v.chunks(self.end_x - self.start_x) {
                    // each line, except for the last one, extends the entire chunk. We only need to add extra blank space on the next one.
                    // As long as there's an item after, we don't need to extend the line with blank space.
                    if storage.len() != 0 {
                        res.push(TrimmedText(storage.iter().copied().collect::<String>()));
                    }
                    storage = line;
                }
                // Creates a cycle of blank space to extend the line with until the end of the chunk (to make sure no extra text from the chunk stays).
                let blank_space = " ".graphemes(true).cycle();
                // Adds a TrimmedText value of exactly the right visual length.
                res.push(TrimmedText(
                    storage
                        .iter()
                        .copied()
                        .chain(blank_space)
                        .take(self.end_x - self.start_x)
                        .collect::<String>(),
                ));
                if matches!(a, Alignment::Minus) {
                    // Reverses the direction if we're in the minus direction.
                    res.reverse();
                }
                Ok(TrimResult::WrapText(res))
            }
            TrimStrategy::Ignore => {
                // DANGEROUS: Just passes the string into TrimmedText. This is bad if it doesn't fit - it could interfere with the other box.
                Ok(TrimResult::Text(TrimmedText(s)))
            }
        }
    }
    /// Converts an internal error (uses private types) into a normal error (uses public types).
    fn convert_errors(&self, val: InternalFormatError) -> FormatError {
        match val {
            InternalFormatError::NoSpace(v) => FormatError::NoSpace(v.0),
        }
    }
    /// Converts an internal error (uses private types) into a normal error (uses public types) and adds unprocessed text back.
    fn convert_supplemented(&self, val: InternalFormatError, supplement: Vec<TrimmedText>, a: Alignment) -> FormatError {
        match val {
            InternalFormatError::NoSpace(v) => {
                let mut s: String = v.0;
                for line in supplement {
                    if matches!(a, Alignment::Minus) {
                        let mut line = line.0;
                        line.push_str(&s);
                        s = line;
                    } else {
                        s.push_str(&line.0);
                    }
                }
                FormatError::NoSpace(s)
            }
        }
    }
    /// Adds multi-line content to the selection, using the inputted strategy inside the inputted alignment. Returns everything that can't fit.
    /// Note that the multi-line content goes top to bottom, even if Alignment::Minus is selected
    pub fn add_to_section_lines(&mut self, mut text: Vec<String>, strategy: TrimStrategy, section: Alignment) -> Vec<Result<(), FormatError>> {
        if matches!(section, Alignment::Minus) {
            text.reverse();
        }
        let mut res = text.into_iter()
            .map(|x| self.add_to_section(x, strategy, section))
            .collect::<Vec<_>>();
        if matches!(section, Alignment::Minus) {
            res.reverse();
        }
        res
    }
    /// Adds single-line content to the selection, using the inputted strategy inside the inputted alignment. Returns if it can't fit.
    pub fn add_to_section(&mut self, text: String, strategy: TrimStrategy, section: Alignment) -> Result<(), FormatError> {
        let text = self.trim(text, strategy, section).map_err(|x| self.convert_errors(x))?;
        match text {
            TrimResult::WrapText(v) => {
                let mut i = v.into_iter();
                let v: InternalFormatError = loop {
                    if let Some(val) = i.next() {
                        // If there's more trimmed text...
                        if let Err(e) = self.add_to_section_trimmed(val, section) {
                            // Adds it to the section. If an error occurs, break out of the loop.
                            break e;
                        }
                    } else {
                        // If we successfully made it through, we're ok.
                        return Ok(());
                    }
                };
                // There's still stuff that we haven't processed.
                let extras = i.collect::<Vec<_>>();
                // Converts the error.
                Err(self.convert_supplemented(v, extras, section))
            }
            TrimResult::Text(v) => self.add_to_section_trimmed(v, section).map_err(|e| self.convert_errors(e)),
        }
    }
    /// Adds trimmed text to a section.
    fn add_to_section_trimmed(&mut self, text: TrimmedText, section: Alignment) -> Result<(), InternalFormatError> {
        if matches!(section, Alignment::Minus) {
            let space = self.divider - self.minus.len();
            if space == 0 {
                return Err(InternalFormatError::NoSpace(text));
            }
            self.minus.push(text);
        } else {
            let space = self.end_y - self.start_y - self.divider - self.plus.len();
            if space == 0 {
                return Err(InternalFormatError::NoSpace(text));
            }
            self.plus.push(text);
        }
        Ok(())
    }
    /// Shoves the data in the positive or negative direction, changing the divider to make more space available on one side.
    pub fn shove(&mut self, direction: Alignment) {
        match direction {
            Alignment::Minus => self.divider = self.divider.min(self.minus.len()),
            Alignment::Plus => self.divider = self.divider.max(self.end_y - self.start_y + self.plus.len()),
        }
    }
    /// Prints out the chunk.
    fn grab_actions<'a>(&'a mut self) -> Vec<Action<'a>> {
        let mut result = Vec::new();
        let start_x = self.start_x;
        let start_y = self.divider - self.minus.len();
        let divider = self.divider;
        // Prints blank lines, making sure that the entirety of grid is clear.
        for i in self.start_y..start_y {
            result.push(Action::MoveTo(start_x, i));
            result.push(Action::Print(&self.example_str));
        }
        // Prints negative lines
        for (i, line) in self.minus.iter().rev().enumerate() {
            result.push(Action::MoveTo(start_x, start_y + i));
            result.push(Action::Print(&line.0));
        }
        // Prints positive lines
        for (i, line) in self.plus.iter().enumerate() {
            result.push(Action::MoveTo(start_x, divider + i));
            result.push(Action::Print(&line.0));
        }
        // Prints blank lines, making sure that the entirety of grid is clear.
        for i in self.start_y + self.divider + self.plus.len()..self.end_y {
            result.push(Action::MoveTo(start_x, i));
            result.push(Action::Print(&self.example_str));
        }
        result
    }
    /// Prints using a handler.
    pub fn print<H: Handler>(&mut self, handler: &mut H, out: &mut H::OutputDevice) -> Result<(), H::Error> {
        let actions = self.grab_actions();
        for line in actions {
            handler.handle(out, &line)?;
        }
        Ok(())
    }
}
