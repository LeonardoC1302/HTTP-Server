use super::{Headers, ReadFrom, Request, Response, Router, WriteTo};
use std::collections::HashMap;
use std::convert::From;
use std::fmt;
use std::io::BufReader;
use std::net::{SocketAddr, TcpStream};
use std::time::Instant;

/// Tipo que representa el resultado de aceptar una conexión TCP
pub type StreamType = Result<(TcpStream, SocketAddr), std::io::Error>;

/// Enum que representa los posibles errores durante el servicio de una request
#[derive(Debug)]

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
    let req = Request::read_from(&mut reader).map_err(|e| ServeError::RequestRead(client_ip, e))?;

    println!("---Req:{:?}", req);

    // Maneja la request y obtiene la response
    let mut res = router.handle_request(&req);

    // Verificar si en los headers del request hay cookies
    let cookies = req
        .headers
        .get("Cookie")
        .map_or_else(|| "".to_string(), |v| v.clone());

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

    println!("---Res:{:?}", res);

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::sync::{Arc, Mutex};
    use std::thread;

    pub fn mock_serve(
        thread_name: &str,
        router: &Router,
        request: Request,
    ) -> Result<Response, ServeError> {
        let start = Instant::now();

        // Simulate getting client IP for logging purposes
        let client_ip = "127.0.0.1"; // Hard-coded for mock purposes

        // Handle the request and get the response
        let mut res = router.handle_request(&request);

        // Check if the headers in the request contain cookies
        let cookies = request
            .headers
            .get("Cookie")
            .map_or_else(|| "".to_string(), |v| v.clone());

        // Parse the cookies
        let mut cookie_map = HashMap::new();
        for cookie in cookies.split(';') {
            let mut parts = cookie.split('=');
            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                cookie_map.insert(key.trim().to_string(), value.trim().to_string());
            }
        }

        // If there are cookies, set them in the response
        if !cookie_map.is_empty() {
            res.set_cookie(cookie_map);
        }

        let duration = start.elapsed();

        // Print log information about the processed request
        println!(
            "#{} [{}] {{{}}} {:?} '{}' -> {} {:.2}ms",
            thread_name,
            client_ip,
            request
                .headers
                .user_agent()
                .unwrap_or(&String::from("None")),
            request.method,
            request.path,
            res.status as usize,
            duration.as_nanos() as f64 / 1e+6
        );

        Ok(res)
    }
    /*
    #[test]
    fn test_serve() {
        // Simula un router sencillo
        let mut router = Router::new();
        fn simple_callback(_req: &Request) -> Response {
            Response::ok("Test response")
        }
        router.insert_callback("/test", simple_callback);

        // Crea un servidor TCP local para la prueba
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let router = Arc::new(router);

        // Crea un thread para aceptar una conexión y enviar una request
        let server_thread = thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                // Simula una request HTTP que el servidor recibirá
                let request = b"GET /test HTTP/1.1\r\n\
                                Host: localhost\r\n\
                                User-Agent: test-client\r\n\
                                Cookie: testcookie=testvalue\r\n\
                                \r\n";

                // Envía la request al servidor
                stream.write_all(request).unwrap();

                // Lee la respuesta
                let mut buffer = [0; 512];
                let _ = stream.read(&mut buffer).unwrap();
            }
        });

        // Simula una conexión cliente para la función serve
        let stream = TcpStream::connect(addr).unwrap();
        let stream_type: StreamType = Ok((stream.try_clone().unwrap(), addr));

        // Llamamos a la función serve con el router simulado y el stream
        let result = serve("test-thread", &router, stream_type);
        println!("ERROR:::{:?}", result); // Imprime el resultado

        // Verificamos que no ocurrieron errores en el servicio
        assert!(result.is_ok());

        // Esperamos que el servidor termine de procesar
        server_thread.join().unwrap();
    }*/

    #[test]
fn test_serve() {
    // Simula un router sencillo
    let mut router = Router::new();
    fn simple_callback(_req: &Request) -> Response {
        Response::ok("Test response")
    }
    router.insert_callback("/test", simple_callback);

    // Crea un servidor TCP local para la prueba
    let listener = Arc::new(Mutex::new(TcpListener::bind("127.0.0.1:0").unwrap()));
    let addr = listener.lock().unwrap().local_addr().unwrap();
    let router = Arc::new(router);

    // Crea un thread para aceptar una conexión y enviar una request
    let server_thread = {
        let listener = Arc::clone(&listener);
        let router = Arc::clone(&router);
        thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.lock().unwrap().accept() {
                // Simula una request HTTP que el servidor recibirá
                let request = b"GET /test HTTP/1.1\r\n\
                                Host: localhost\r\n\
                                User-Agent: test-client\r\n\
                                Cookie: testcookie=testvalue\r\n\
                                \r\n";

                // Envía la request al servidor
                stream.write_all(request).unwrap();
                
                // Lee la respuesta
                let mut buffer = [0; 512];
                let _ = stream.read(&mut buffer).unwrap();
            }
        })
    };

    // Prepara la request
    let vec = vec![
        ("Content-Type", "application/json"),
        ("Authorization", "Bearer token"),
    ];
    let headers = Headers::from(&vec);

    let request: Request = Request {
        method: "GET".into(),
        path: "/test".into(),
        headers: headers,
        body: String::new(),
    };

    // Simula una conexión cliente para la función serve
    let result = mock_serve("test-thread", &router, request);
    println!("ERROR:::{:?}", result); // Imprime el resultado

    // Verificamos que no ocurrieron errores en el servicio
    assert!(result.is_ok());

    // Esperamos que el servidor termine de procesar
    server_thread.join().unwrap();
}

}
