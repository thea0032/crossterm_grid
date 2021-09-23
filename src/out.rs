pub enum Action <'a>{
    Print(&'a str),
    MoveTo(usize, usize),
}
pub trait Handler {
    type OutputDevice;
    type Error;
    fn handle(&mut self, out: &mut Self::OutputDevice, input: &Action) -> Result<(), Self::Error>;
}