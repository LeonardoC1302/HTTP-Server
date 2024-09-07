use std::io::Write;
pub trait WriteTo {
    type Error;
    fn write_to<W: Write>(&self, f: &mut W) -> Result<(), Self::Error>;
}