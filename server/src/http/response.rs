use super::{mime_type, Headers, StatusCode, WriteTo};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Response {
    pub status: StatusCode,
    pub headers: Headers,
    pub body: Vec<u8>,
}

impl Response {
    pub fn ok(body: &str) -> Self {
        let body_len_str = body.len().to_string();
        let body_len_ref: &str = &body_len_str;
        Self {
            status: StatusCode::OK,
            headers: Headers::from(&vec![("Content-Length", body_len_ref)]),
            body: body.bytes().collect(),
        }
    }
}

impl WriteTo for Response {
    type Error = &'static str;

    fn write_to<W: Write>(&self, stream: &mut W) -> Result<(), Self::Error> {
        // Write status line
        stream
            .write_fmt(format_args!("HTTP/1.1 {}\n", self.status as u32))
            .or(Err("Failed sending status code"))?;
            
        // Write headers
        for (key, value) in self.headers.iter() {
            stream
                .write_fmt(format_args!("{}: {}\r\n", key, value)) // Use CRLF for headers
                .or(Err("Failed sending headers data"))?;
        }

        // Write a blank line to separate headers from body
        stream
            .write_all(b"\r\n") // Use CRLF for body separator
            .or(Err("Failed sending body separator"))?;

        // Write body
        stream
            .write_all(&self.body)
            .or(Err("Failed sending payload"))?;

        Ok(())
    }
}

