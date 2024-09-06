use std::io::BufRead;

// use crate::method::Method;
// use crate::path::Path;
// use crate::read_from::ReadFrom;
use super::{Method, Path, Headers, ReadFrom};

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: Path,
    pub headers : Headers,
    pub body: String
}

// Create request from vector
impl ReadFrom for Request {
    type Error = &'static str;
    fn read_from<R: BufRead>(stream: &mut R) -> Result<Self, Self::Error> {
        let mut buff: [u8; 4000] = [0; 4000];
        let l = stream.read(&mut buff).or(Err("failed receiving request"))?;
        let buff_str = String::from_utf8_lossy(&buff[0..l]);
        let mut buff_str_splitted = buff_str.split('\n');

        let first_line = match buff_str_splitted.next() {
            Some(line) => line,
            None => return Err("Empty request")
        };

        let mut first_line_splitted = first_line.split_whitespace();

        let method = Method::from(match first_line_splitted.next(){
            Some(method) => method,
            None => return Err("Empty request")
        });

        let path = match first_line_splitted.next(){
            Some(path) => Path::new(path.to_string()),
            None => return Err("Empty request")
        };

        let headers = Headers::try_from(&mut buff_str_splitted).or(Err("Failed parsing headers"))?;

        let body = buff_str_splitted.collect::<Vec<&str>>().join("\n");

        Ok(Self {
            method,
            path,
            headers,
            body
        })
    }
}