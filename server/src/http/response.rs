use super::{mime_type, Headers, StatusCode, WriteTo};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

/// Respuesta HTTP
#[derive(Debug)]

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
            return Self::internal_err("Could not read file");
        }
        Self {
            status: StatusCode::OK,
            headers: Headers::from(&vec![("Content-Type", mime_type(&path_buf))]),
            body,
        }
    }

    pub fn set_cookie(&mut self, cookies: HashMap<String, String>) {
        let mut cookie_string = String::new();
        for (key, value) in cookies {
            if !cookie_string.is_empty() {
                cookie_string.push_str("; "); // Separate multiple cookies with a semicolon
            }
            cookie_string.push_str(&format!("{}={}", key, value));
        }

        // Store the concatenated cookie string in the Set-Cookie header
        self.headers.insert("Set-Cookie".to_string(), cookie_string);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    // prueba de respuesta de redirección
    fn test_redirect_response() {
        let response = Response::redirect("/new-path");
        assert_eq!(response.status, StatusCode::REDIRECT);
        assert_eq!(
            response.headers.get("Location"),
            Some(&"/new-path".to_string())
        );
        assert!(response.body.is_empty());
    }

    #[test]
    // prueba de respuesta OK
    fn test_ok_response() {
        let response = Response::ok("Hello, world!");
        assert_eq!(response.status, StatusCode::OK);
        assert_eq!(
            response.headers.get("Content-Type"),
            Some(&"text/plain".to_string())
        );
        assert_eq!(response.body, b"Hello, world!");
    }

    #[test]
    // prueba de respuesta 404 Not Found
    fn test_not_found_response() {
        let response = Response::not_found();
        assert_eq!(response.status, StatusCode::NOTFOUND);
        assert_eq!(response.body, b"404\n");
    }
    // prueba de respuesta de error interno
    #[test]
    fn test_internal_err_response() {
        let response = Response::internal_err("Server error");
        assert_eq!(response.status, StatusCode::INTERNALERR);
        assert_eq!(response.body, b"Server error");
    }

    // prueba de respuesta con el contenido de un archivo
    #[test]
    fn test_set_cookie() {
        let mut response = Response::ok("Test");
        let mut cookies = HashMap::new();
        cookies.insert("session".to_string(), "abc123".to_string());
        cookies.insert("user".to_string(), "john".to_string());
        response.set_cookie(cookies);
        assert!(response
            .headers
            .get("Set-Cookie")
            .unwrap()
            .contains("session=abc123"));
        assert!(response
            .headers
            .get("Set-Cookie")
            .unwrap()
            .contains("user=john"));
    }

    #[test]
    // prueba de escritura en un stream
    fn test_write_to() {
        let response = Response::ok("Test body");
        let mut buffer = Cursor::new(Vec::new());
        response.write_to(&mut buffer).unwrap();

        let written = String::from_utf8(buffer.into_inner()).unwrap();
        assert!(written.starts_with("HTTP/1.1 200\n"));
        assert!(written.contains("Content-Type: text/plain\n"));
        assert!(written.ends_with("\n\nTest body"));
    }
}
