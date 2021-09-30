use unicode_segmentation::UnicodeSegmentation;

use crate::grid::Frame;

/// Currently, an action is either printing a string or moving to a location.
/// The first value is the x location, the second is the y location.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action<'a> {
    Print(&'a str),
    MoveTo(usize, usize),
}
/**
A handler is a structure that can convert actions into an output on an output device.
This simple trait is rather self-explanatory.
# Example
``` rust
# use grid_ui::grid;
# use grid_ui::out;
# use grid_ui::trim::Ignore;
# fn main() -> Result<(), ()>{
let mut grid = grid::Frame::new(0, 0, 10, 4).next_frame();
let mut process = grid.into_process(grid::DividerStrategy::Halfway);
process.add_to_section("Some stuff".to_string(), &mut Ignore, grid::Alignment::Plus);
let mut some_handler = out::OutToString;
let mut output_device = String::new();
process.print(&mut some_handler, &mut output_device)?;
assert_eq!(output_device, "          \n          \nSome stuff\n          \n".to_string());
# Ok(())
# }
```
*/
pub trait Handler {
    type OutputDevice;
    type Error;
    fn handle(&mut self, out: &mut Self::OutputDevice, input: &Action) -> Result<(), Self::Error>;
}
/**
A handler that is "safe", ie doesn't return an error. All safe handlers are also handlers - you can use them as such. 
# Example
``` rust
# use grid_ui::grid;
# use grid_ui::out;
# use grid_ui::trim::Ignore;
# fn main() -> Result<(), ()>{
let mut grid = grid::Frame::new(0, 0, 10, 4).next_frame();
let mut process = grid.into_process(grid::DividerStrategy::Halfway);
process.add_to_section("Some stuff".to_string(), &mut Ignore, grid::Alignment::Plus);
let mut some_handler = out::OutToString;
let mut output_device = String::new();
process.print_safe(&mut some_handler, &mut output_device); // no need for the ? operator
assert_eq!(output_device, "          \n          \nSome stuff\n          \n".to_string());
# Ok(())
# }
```
*/
pub trait SafeHandler {
    type OutputDevice;
    fn safe_handle(&mut self, out: &mut Self::OutputDevice, input: &Action);
}
/**
A handler that outputs the text to a string, as lines. It does not pay attention to the location used.
This means that it won't panic at all, and will generally accept whatever text is thrown at it.
This makes it useful for debug purposes.
However, it doesn't do any formatting, and doesn't change behavior based on locations - only where you call it matters.
# Example
``` rust
# use grid_ui::grid;
# use grid_ui::out;
# use grid_ui::trim::Ignore;
# fn main() -> Result<(), ()>{
let mut grid = grid::Frame::new(0, 0, 10, 3).next_frame();
let mut process = grid.into_process(grid::DividerStrategy::Beginning);
process.add_to_section_lines(vec!["Some stuff".to_string(), "More stuff".to_string()].into_iter(), &mut Ignore, grid::Alignment::Plus);
let mut output: String = String::new();
process.print(&mut out::OutToString, &mut output)?;
assert_eq!("Some stuff\nMore stuff\n          \n".to_string(), output);
# Ok(())
# }
```
The limitations of this method
``` rust
# use grid_ui::grid;
# use grid_ui::out::*;
# use grid_ui::trim::Ignore;
# fn main() -> Result<(), ()>{
let frame = grid::Frame::new(0, 0, 10, 1);
let mut left = frame.next_frame();
let mut right = left.split(&grid::SplitStrategy::new().max_x(5, grid::Alignment::Plus)).ok_or(())?;
let mut left_process = left.into_process(grid::DividerStrategy::Beginning);
let mut right_process = right.into_process(grid::DividerStrategy::Beginning);
right_process.add_to_section("stuff".to_string(), &mut Ignore, grid::Alignment::Plus);
left_process.add_to_section("Some".to_string(), &mut Ignore, grid::Alignment::Plus);
let mut output: String = String::new();
right_process.print(&mut OutToString, &mut output)?;
left_process.print(&mut OutToString, &mut output)?;
assert_eq!("stuff\nSome\n".to_string(), output);
# Ok(())
# }
```
*/
pub struct OutToString;
impl SafeHandler for OutToString {
    type OutputDevice = String;
    fn safe_handle(&mut self, out: &mut String, input: &Action) {
        match input {
            Action::Print(s) => {
                out.push_str(s);
                out.push('\n')
            }
            Action::MoveTo(_, _) => {}
        }
    }
}
impl<H: SafeHandler> Handler for H {
    type OutputDevice = H::OutputDevice;
    type Error = ();
    fn handle(&mut self, out: &mut Self::OutputDevice, input: &Action) -> Result<(), Self::Error> {
        self.safe_handle(out, input);
        Ok(())
    }
}
/**
A more complicated version of the structure OutToString. This modifies a string buffer
instead of pushing any text directly to a string. This allows the structure to actually
process multiple grids in any order, at the expense of time cost.
# Panics
This structure will cause a panic if it tries to print text that cannot fit in the assigned buffer.
This will not happen unless you either (A) use a structure that doesn't force text to fit
the frame/grid such as trim::Ignore, or (B) construct this structure with starting or ending
points different from the frame it's used in.
# Examples
Basic usage
``` rust
# use grid_ui::grid;
# use grid_ui::out::*;
# use grid_ui::trim::Ignore;
# fn main() -> Result<(), ()>{
let frame = grid::Frame::new(0, 0, 10, 1);
let mut output: StringBuffer = StringBuffer::from_frame(&frame);
let mut left = frame.next_frame();
let mut right = left.split(&grid::SplitStrategy::new().max_x(5, grid::Alignment::Plus)).ok_or(())?;
let mut left_process = left.into_process(grid::DividerStrategy::Beginning);
let mut right_process = right.into_process(grid::DividerStrategy::Beginning);
left_process.add_to_section("Some".to_string(), &mut Ignore, grid::Alignment::Plus);
right_process.add_to_section("stuff".to_string(), &mut Ignore, grid::Alignment::Plus);
left_process.print(&mut output, &mut ())?;
right_process.print(&mut output, &mut ())?;
assert_eq!(vec!["Some stuff".to_string()], output.lines());
# Ok(())
# }
```
Panicking with ignore
``` should_panic
# use grid_ui::grid;
# use grid_ui::out::*;
# use grid_ui::trim::Ignore;
# fn main() -> Result<(), ()>{
let frame = grid::Frame::new(0, 0, 10, 1);
let mut output: StringBuffer = StringBuffer::from_frame(&frame);
let mut grid = frame.next_frame();
let mut process = grid.into_process(grid::DividerStrategy::Beginning);
process.add_to_section("This string is too long.".to_string(), &mut Ignore, grid::Alignment::Plus);
process.print(&mut output, &mut ())?; // panics
# Ok(())
# }
```
Panicking with a grid mismatch
``` should_panic
# use grid_ui::grid;
# use grid_ui::out::*;
# use grid_ui::trim::Truncate;
# fn main() -> Result<(), ()>{
let frame = grid::Frame::new(0, 0, 10, 1);
let mut small_output: StringBuffer = StringBuffer::new(5, 0, 10, 1);
let mut grid = frame.next_frame();
let mut process = grid.into_process(grid::DividerStrategy::Beginning);
process.add_to_section("This string is trimmed to fit here, but not on the string buffer.".to_string(), &mut Truncate, grid::Alignment::Plus);
process.print(&mut small_output, &mut ())?; // panics
# Ok(())
# }
```

*/
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringBuffer {
    pub contents: Vec<Vec<String>>,
    pub offset_x: usize,
    pub offset_y: usize,
    current_x: usize,
    current_y: usize,
}

