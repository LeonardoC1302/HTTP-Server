use super::{mime_type, Headers, StatusCode, WriteTo};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

/// Respuesta HTTP
pub struct Response {
    pub status: StatusCode,
    pub headers: Headers,
    pub body: Vec<u8>,
}

impl Response {
    /// Respuesta de redirección
    pub fn redirect(path: &str) -> Self {
        Self {
            status: StatusCode::REDIRECT,
            headers: Headers::from(&vec![("Content-Type", "text/plain"), ("Location", path)]),
            body: vec![],
        }
    }

    /// Respuesta OK
    pub fn ok(body: &str) -> Self {
        Self {
            status: StatusCode::OK,
            headers: Headers::from(&vec![("Content-Type", "text/plain")]),
            body: body.bytes().collect(),
        }
    }

    /// Respuesta 404 Not Found
    pub fn not_found() -> Self {
        Self {
            status: StatusCode::NOTFOUND,
            headers: Headers::from(&vec![("Content-Type", "text/plain")]),
            body: "404\n".bytes().collect(),
        }
    }

    /// Respuesta de error interno del servidor
    pub fn internal_err(body: &str) -> Self {
        Response {
            status: StatusCode::INTERNALERR,
            headers: Headers::from(&vec![("Content-Type", "text/plain")]),
            body: body.bytes().collect(),
        }
    }

    /// Respuesta con el contenido de un archivo
    pub fn file(path: &str) -> Self {
        let path_buf = PathBuf::from(path);
        let mut f = match File::open(path) {
            Ok(s) => s,
            Err(_) => return Self::not_found(),
        };
        let mut body = Vec::<u8>::new();
        if f.read_to_end(&mut body).is_err() {
            return Self::internal_err("Couldn't read file");
        }
        Self {
            status: StatusCode::OK,
            headers: Headers::from(&vec![("Content-Type", mime_type(&path_buf))]),
            body,
        }
    }
}

impl WriteTo for Response {
    type Error = &'static str;

    /// Escribe la respuesta HTTP en el stream proporcionado
    fn write_to<W: Write>(&self, stream: &mut W) -> Result<(), Self::Error> {
        // Escribe la línea de estado
        stream
            .write_fmt(format_args!("HTTP/1.1 {}\n", self.status as u32))
            .or(Err("Failed sending status code"))?;

        // Escribe los headers
        for (key, value) in self.headers.iter() {
            stream
                .write_fmt(format_args!("{}: {}\n", key, value))
                .or(Err("Failed sending headers data"))?;
        }

        // Escribe el separador entre headers y body
        stream
            .write_fmt(format_args!("\n"))
            .or(Err("Failed sending body separator"))?;

        // Escribe el body
        stream
            .write_all(&self.body)
            .or(Err("Failed sending payload"))?;

        Ok(())
    }
}