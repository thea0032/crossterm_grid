use std::io::Stdout;

use unicode_segmentation::UnicodeSegmentation;

use crate::grid::{Alignment, DividerStrategy, Chunk};

#[derive(Debug)]
/// Represents a formatting problem. Currently, the only problem that can occur is a lack of space. 
/// Your string is given back. 
pub enum FormatError {
    NoSpace(String),
}
#[derive(Debug, Clone)]
enum InternalFormatError {
    NoSpace(TrimmedText),
}

#[derive(Clone, Copy, Debug)]
/// Determines how text too big to fit is handled. 
/// TrimStrategy::Cut truncates the value, removing any text that doesn't fit. 
/// TrimStrategy::Split moves all extra text to extra lines. 
/// TrimStrategy::Ignore ignores this, although this kind of defeats the point of the entire system I made. 
pub enum TrimStrategy {
    Cut,
    Split,
    Ignore,
}
enum TrimResult {
    WrapText(Vec<TrimmedText>),
    Text(TrimmedText)
}
#[derive(Clone, Debug)]
struct TrimmedText(String);
/// A structure that can display text inside a chunk.  
pub struct ChunkProcess {
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
    divider: usize,
    minus: Vec<TrimmedText>,
    plus: Vec<TrimmedText>,
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
            DividerStrategy::Beginning => val.start_y(),
            DividerStrategy::End => val.end_y(),
            DividerStrategy::Halfway => (val.start_y() + val.end_y()) / 2,
            DividerStrategy::Pos(v) => val.start_y() + v,
        },
        minus: Vec::new(),
        plus: Vec::new(),
        }
    }
    /// Trims a string using a trim strategy. 
    fn trim(&self, s: String, b: TrimStrategy, a: Alignment) -> Result<TrimResult, InternalFormatError> {
        match b {
            TrimStrategy::Cut => {
                let blank_space = " ".graphemes(true).cycle();
                let i = s.graphemes(true).chain(blank_space).take(self.end_x - self.start_x);
                Ok(TrimResult::Text(TrimmedText(i.collect())))
            },
            TrimStrategy::Split => {
                let mut v = s.graphemes(true).collect::<Vec<_>>();
                if v.is_empty() {
                    v.push(" ");
                }
                let mut storage: &[&str] = &[];
                let mut res: Vec<TrimmedText> = Vec::new();
                for line in v.chunks(self.end_x - self.start_x) {
                    if storage.len() != 0 {
                        res.push(TrimmedText(storage.iter().copied().collect::<String>()));
                    }
                    storage = line;
                }
                let blank_space = " ".graphemes(true).cycle();
                res.push(TrimmedText(storage.iter().copied().chain(blank_space).take(self.end_x - self.start_x).collect::<String>()));
                if matches!(a, Alignment::Minus) {
                    res.reverse();
                }
                Ok(TrimResult::WrapText(res))
            },
            TrimStrategy::Ignore => {
                Ok(TrimResult::Text(TrimmedText(s)))
            },
        }
    }
    fn convert_errors(&self, val: InternalFormatError) -> FormatError {
        match val {
            InternalFormatError::NoSpace(v) => FormatError::NoSpace(v.0),
        }
    }
    fn convert_supplemented(&self, val: InternalFormatError, supplement: Vec<TrimmedText>, a: Alignment) -> FormatError {
        match val {
            InternalFormatError::NoSpace(v) => {
                let mut s:String = v.0;
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
            },
        }
    }
    /// Adds multi-line content to the selection, using the inputted strategy inside the inputted alignment. Returns everything that can't fit. 
    /// Note that the multi-line content goes top to bottom, even if Alignment::Minus is selected
    pub fn add_to_section_lines(&mut self, mut text: Vec<String>, strategy: TrimStrategy, section: Alignment) -> Vec<Result<(), FormatError>> {
        if matches!(section, Alignment::Minus) {
            text.reverse();
        }
        let mut res = text.into_iter().map(|x| self.add_to_section(x, strategy, section)).collect::<Vec<_>>();
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
                //println!("{:?}", v);
                let mut i = v.into_iter();
                let v: InternalFormatError = loop {
                    //println!("IN LOOP");
                    if let Some(val) = i.next() {
                        //println!("{:?}", val);
                        if let Err(e) = self.add_to_section_trimmed(val, section) {
                            //println!("{:?}", e);
                            break e;
                        }
                    } else {
                        return Ok(());
                    }
                };
                let extras = i.collect::<Vec<_>>();
                Err(self.convert_supplemented(v, extras, section))
            },
            TrimResult::Text(v) => self.add_to_section_trimmed(v, section).map_err(|e| self.convert_errors(e)),
        }
    }
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
    pub fn print(&self, out: &mut Stdout) -> crossterm::Result<()>{
        let start_x = self.start_x;
        let start_y = self.divider - self.minus.len();
        let divider = self.divider;
        let v = &self.minus;
        for (i, line) in v.into_iter().rev().enumerate() {
            crossterm::queue!(out, crossterm::cursor::MoveTo((start_x) as u16, (start_y + i) as u16), crossterm::style::Print(&line.0))?;
        }
        for (i, line) in (&self.plus).into_iter().enumerate() {
            crossterm::queue!(out, crossterm::cursor::MoveTo((start_x) as u16, (divider + i) as u16), crossterm::style::Print(&line.0))?;
        }
        Ok(())
    }
}
