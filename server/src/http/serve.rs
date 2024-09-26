use super::{ReadFrom, Request, Router, WriteTo};
use std::convert::From;
use std::fmt;
use std::io::BufReader;
use std::net::{SocketAddr, TcpStream};
use std::time::Instant;
use std::collections::HashMap;

/// Tipo que representa el resultado de aceptar una conexión TCP
pub type StreamType = Result<(TcpStream, SocketAddr), std::io::Error>;

/// Enum que representa los posibles errores durante el servicio de una request
pub enum ServeError {
    StartConnection,
    RequestRead(SocketAddr, &'static str),
    ResponseRead(SocketAddr, &'static str),
}

impl fmt::Display for ServeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServeError::StartConnection => write!(f, "couldn't start client connection"),
            ServeError::RequestRead(ip, err) => {
                write!(f, "couldn't read request from {} because '{}'", ip, err)
            }
            ServeError::ResponseRead(ip, err) => {
                write!(f, "couldn't write response to {} because '{}'", ip, err)
            }
        }
    }
}

/// Maneja una conexión entrante, procesa la request y envía la response
pub fn serve(thread_name: &str, router: &Router, stream: StreamType) -> Result<(), ServeError> {
    let start = Instant::now();

    // Obtiene el stream y la dirección IP del cliente
    let (mut client, client_ip) = stream.or(Err(ServeError::StartConnection))?;

    // Crea un BufReader para leer eficientemente del stream
    let mut reader = BufReader::with_capacity(4000, &mut client);

    // Lee y parsea la request
    let req = Request::read_from(&mut reader)
        .map_err(|e| ServeError::RequestRead(client_ip, e))?;

    // Maneja la request y obtiene la response
    let mut res = router.handle_request(&req);

    // Verificar si en los headers del request hay cookies
    let cookies = req.headers.get("Cookie").map_or_else(|| "".to_string(), |v| v.clone());

    // Parsear las cookies
    let mut cookie_map = HashMap::new();
    for cookie in cookies.split(';') {
        let mut parts = cookie.split('=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            cookie_map.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    // If theres cookies, set them
    if !cookie_map.is_empty() {
        res.set_cookie(cookie_map);
    }


    // Escribe la response al cliente
    res.write_to(&mut client)
        .map_err(|e| ServeError::ResponseRead(client_ip, e))?;

    let duration = start.elapsed();

    // Imprime información de registro sobre la request procesada
    println!(
        "#{} [{}] {{{}}} {:?} '{}' -> {} {:.2}ms",
        thread_name,
        client_ip,
        req.headers.user_agent().unwrap_or(&String::from("None")),
        req.method,
        req.path,
        res.status as usize,
        duration.as_nanos() as f64 / 1e+6
    );
    Ok(())
}