use std::convert::TryFrom;
use std::io::BufRead;
use super::{Headers, Method, Path, ReadFrom};

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
        let l = stream.read(&mut buff).or(Err("Could not receive request"))?;
        
        // Convierte el buffer a string
        let buff_str = String::from_utf8_lossy(&buff[0..l]);
        let mut buff_str_splitted = buff_str.split('\n');

        // Analiza la primera línea (línea de solicitud)
        let first_line = buff_str_splitted.next()
            .ok_or("There is no first line in the request")?;

        let mut first_line_splitted = first_line.split(' ');

        // Extrae el método y la ruta de la primera línea
        let method = Method::from(first_line_splitted.next()
            .ok_or("First line has no spaces")?);
        let path = Path::from(first_line_splitted.next()
            .ok_or("First line doesn't have a path")?);

        // Analiza los encabezados
        let headers = Headers::try_from(&mut buff_str_splitted)
            .or(Err("Failed parsing headers"))?;

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