use std::{
    error::Error,
    fmt::{Debug, Display},
};

use unicode_segmentation::UnicodeSegmentation;

use crate::{grid::Alignment, process::DrawProcess};

/// Represents a formatting problem. Contains the original inputted string, restored as close to its original glory as possible. 
/// Note that some of the information in the string may be lost.
/// Currently, there's only one variant of this error, indicating a lack of space.
/// # Examples  
/// ``` rust
/// # use grid_ui::grid;
/// # use grid_ui::out;
/// # use grid_ui::trim::Ignore;
/// # use grid_ui::trim::FormatError;
/// # fn main() -> Result<(), ()>{
/// let mut grid = grid::Frame::new(0, 0, 10, 1).next_frame(); // creates a grid with one line
/// let mut process = grid.into_process(grid::DividerStrategy::Beginning);
/// process.add_to_section("Some stuff".to_string(), &mut Ignore, grid::Alignment::Plus);
/// let e = process.add_to_section("No more".to_string(), &mut Ignore, grid::Alignment::Plus).unwrap_err();
/// if let FormatError::NoSpace(val) = e {
///     println!("{:?}", val);
///     assert_eq!(val, "No more".to_string());    
/// }
/// # Ok(())
/// # }
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FormatError<T: TrimStrategy> {
    NoSpace(T::Input),
}
impl<T: TrimStrategy> Display for FormatError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatError::NoSpace(value) => write!(f, "No space found for {}", value),
        }
    }
}
impl<T: TrimStrategy> Error for FormatError<T> {}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
/// Trimmed text is text that is marked as processed and displayable.
/// It is only public so that users can create TrimStrategy objects other than the 3 provided.
/// It is not meant to be manually be created by anything other than a TrimStrategy.
pub struct TrimmedText(pub String);

/// This trait is used for debug purposes.
/// T implements DisplayAndDebug iff T implements Display and T implements Debug.
pub trait DisplayAndDebug
where
    Self: Display,
    Self: Debug,
{
}

impl<T> DisplayAndDebug for T
where
    T: Display,
    T: Debug,
{
}
/// A TrimStrategy can be used to trim inputs down into TrimmedText
pub trait TrimStrategy
where
    Self: DisplayAndDebug,
{
    type Input: DisplayAndDebug;
    /// Processes the string, allowing it to be properly displayed.
    /// For examples, see the three TrimStrategy structs below.
    /// This function generally shouldn't panic, and it should be marked clearly if it does.
    fn trim(&mut self, text: Self::Input, chunk: &DrawProcess, a: Alignment) -> Vec<TrimmedText>;
    /// Undoes processing of the string, allowing it to be used again (or in a different way) by the user.
    /// For examples, see the three TrimStrategy structs below.
    /// Any alterations and information loss should be marked clearly.
    /// This function generally shouldn't panic, and it should be marked clearly if it does.
    fn back(&mut self, text: Vec<TrimmedText>, _: &DrawProcess, a: Alignment) -> Self::Input;
}
#[derive(Debug)]
/// Useful for debug purposes, or for quick code. Bypasses the grid restrictions entirely.
/// Does absolutely nothing to the text. This could potentially lead to bad formatting.
/// Bad formatting is what this crate is designed to prevent.
/// # Example
/// ``` rust
/// # use grid_ui::grid;
/// # use grid_ui::out;
/// # use grid_ui::trim::Ignore;
/// # use grid_ui::trim::TrimStrategy;
/// # use grid_ui::trim::TrimmedText;
/// # fn main() -> Result<(), ()>{
/// let mut grid = grid::Frame::new(0, 0, 10, 3).next_frame();
/// let mut process = grid.into_process(grid::DividerStrategy::Beginning);
/// let v = Ignore.trim("small".to_string(), &process, grid::Alignment::Plus);
/// assert_eq!(vec![TrimmedText("small".to_string())], v);
/// let v = Ignore.trim("This fits.".to_string(), &process, grid::Alignment::Plus);
/// assert_eq!(vec![TrimmedText("This fits.".to_string())], v);
/// let v = Ignore.trim("This is a really long line that will break things in a terminal setup.".to_string(), &process, grid::Alignment::Plus);
/// assert_eq!(vec![TrimmedText("This is a really long line that will break things in a terminal setup.".to_string())], v);
/// # Ok(())
/// # }
/// ```
pub struct Ignore;
impl Display for Ignore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", Ignore)
    }
}
impl TrimStrategy for Ignore {
    type Input = String;
    fn trim(&mut self, text: String, _: &DrawProcess, _: Alignment) -> Vec<TrimmedText> {
        vec![TrimmedText(text)]
    }

