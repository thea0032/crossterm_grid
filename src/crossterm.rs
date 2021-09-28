use std::io::Stdout;

use crate::out::{Action, Handler};

use crossterm::{cursor::MoveTo, execute, queue, style::Print};
/// A basic wrapper for crossterm. Turns this output into crossterm-based output. 
pub struct CrosstermHandler;
impl CrosstermHandler {
    /// Flushes any stray text into the terminal. 
    pub fn finish(out: &mut Stdout) -> Result<(), crossterm::ErrorKind> {
        execute!(out)
    }
}

impl Handler for CrosstermHandler {
    type OutputDevice = Stdout;
    type Error = crossterm::ErrorKind;
    fn handle(&mut self, out: &mut Self::OutputDevice, input: &Action) -> Result<(), Self::Error> {
        match input {
            Action::Print(v) => {
                queue!(out, Print(v))
            }
            Action::MoveTo(x, y) => {
                queue!(out, MoveTo(*x as u16, *y as u16))
            }
        }
    }
}
