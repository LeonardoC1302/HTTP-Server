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
    server.on(r"/login", |req| {
        // Solo aceptamos solicitudes GET y POST
        if req.method != Method::GET && req.method != Method::POST {
            return Response::internal_err("Only get or post requests are allowed");
        }
    
        // Parsear los parámetros de la URL (para GET)
        let mut query = match req.path.parse_params() {
            Ok(q) => q,
            Err(_) => return Response::internal_err("Couldn't parse query parameters"),
        };
    
        // Parsear los parámetros del cuerpo de la solicitud (para POST)
        let body = match parse_url_param(&req.body) {
            Ok(b) => b,
            Err(_) => return Response::internal_err("Couldn't parse body parameters"),
        };
    
        // Mezclar los parámetros de la URL y del cuerpo
        query.extend(body);
    
        // Obtener el valor del campo 'email'
        let email = match query.get("email") {
            Some(v) => v,
            None => return Response::internal_err("Missing email"),
        };
    
        // Obtener el valor del campo 'password'
        let password = match query.get("password") {
            Some(v) => v,
            None => return Response::internal_err("Missing password"),
        };
    
        // Crear una respuesta con los valores de email y password
        let response = format!("Email: {}, Password: {}\n", email, password);
        Response::ok(&response)
    });    


    server.run(thread_qty)
}