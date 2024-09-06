use std::io::{BufRead, Read};
pub trait ReadFrom {
    type Error;
    fn read_from<R: Read + BufRead>(f: &mut R) -> Result<Self, Self::Error>
    where
        Self: Sized;
}