use super::{Headers, Method, Path, ReadFrom};
use std::convert::TryFrom;
use std::io::BufRead;

/// Solicitud HTTP
pub struct Request {
    pub method: Method,
    pub path: Path,
    pub headers: Headers,
    pub body: String,
}

impl ReadFrom for Request {
    type Error = &'static str;

    fn read_from<R: BufRead>(stream: &mut R) -> Result<Self, Self::Error> {
        // Buffer para almacenar la solicitud entrante
        let mut buff: [u8; 4000] = [0; 4000];

        // Lee la solicitud en el buffer
        let l = stream
            .read(&mut buff)
            .or(Err("Could not receive request"))?;

        // Convierte el buffer a string
        let buff_str = String::from_utf8_lossy(&buff[0..l]);
        let mut buff_str_splitted = buff_str.split('\n');

        // Analiza la primera línea (línea de solicitud)
        let first_line = buff_str_splitted
            .next()
            .ok_or("There is no first line in the request")?;

        let mut first_line_splitted = first_line.split(' ');

        // Extrae el método y la ruta de la primera línea
        let method = Method::from(
            first_line_splitted
                .next()
                .ok_or("First line has no spaces")?,
        );
        let path = Path::from(
            first_line_splitted
                .next()
                .ok_or("First line doesn't have a path")?,
        );

        // Analiza los encabezados
        let headers =
            Headers::try_from(&mut buff_str_splitted).or(Err("Failed parsing headers"))?;

        // Recolecta las líneas restantes como cuerpo
        let body = buff_str_splitted.collect::<Vec<&str>>().join("\n");

        Ok(Self {
            method,
            path,
            headers,
            body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    // Prueba la lectura de una solicitud HTTP
    fn test_request_read_from() {
        let request_str = "\
            GET /index.html HTTP/1.1\r\n\
            Host: www.example.com\r\n\
            User-Agent: rust-test\r\n\
            \r\n\
            Este es el cuerpo de la solicitud.";

        let mut cursor = Cursor::new(request_str);

        let result = Request::read_from(&mut cursor);

        assert!(result.is_ok(), "La lectura falló");

        let request = result.unwrap();

        assert_eq!(request.method, Method::GET, "El método falló");
        assert_eq!(request.path.to_string(), "/index.html", "La ruta falló");
        assert_eq!(
            request.headers.get("Host"),
            Some(&"www.example.com".to_string()),
            "El encabezado Host falló"
        );
        assert_eq!(
            request.headers.get("User-Agent"),
            Some(&"rust-test".to_string()),
            "El encabezado User-Agent falló"
        );
        assert_eq!(
            request.body, "Este es el cuerpo de la solicitud.",
            "El cuerpo de la solicitud falló"
        );
    }
}
