use std::io::Stdout;

use crate::out::{Action, Handler};

use crossterm::{cursor::MoveTo, queue, style::Print};

struct CrosstermHandler;
impl Handler for CrosstermHandler {
    type OutputDevice = Stdout;
    type Error = crossterm::ErrorKind;
    fn handle(&mut self, out: &mut Self::OutputDevice, input: &Action) -> Result<(), Self::Error> {
        match input {
            Action::Print(v) => {
                queue!(out, Print(v))
            },
            Action::MoveTo(x, y) => {
                queue!(out, MoveTo(*x as u16, *y as u16))
            },
        }
    }
}