    fn back(&mut self, text: Vec<TrimmedText>, _: &DrawProcess, _: Alignment) -> Self::Input {
        text.into_iter().next().expect("Safe unwrap").0
    }
}
#[derive(Debug)]
/// The trim strategy cuts out anything that doesn't fit into the box in order to deal with grid restrictions.
/// It also adds blank space to any short lines to make sure every bit of blank space is refreshed.
/// # Example
/// ``` rust
/// # use grid_ui::grid;
/// # use grid_ui::out;
/// # use grid_ui::trim::Truncate;
/// # use grid_ui::trim::TrimStrategy;
/// # use grid_ui::trim::TrimmedText;
/// # fn main() -> Result<(), ()>{
/// let mut grid = grid::Frame::new(0, 0, 10, 3).next_frame();
/// let mut process = grid.into_process(grid::DividerStrategy::Beginning);
/// let v = Truncate.trim("small".to_string(), &process, grid::Alignment::Plus);
/// assert_eq!(vec![TrimmedText("small     ".to_string())], v);
/// let v = Truncate.trim("This fits.".to_string(), &process, grid::Alignment::Plus);
/// assert_eq!(vec![TrimmedText("This fits.".to_string())], v);
/// let v = Truncate.trim("This is a really long line that will break things in a terminal setup.".to_string(), &process, grid::Alignment::Plus);
/// assert_eq!(vec![TrimmedText("This is a ".to_string())], v);
/// # Ok(())
/// # }
/// ```
pub struct Truncate;
impl Display for Truncate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", Ignore)
    }
}
impl TrimStrategy for Truncate {
    type Input = String;
    fn trim(&mut self, text: String, chunk: &DrawProcess, _: Alignment) -> Vec<TrimmedText> {
        let blank_space = " ".graphemes(true).cycle();
        let res = text.graphemes(true).chain(blank_space).take(chunk.width()).collect();
        vec![TrimmedText(res)]
    }
    fn back(&mut self, text: Vec<TrimmedText>, _: &DrawProcess, _: Alignment) -> Self::Input {
        text.into_iter().next().expect("Safe unwrap").0
    }
}
#[derive(Debug)]
/// This split splits the text into different lines, each of which fit just fine.
/// It also adds blank space to any short lines to make sure every bit of blank space is refreshed.
/// # Panics
/// Panics if printing to a grid of 0 width.
/// # Example
/// ``` rust
/// # use grid_ui::grid;
/// # use grid_ui::out;
/// # use grid_ui::trim::Split;
/// # use grid_ui::trim::TrimStrategy;
/// # use grid_ui::trim::TrimmedText;
/// # fn main() -> Result<(), ()>{
/// let mut grid = grid::Frame::new(0, 0, 10, 3).next_frame();
/// let mut process = grid.into_process(grid::DividerStrategy::Beginning);
/// let v = Split.trim("small".to_string(), &process, grid::Alignment::Plus);
/// assert_eq!(vec![TrimmedText("small     ".to_string())], v);
/// let v = Split.trim("This fits.".to_string(), &process, grid::Alignment::Plus);
/// assert_eq!(vec![TrimmedText("This fits.".to_string())], v);
/// let v = Split.trim("This is a little too big..".to_string(), &process, grid::Alignment::Plus);
/// assert_eq!(vec![TrimmedText("This is a ".to_string()), TrimmedText("little too".to_string()), TrimmedText(" big..    ".to_string())], v);
/// # Ok(())
/// # }
/// ```
pub struct Split;
impl Display for Split {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", Ignore)
    }
}
impl TrimStrategy for Split {
    type Input = String;
    fn trim(&mut self, text: String, chunk: &DrawProcess, a: Alignment) -> Vec<TrimmedText> {
        let mut v = text.graphemes(true).collect::<Vec<_>>();
        if v.is_empty() {
            v.push(" ");
        } // An empty string won't create a line break unless we do this.
          // Stores the previous value
        let mut storage: &[&str] = &[];
        // The trimmed text result
        let mut res: Vec<TrimmedText> = Vec::new();
        for line in v.chunks(chunk.width()) {
            // each line, except for the last one, extends the entire grid. We only need to add extra blank space on the next one.
            // As long as there's an item after, we don't need to extend the line with blank space.
            if !storage.is_empty() {
                res.push(TrimmedText(storage.iter().copied().collect::<String>()));
            }
            storage = line;
        }
        // Creates a cycle of blank space to extend the line with until the end of the chunk (to make sure no extra text from the chunk stays).
        let blank_space = " ".graphemes(true).cycle();
        // Adds a TrimmedText value of exactly the right visual length.
        res.push(TrimmedText(
            storage.iter().copied().chain(blank_space).take(chunk.width()).collect::<String>(),
        ));
        if matches!(a, Alignment::Minus) {
            // Reverses the direction if we're in the minus direction.
            res.reverse();
        }
        res
    }
    fn back(&mut self, text: Vec<TrimmedText>, _: &DrawProcess, a: Alignment) -> Self::Input {
        if text.is_empty() {
            panic!("This shouldn't be an error!");
        }
        let mut res = String::new();
        for line in text {
            if matches!(a, Alignment::Minus) {
                let mut line = line.0;
                line.push_str(&res);
                res = line;
            } else {
                res.push_str(&line.0);
            }
        }
        res
    }
}
