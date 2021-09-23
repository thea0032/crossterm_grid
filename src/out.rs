/// Currently, an action is either printing a string or moving to a location.
/// The first value is the x location, the second is the y location.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action<'a> {
    Print(&'a str),
    MoveTo(usize, usize),
}
/// A handler is a structure that can convert actions into an output on an output device.
/// This simple trait is rather self-explanatory.
pub trait Handler {
    type OutputDevice;
    type Error;
    fn handle(&mut self, out: &mut Self::OutputDevice, input: &Action) -> Result<(), Self::Error>;
}
