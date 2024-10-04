// Dependencias
mod http;
use http::{parse_url_param, Method, Response, Server};
use std::env;

fn main() {
    // Obtenemos los arguentos de la línea de comandos
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("[USAGE] {} <PORT> <THREAD_QTY>", args[0]);
    }
    let thread_qty: usize = args[2].parse().unwrap();

    let mut server = Server::new("127.0.0.1", &args[1]);
    server.on_file(r"/index.html", "./static/index.html");
    server.on(r"/", |_| Response::redirect("/index.html"));

    // Simulamos una página de login
    server.on_file(r"/login", "./static/login.html");

    // Simulamos el API de login
    server.on(r"/api/login", |req| {
        // Solo aceptamos solicitudes POST
        if req.method != Method::POST {
            return Response::internal_err("Only post requests are allowed");
        }
    
        // Parsear los parámetros del cuerpo de la solicitud
        let body = match parse_url_param(&req.body) {
            Ok(b) => b,
            Err(_) => return Response::internal_err("Couldn't parse body parameters"),
        };
    
        // Obtener el valor del campo 'email'
        let email = match body.get("email") {
            Some(v) => v,
            None => return Response::internal_err("Missing email"),
        };

        // Obtener el valor del campo 'password'
        let password = match body.get("password") {
            Some(v) => v,
            None => return Response::internal_err("Missing password"),
        };

        // Crear una respuesta con los valores de email y password
        let response = format!("Email: {}, Password: {}\n", email, password);
        Response::ok(&response)
    });
    
    // Simulamos una API de pruebas
    server.on(r"/api/tests", |req| {
        // Parsear los parámetros de la URL
        let mut query = match req.path.parse_params() {
            Ok(q) => q,
            Err(_) => return Response::internal_err("Couldn't parse query parameters"),
        };

        // Parsear los parámetros del cuerpo de la solicitud
        let body = match parse_url_param(&req.body) {
            Ok(b) => b,
            Err(_) => return Response::internal_err("Couldn't parse body parameters"),
        };

        // Mezclar los parámetros de la URL y del cuerpo
        query.extend(body);

        // Crear una respuesta con el método y los parámetros
        let mut response = format!("Method: {:?}\n", req.method);
        for (key, value) in query {
            response.push_str(&format!("{}: {}\n", key, value));
        }

        // Devolver la respuesta
        Response::ok(&response)
    });

    server.run(thread_qty)
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::{Headers, Method, Request, StatusCode};

    #[test]
    fn test_login_handler() {
        let mut server = Server::new("127.0.0.1", "8080");
        server.on_file(r"/index.html", "./static/index.html");
        server.on(r"/", |_| Response::redirect("/index.html"));

        // Define the login handler
        let login_handler = |req: &Request| {
            // Only accept GET and POST requests
            if req.method != Method::GET && req.method != Method::POST {
                return Response::internal_err("Only get or post requests are allowed");
            }

            // Parse URL parameters (for GET)
            let mut query = match req.path.parse_params() {
                Ok(q) => q,
                Err(_) => return Response::internal_err("Couldn't parse query parameters"),
            };

            // Parse body parameters (for POST)
            let body = match parse_url_param(&req.body) {
                Ok(b) => b,
                Err(_) => return Response::internal_err("Couldn't parse body parameters"),
            };

            // Merge URL and body parameters
            query.extend(body);

            // Get email
            let email = match query.get("email") {
                Some(v) => v,
                None => return Response::internal_err("Missing email"),
            };

            // Get password
            let password = match query.get("password") {
                Some(v) => v,
                None => return Response::internal_err("Missing password"),
            };

            // Create response
            let response = format!("Email: {}, Password: {}\n", email, password);
            Response::ok(&response)
        };

        // Create headers
        let vec = vec![
            ("Content-Type", "application/json"),
            ("Authorization", "Bearer token"),
        ];
        let headers = Headers::from(&vec);

        // Test GET request
        let req = Request {
            method: Method::GET,
            path: "/login?email=test@example.com&password=123456".into(),
            headers: headers,
            body: String::new(),
        };
        let response = login_handler(&req);

        assert_eq!(response.status, StatusCode::OK);
        assert_eq!(
            String::from_utf8(response.body).unwrap(),
            "Email: test@example.com, Password: 123456\n"
        );

        // Create headers
        let vec = vec![
            ("Content-Type", "application/json"),
            ("Authorization", "Bearer token"),
        ];
        let headers = Headers::from(&vec);
        // Test POST request
        let req = Request {
            method: Method::POST,
            path: "/login".into(),
            headers: headers,
            body: "email=post@example.com&password=654321".to_string(),
        };
        let response = login_handler(&req);

        assert_eq!(response.status, StatusCode::OK);
        assert_eq!(
            String::from_utf8(response.body).unwrap(),
            "Email: post@example.com, Password: 654321\n"
        );

        // Create headers
        let vec = vec![
            ("Content-Type", "application/json"),
            ("Authorization", "Bearer token"),
        ];
        let headers = Headers::from(&vec);
        // Test invalid method
        let req = Request {
            method: Method::PUT,
            path: "/login".into(),
            headers,
            body: String::new(),
        };
        let response = login_handler(&req);

        assert_eq!(response.status, StatusCode::INTERNALERR);
        assert_eq!(
            String::from_utf8(response.body).unwrap(),
            "Only get or post requests are allowed"
        );
    }

}
