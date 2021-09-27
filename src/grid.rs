use crate::process::ChunkProcess;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Whether the alignment is in the negative direction [up/left] or in the positive direction [down/right].
/// Alignments will have different behaviors depending on where they're used.
pub enum Alignment {
    Minus,
    Plus,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum Maximum {
    None,
    X(usize, Alignment),
    Y(usize, Alignment),
}
impl Default for Maximum {
    fn default() -> Self {
        Maximum::None
    }
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
/// Inputting this to a grid will give a GridData based on the specifications used in the function.
/// # Examples
/// Creating a grid
/// ``` rust
/// # use ui_utils::out;
/// # use ui_utils::trim::Ignore;
/// # use ui_utils::grid::*;
/// # fn main() -> Result<(), ()>{
/// let mut grid = Grid::new(0, 0, 10, 10);
/// let chunk = grid.apply_strategy(&GridStrategy::new());
/// assert_eq!(chunk, Some(Chunk {start_x: 0, start_y: 0, end_x: 10, end_y: 10}));
/// # Ok(())
/// # }
/// ```
pub struct GridStrategy {
    min_size_x: Option<usize>,
    min_size_y: Option<usize>,
    max_size: Maximum,
}
impl GridStrategy {
    /// Creates an empty grid strategy. Empty grid strategies will simply take up the entire grid.
    /// # Examples
    /// The default grid:
    /// ``` rust
    /// # use ui_utils::out;
    /// # use ui_utils::trim::Ignore;
    /// # use ui_utils::grid::*;
    /// # fn main() -> Result<(), ()>{
    /// let mut grid = Grid::new(0, 0, 10, 10);
    /// let chunk = grid.apply_strategy(&GridStrategy::new());
    /// assert_eq!(chunk, Some(Chunk {start_x: 0, start_y: 0, end_x: 10, end_y: 10}));
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> GridStrategy {
        GridStrategy {
            min_size_x: None,
            min_size_y: None,
            max_size: Maximum::None,
        }
    }
    /// Sets a maximum X value. The resulting grid data will only be of length v.
    /// It'll be either on the left or the right, depending on the alignment (left = minus).
    /// Only one maximum direction can be set. Otherwise, this function will panic.
    /// # Examples
    /// Applying a grid with a maximum x value
    /// ``` rust
    /// # use ui_utils::out;
    /// # use ui_utils::trim::Ignore;
    /// # use ui_utils::grid::*;
    /// # fn main() -> Result<(), ()>{
    /// let mut grid = Grid::new(0, 0, 10, 10);
    /// let chunk = grid.apply_strategy(&GridStrategy::new().max_x(5, Alignment::Minus));
    /// assert_eq!(chunk, Some(Chunk {start_x: 0, start_y: 0, end_x: 5, end_y: 10}));
    /// let chunk = grid.apply_strategy(&GridStrategy::new().max_x(2, Alignment::Plus));
    /// assert_eq!(chunk, Some(Chunk {start_x: 8, start_y: 0, end_x: 10, end_y: 10}));
    /// # Ok(())
    /// # }
    /// ```
    /// Cannot set both max x and max y 
    /// ```should_panic
    /// # use ui_utils::out;
    /// # use ui_utils::trim::Ignore;
    /// # use ui_utils::grid::*;
    /// # fn main() -> Result<(), ()>{
    /// let cannot_set_both_x_and_y = GridStrategy::new().max_x(2, Alignment::Minus).max_y(1, Alignment::Plus);
    /// # Ok(())
    /// # }
    /// ```
    pub fn max_x(mut self, v: usize, a: Alignment) -> Self {
        if matches!(self.max_size, Maximum::None) {
            self.max_size = Maximum::X(v, a);
            self
        } else {
            panic!("A maximum already exists!")
        }
    }
    /// Sets a maximum Y value. The resulting grid data will only be of height v.
    /// It'll be either on the top or the bottom, depending on the alignment (top = minus).
    /// Only one maximum direction can be set. Otherwise, this function will panic.
    /// # Examples
    /// Applying a grid with a maximum x value
    /// ``` rust
    /// # use ui_utils::out;
    /// # use ui_utils::trim::Ignore;
    /// # use ui_utils::grid::*;
    /// # fn main() -> Result<(), ()>{
    /// let mut grid = Grid::new(0, 0, 10, 10);
    /// let chunk = grid.apply_strategy(&GridStrategy::new().max_y(5, Alignment::Minus));
    /// assert_eq!(chunk, Some(Chunk {start_x: 0, start_y: 0, end_x: 10, end_y: 5}));
    /// let chunk = grid.apply_strategy(&GridStrategy::new().max_y(2, Alignment::Plus));
    /// assert_eq!(chunk, Some(Chunk {start_x: 0, start_y: 8, end_x: 10, end_y: 10}));
    /// # Ok(())
    /// # }
    /// ```
    /// Cannot set both max x and max y 
    /// ```should_panic
    /// # use ui_utils::out;
    /// # use ui_utils::trim::Ignore;
    /// # use ui_utils::grid::*;
    /// # fn main() -> Result<(), ()>{
    /// let cannot_set_both_x_and_y = GridStrategy::new().max_x(2, Alignment::Minus).max_y(1, Alignment::Plus);
    /// # Ok(())
    /// # }
    /// ```
    pub fn max_y(mut self, v: usize, a: Alignment) -> Self {
        if matches!(self.max_size, Maximum::None) {
            self.max_size = Maximum::Y(v, a);
            self
        } else {
            panic!("A maximum already exists!")
        }
    }
    /// Sets a minimum X value. If the grid cannot give the grid data this amount of length,
    /// no strategy will be returned.
    /// # Examples
    /// ``` rust
    /// # use ui_utils::out;
    /// # use ui_utils::trim::Ignore;
    /// # use ui_utils::grid::*;
    /// # fn main() -> Result<(), ()>{
    /// let mut grid = Grid::new(0, 0, 10, 10);
    /// let chunk = grid.apply_strategy(&GridStrategy::new().min_x(15));
    /// assert_eq!(chunk, None);
    /// let chunk = grid.apply_strategy(&GridStrategy::new().min_x(5));
    /// assert_eq!(chunk, Some(Chunk {start_x: 0, start_y: 0, end_x: 10, end_y: 10}));
    /// # Ok(())
    /// # }
    /// ```
    pub fn min_x(mut self, v: usize) -> Self {
        self.min_size_x = Some(v);
        self
    }
    /// Sets a minimum Y value. If the grid cannot give the grid data this amount of height,
    /// no strategy will be returned.
    /// # Examples
    /// ``` rust
    /// # use ui_utils::out;
    /// # use ui_utils::trim::Ignore;
    /// # use ui_utils::grid::*;
    /// # fn main() -> Result<(), ()>{
    /// let mut grid = Grid::new(0, 0, 10, 10);
    /// let chunk = grid.apply_strategy(&GridStrategy::new().min_y(15));
    /// assert_eq!(chunk, None);
    /// let chunk = grid.apply_strategy(&GridStrategy::new().min_y(5));
    /// assert_eq!(chunk, Some(Chunk {start_x: 0, start_y: 0, end_x: 10, end_y: 10}));
    /// # Ok(())
    /// # }
    /// ```
    pub fn min_y(mut self, v: usize) -> Self {
        self.min_size_y = Some(v);
        self
    }
    #[doc(hidden)]
    /// Applies a grid strategy. This is meant to be indirectly called.
    fn apply_grid_strategy(&self, grid: &mut Grid) -> Option<Chunk> {
        if grid.start_x == grid.end_x || grid.start_y == grid.end_y {
            // no space left
            return None;
        }
        if let Some(val) = self.min_size_y {
            // below minimum size
            if grid.end_y <= grid.start_y + val {
                return None;
            }
        }
        if let Some(val) = self.min_size_x {
            // below minimum size
            if grid.end_x <= grid.start_x + val {
                return None;
            }
        }
        match &self.max_size {
            Maximum::None => {
                // Takes up the entire grid
                let return_value = Some(Chunk {
                    start_x: grid.start_x,
                    start_y: grid.start_y,
                    end_x: grid.end_x,
                    end_y: grid.end_y,
                });
                grid.start_x = grid.end_x;
                grid.start_y = grid.end_y;
                return_value
            }
            Maximum::X(size, alignment) => {
                let size = *size;
                let size = size.min(grid.end_x - grid.start_x);
                if matches!(alignment, Alignment::Minus) {
                    // Takes up the entire grid, up to the maximum size from the left.
                    let return_value = Some(Chunk {
                        start_x: grid.start_x,
                        start_y: grid.start_y,
                        end_x: grid.start_x + size,
                        end_y: grid.end_y,
                    });
                    grid.start_x += size;
                    return_value
                } else {
                    // Takes up the entire grid, up to the maximum size from the right.
                    let return_value = Some(Chunk {
                        start_x: grid.end_x - size,
                        start_y: grid.start_y,
                        end_x: grid.end_x,
                        end_y: grid.end_y,
                    });
                    grid.end_x -= size;
                    return_value
                }
            }
            Maximum::Y(size, alignment) => {
                let size = *size;
                let size = size.min(grid.end_y - grid.start_y);
                if matches!(alignment, Alignment::Minus) {
                    // Takes up the entire grid, up to the maximum size from the top.
                    let return_value = Some(Chunk {
                        start_x: grid.start_x,
                        start_y: grid.start_y,
                        end_x: grid.end_x,
                        end_y: grid.start_y + size,
                    });
                    grid.start_y += size;
                    return_value
                } else {
                    // Takes up the entire grid, up to the maximum size from the bottom.
                    let return_value = Some(Chunk {
                        start_x: grid.start_x,
                        start_y: grid.end_y - size,
                        end_x: grid.end_x,
                        end_y: grid.end_y,
                    });
                    grid.end_y -= size;
                    return_value
                }
            }
        }
    }
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// A grid - basically, a square meant to resemble a terminal. Chunks can be "bitten off" into GridData.
/// This reserves a chunk of the terminal to only be written on a certain space.
pub struct Grid {
    orig: Chunk,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
}
impl Grid {
    /// Creates a new grid. Usually, start_x and start_y should be 0.
    /// End_x and end_y are usually the size of your terminal.
    pub fn new(start_x: usize, start_y: usize, end_x: usize, end_y: usize) -> Grid {
        Grid {
            orig: Chunk {
                start_x,
                start_y,
                end_x,
                end_y,
            },
            start_x,
            start_y,
            end_x,
            end_y,
        }
    }
    /// Applies a grid strategy to the grid, returning data that is capable of writing to a chunk of the terminal.
    /// This will only be written on by that GridData.
    pub fn apply_strategy(&mut self, strategy: &GridStrategy) -> Option<Chunk> {
        strategy.apply_grid_strategy(self)
    }
    /// Updates the size of the terminal with new data. Will take effect next time the function new_frame is called.
    pub fn update_size(&mut self, new_size: Chunk) {
        self.orig = new_size;
    }
    /// Resets the grid, allowing the entire terminal to be "bitten off" and overwritten.
    pub fn new_frame(&mut self) {
        self.start_x = self.orig.start_x;
        self.start_y = self.orig.start_y;
        self.end_x = self.orig.end_x;
        self.end_y = self.orig.end_y;
    }
    /// Shows how much of the terminal is still on the grid.
    pub fn info(&self) -> Chunk {
        Chunk {
            start_x: self.start_x,
            start_y: self.start_y,
            end_x: self.end_x,
            end_y: self.end_y,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Where the divider will be placed. The divider is between two sides: A plus side and a minus side.
/// Content can be added on the plus or minus side if there's space available.
/// For examples of divider behavior, see docs for ChunkProcess.
pub enum DividerStrategy {
    Beginning,
    End,
    Halfway,
    Pos(usize),
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// A square chunk of the terminal. This is represented by
pub struct Chunk {
    pub start_x: usize,
    pub start_y: usize,
    pub end_x: usize,
    pub end_y: usize,
}
impl Chunk {
    /// Converts the chunk into a process. A process can have text added to it, and it can be printed.
    pub fn to_process(&self, strategy: DividerStrategy) -> ChunkProcess {
        ChunkProcess::new(self, strategy)
    }
    pub(crate) fn start_x(&self) -> usize {
        self.start_x
    }
    pub(crate) fn start_y(&self) -> usize {
        self.start_y
    }
    pub(crate) fn end_x(&self) -> usize {
        self.end_x
    }
    pub(crate) fn end_y(&self) -> usize {
        self.end_y
    }
}