impl StringBuffer {
    /// Creates a new StringBuffer from 4 dimensions. 
    pub fn new(min_x: usize, min_y: usize, max_x: usize, max_y: usize) -> StringBuffer {
        StringBuffer {
            contents: vec![vec![" ".to_string(); max_x - min_x]; max_y - min_y],
            current_x: 0,
            current_y: 0,
            offset_x: min_x,
            offset_y: min_y,
        }
    }
    /// Creates a new StringBuffer with the same dimensions as the frame inputted. 
    pub fn from_frame(f: &Frame) -> StringBuffer {
        let g = f.next_frame();
        StringBuffer::new(g.start_x, g.start_y, g.end_x, g.end_y)
    }
    /// Prints the StringBuffer.
    pub fn finalize(&self) {
        for line in &self.contents {
            for block in line {
                print!("{}", block);
            }
            println!();
        }
    }
    /// Returns the StringBuffer lines, collected into strings (instead of each grapheme being individually displayed)
    pub fn lines(self) -> Vec<String> {
        self.contents.into_iter().map(|x| x.into_iter().collect::<String>()).collect::<Vec<_>>()
    }
}
impl SafeHandler for StringBuffer {
    type OutputDevice = ();

    fn safe_handle(&mut self, _: &mut (), input: &Action) {
        match input {
            Action::Print(v) => {
                for (i, line) in v.grapheme_indices(true) {
                    self.contents[self.current_y][self.current_x + i] = line.to_string();
                }
            }
            Action::MoveTo(x, y) => {
                self.current_x = *x - self.offset_x;
                self.current_y = *y - self.offset_y;
            }
        }
    }
}
