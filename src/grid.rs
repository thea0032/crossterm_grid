use crate::process::DrawProcess;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// This is a frame. It stores the terminal's size in a convenient place.
/// It isn't stored in a grid, as grids are altered when they're split.
/// For examples, see the frame's methods.
pub struct Frame {
    grid: Grid,
}
impl Frame {
    /**
    Creates a new frame.
    # Example
    ``` rust
    # use ui_utils::grid::Frame;
    # fn main() {
    let ten_by_ten: Frame = Frame::new(0, 0, 10, 10);
    # }
    ```
    */
    pub fn new(x_min: usize, y_min: usize, x_max: usize, y_max: usize) -> Frame {
        Frame {
            grid: Grid {
                start_x: x_min,
                start_y: y_min,
                end_x: x_max,
                end_y: y_max,
            },
        }
    }
    /**
    Produces a fresh grid, which contains the entire frame.
    # Example
    ``` rust
    # use ui_utils::grid::Frame;
    # use ui_utils::grid::Grid;
    # fn main() {
    let ten_by_ten: Frame = Frame::new(0, 0, 10, 10);
    let ten_by_ten_grid: Grid = ten_by_ten.next_frame();
    assert_eq!(ten_by_ten_grid, Grid {start_x: 0, start_y: 0, end_x: 10, end_y: 10});
    # }
    ```
    */
    pub fn next_frame(&self) -> Grid {
        self.grid.clone()
    }
    /**
    Resizes the grid, changing its size.
    # Example
    ``` rust
    # use ui_utils::grid::Frame;
    # use ui_utils::grid::Grid;
    # fn main() {
    let mut ten_by_ten: Frame = Frame::new(0, 0, 10, 10);
    let ten_by_ten_grid: Grid = ten_by_ten.next_frame();
    assert_eq!(ten_by_ten_grid, Grid {start_x: 0, start_y: 0, end_x: 10, end_y: 10});
    ten_by_ten.resize(5, 5, 10, 10);
    let five_by_five_grid: Grid = ten_by_ten.next_frame();
    assert_eq!(five_by_five_grid, Grid {start_x: 5, start_y: 5, end_x: 10, end_y: 10});
    # }
    ```
    */
    pub fn resize(&mut self, x_min: usize, y_min: usize, x_max: usize, y_max: usize) {
        self.grid = Grid {
            start_x: x_min,
            start_y: y_min,
            end_x: x_max,
            end_y: y_max,
        }
    }
}
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
/**
Inputting this to a grid will give a GridData based on the specifications used in the function.
# Examples
Creating a grid
``` rust
# use ui_utils::out;
# use ui_utils::trim::Ignore;
# use ui_utils::grid::*;
# fn main() -> Result<(), ()>{
let mut grid = Frame::new(0, 0, 10, 10).next_frame();
let chunk = grid.split(&SplitStrategy::new());
assert_eq!(chunk, Some(Grid {start_x: 0, start_y: 0, end_x: 10, end_y: 10}));
# Ok(())
# }
```
*/
pub struct SplitStrategy {
    min_size_x: Option<usize>,
    min_size_y: Option<usize>,
    max_size: Maximum,
}
impl SplitStrategy {
    /**
    Creates an empty split strategy. Empty strategies will simply take up the entire grid when used. 
    # Examples
    The default grid:
    ``` rust
    # use ui_utils::out;
    # use ui_utils::trim::Ignore;
    # use ui_utils::grid::*;
    # fn main() -> Result<(), ()>{
    let mut grid = Frame::new(0, 0, 10, 10).next_frame();
    let chunk = grid.split(&SplitStrategy::new());
    assert_eq!(chunk, Some(Grid {start_x: 0, start_y: 0, end_x: 10, end_y: 10}));
    # Ok(())
    # }
    ```
    */
    pub fn new() -> SplitStrategy {
        SplitStrategy {
            min_size_x: None,
            min_size_y: None,
            max_size: Maximum::None,
        }
    }
    /**
    Sets a maximum X value. The resulting grid will only be at most of length v.
    It'll be either on the left or the right, depending on the alignment (left = minus).
    # Panics
    Only one maximum direction can be set. Otherwise, this function will panic.
    This is intended. 
    # Examples
    Applying a grid with a maximum x value
    ``` rust
    # use ui_utils::out;
    # use ui_utils::trim::Ignore;
    # use ui_utils::grid::*;
    # fn main() -> Result<(), ()>{
    let mut grid = Frame::new(0, 0, 10, 10).next_frame();
    let chunk = grid.split(&SplitStrategy::new().max_x(5, Alignment::Minus));
    assert_eq!(chunk, Some(Grid {start_x: 0, start_y: 0, end_x: 5, end_y: 10}));
    let chunk = grid.split(&SplitStrategy::new().max_x(2, Alignment::Plus));
    assert_eq!(chunk, Some(Grid {start_x: 8, start_y: 0, end_x: 10, end_y: 10}));
    # Ok(())
    # }
    ```
    This function will panic - you can't set two maximums. 
    ```should_panic
    # use ui_utils::out;
    # use ui_utils::trim::Ignore;
    # use ui_utils::grid::*;
    # fn main() -> Result<(), ()>{
    let cannot_set_both_x_and_y = SplitStrategy::new().max_x(2, Alignment::Minus).max_y(1, Alignment::Plus);
    # Ok(())
    # }
    ```
    */
    pub fn max_x(mut self, v: usize, a: Alignment) -> Self {
        if matches!(self.max_size, Maximum::None) {
            self.max_size = Maximum::X(v, a);
            self
        } else {
            panic!("A maximum already exists!")
        }
    }
    /**
    Sets a maximum Y value. The resulting grid data will only be of height v.
    It'll be either on the top or the bottom, depending on the alignment (top = minus).
    # Panics
    Only one maximum direction can be set. Otherwise, this function will panic.
    This is intended. 
    # Examples
    Applying a grid with a maximum x value
    ``` rust
    # use ui_utils::out;
    # use ui_utils::trim::Ignore;
    # use ui_utils::grid::*;
    # fn main() -> Result<(), ()>{
    let mut grid = Frame::new(0, 0, 10, 10).next_frame();
    let chunk = grid.split(&SplitStrategy::new().max_y(5, Alignment::Minus));
    assert_eq!(chunk, Some(Grid {start_x: 0, start_y: 0, end_x: 10, end_y: 5}));
    let chunk = grid.split(&SplitStrategy::new().max_y(2, Alignment::Plus));
    assert_eq!(chunk, Some(Grid {start_x: 0, start_y: 8, end_x: 10, end_y: 10}));
    # Ok(())
    # }
    ```
    This function will panic - you can't set two maximums. 
    ```should_panic
    # use ui_utils::out;
    # use ui_utils::trim::Ignore;
    # use ui_utils::grid::*;
    # fn main() -> Result<(), ()>{
    let cannot_set_both_x_and_y = SplitStrategy::new().max_x(2, Alignment::Minus).max_y(1, Alignment::Plus);
    # Ok(())
    # }
    ```
    */
    pub fn max_y(mut self, v: usize, a: Alignment) -> Self {
        if matches!(self.max_size, Maximum::None) {
            self.max_size = Maximum::Y(v, a);
            self
        } else {
            panic!("A maximum already exists!")
        }
    }
    /**
    Sets a minimum X value. If the grid cannot give the grid data this amount of length,
    no strategy will be returned.
    # Examples
    ``` rust
    # use ui_utils::out;
    # use ui_utils::trim::Ignore;
    # use ui_utils::grid::*;
    # fn main() -> Result<(), ()>{
    let mut grid = Frame::new(0, 0, 10, 10).next_frame();
    let chunk = grid.split(&SplitStrategy::new().min_x(15));
    assert_eq!(chunk, None);
    let chunk = grid.split(&SplitStrategy::new().min_x(5));
    assert_eq!(chunk, Some(Grid {start_x: 0, start_y: 0, end_x: 10, end_y: 10}));
    # Ok(())
    # }
    ```
    */
    pub fn min_x(mut self, v: usize) -> Self {
        self.min_size_x = Some(v);
        self
    }
    /**
    Sets a minimum Y value. If the grid cannot give the grid data this amount of height,
    no strategy will be returned.
    # Examples
    ``` rust
    # use ui_utils::out;
    # use ui_utils::trim::Ignore;
    # use ui_utils::grid::*;
    # fn main() -> Result<(), ()>{
    let mut grid = Frame::new(0, 0, 10, 10).next_frame();
    let chunk = grid.split(&SplitStrategy::new().min_y(15));
    assert_eq!(chunk, None);
    let chunk = grid.split(&SplitStrategy::new().min_y(5));
    assert_eq!(chunk, Some(Grid {start_x: 0, start_y: 0, end_x: 10, end_y: 10}));
    # Ok(())
    # }
    ```
    */
    pub fn min_y(mut self, v: usize) -> Self {
        self.min_size_y = Some(v);
        self
    }
    #[doc(hidden)]
    /// Applies a split strategy. This is meant to be indirectly called.
    fn apply(&self, grid: &mut Grid) -> Option<Grid> {
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
                let return_value = Some(Grid::new(grid.start_x, grid.start_y, grid.end_x, grid.end_y));
                grid.start_x = grid.end_x;
                grid.start_y = grid.end_y;
                return_value
            }
            Maximum::X(size, alignment) => {
                let size = *size;
                let size = size.min(grid.end_x - grid.start_x);
                if matches!(alignment, Alignment::Minus) {
                    // Takes up the entire grid, up to the maximum size from the left.
                    let return_value = Some(Grid::new(grid.start_x, grid.start_y, grid.start_x + size, grid.end_y));
                    grid.start_x += size;
                    return_value
                } else {
                    // Takes up the entire grid, up to the maximum size from the right.
                    let return_value = Some(Grid::new(grid.end_x - size, grid.start_y, grid.end_x, grid.end_y));
                    grid.end_x -= size;
                    return_value
                }
            }
            Maximum::Y(size, alignment) => {
                let size = *size;
                let size = size.min(grid.end_y - grid.start_y);
                if matches!(alignment, Alignment::Minus) {
                    // Takes up the entire grid, up to the maximum size from the top.
                    let return_value = Some(Grid::new(grid.start_x, grid.start_y, grid.end_x, grid.start_y + size));
                    grid.start_y += size;
                    return_value
                } else {
                    // Takes up the entire grid, up to the maximum size from the bottom.
                    let return_value = Some(Grid::new(grid.start_x, grid.end_y - size, grid.end_x, grid.end_y));
                    grid.end_y -= size;
                    return_value
                }
            }
        }
    }
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// A grid - basically, a square meant to resemble a portion of a terminal. Can be split up into other grids.
/// Cloning a grid is bad practice! Use it only if you must.
pub struct Grid {
    pub start_x: usize,
    pub start_y: usize,
    pub end_x: usize,
    pub end_y: usize,
}
impl Grid {
    fn new(start_x: usize, start_y: usize, end_x: usize, end_y: usize) -> Grid {
        Grid {
            start_x,
            start_y,
            end_x,
            end_y,
        }
    }
    /**
    Splits the grid into two others based on a SplitStrategy.
    With the default split strategy, the entire grid will go into the returned grid, leaving the first one empty.
    Expect to use this function a lot.
    # Return value
    Returns None if no new grid can be created - either because the grid is already empty or because it's below the minimum size.
    # Examples
    ``` rust
    # use ui_utils::out;
    # use ui_utils::trim::Ignore;
    # use ui_utils::grid::*;
    # fn main() -> Result<(), ()>{
    let mut grid = Frame::new(0, 0, 10, 10).next_frame();
    let second = grid.split(&SplitStrategy::new().max_y(5, Alignment::Minus));
    assert_eq!(second, Some(Grid {start_x: 0, start_y: 0, end_x: 10, end_y: 5}));
    assert_eq!(grid, Grid {start_x: 0, start_y: 5, end_x: 10, end_y: 10});
    let cant_be_made = grid.split(&SplitStrategy::new().min_y(6));
    assert_eq!(cant_be_made, None);
    let takes_up_all = grid.split(&SplitStrategy::new());
    assert_eq!(takes_up_all, Some(Grid {start_x: 0, start_y: 5, end_x: 10, end_y: 10}));
    assert_eq!(grid, Grid {start_x: 10, start_y: 10, end_x: 10, end_y: 10});
    let cant_be_made = grid.split(&SplitStrategy::new());
    assert_eq!(cant_be_made, None);
    # Ok(())
    # }
    ```
    */
    pub fn split(&mut self, strategy: &SplitStrategy) -> Option<Grid> {
        strategy.apply(self)
    }
    /**
    Extends the grid in the either direction, either positive or negative, if the input is compatible
    (ie grids are next to each other and of similar dimensions)
    If the two grids are incompatible, it returns an error and gives the grid back. 
    # Example
    ``` rust
    # use ui_utils::grid;
    # use ui_utils::out;
    # use ui_utils::trim::Ignore;
    # fn main() -> Result<(), ()>{
    let mut grid = grid::Frame::new(0, 0, 10, 10).next_frame();
    let mut second_grid = grid.split(&grid::SplitStrategy::new().max_y(5, grid::Alignment::Plus)).ok_or(())?;
    assert_eq!(grid.end_y, 5);
    assert!(grid.extend(second_grid).is_ok());
    assert_eq!(grid.end_y, 10);
    let incompatible_grid = grid::Frame::new(4, 4, 8, 8).next_frame();
    assert!(grid.extend(incompatible_grid).is_err());
    # Ok(())
    # }
    ```
    */
    
