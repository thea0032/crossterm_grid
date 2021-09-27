/// Currently, an action is either printing a string or moving to a location.
/// The first value is the x location, the second is the y location.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
pub trait SafeHandler {
    type OutputDevice;
    fn safe_handle(&mut self, out: &mut Self::OutputDevice, input: &Action);
}
pub struct OutToString;
impl SafeHandler for OutToString {
    type OutputDevice = String;
    fn safe_handle(&mut self, out: &mut String, input: &Action) {
        match input {
            Action::Print(s) => {out.push_str(s); out.push('\n')},
            Action::MoveTo(_, _) => {},
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