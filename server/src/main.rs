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
    let thread_qty:usize = args[2].parse().unwrap();
    
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