    pub fn extend(&mut self, grid: Grid) -> Result<(), Grid> {
        if self.start_x == grid.start_x && self.end_x == grid.end_x {
            if self.end_y == grid.start_y {
                self.end_y = grid.end_y;
                return Ok(())
            }
            if self.start_y == grid.end_y {
                self.start_y = grid.start_y;
                return Ok(())
            }
        }
        if self.start_y == grid.start_y && self.end_y == grid.end_y {
            if self.end_x == grid.start_x {
                self.end_x = grid.end_x;
                return Ok(())
            }
            if self.start_x == grid.end_x {
                self.start_x = grid.start_x;
                return Ok(())
            }
        }
        Err(grid)
    }
    /**
    Converts the grid into a DrawProcess. The draw process can then be used to draw onto the terminal.
    # Examples
    ``` rust
    # use ui_utils::out;
    # use ui_utils::trim::Truncate;
    # use ui_utils::grid::*;
    # fn main() -> Result<(), ()>{
    let mut grid = Frame::new(0, 0, 10, 10).next_frame();
    let mut process = grid.into_process(DividerStrategy::End);
    process.add_to_section("Some text".to_string(), &mut Truncate, Alignment::Minus);
    # Ok(())
    # }
    ```
    */
    pub fn into_process(self, strategy: DividerStrategy) -> DrawProcess {
        DrawProcess::new(self, strategy)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Where the divider will be placed. The divider is between two sides: A plus side and a minus side.
/// Content can be added on the plus or minus side if there's space available.
/// For examples of divider behavior, see docs for DrawProcess.
pub enum DividerStrategy {
    Beginning,
    End,
    Halfway,
    Pos(usize),
}
