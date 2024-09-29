use super::{Headers, Request, Response, StatusCode};
use std::collections::HashMap;

/// Tipo que representa una función de callback para manejar requests
pub type Callback = fn(&Request) -> Response;

/// Estructura que maneja el enrutamiento de requests
#[derive(Clone)]
pub struct Router {
    handlers: HashMap<String, Handler>,
}

/// Enum que representa los tipos de manejadores de rutas
#[derive(Clone)]
enum Handler {
    Callback(Callback),
    File(String),
}

impl Router {
    /// Crea un nuevo Router
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Maneja una request y devuelve la Response apropiada
    pub fn handle_request(&self, req: &Request) -> Response {
        // Busca un manejador que coincida con la ruta de la request
        let handler = match self.handlers.iter().find(|(k, _)| req.path == **k) {
            Some((_, v)) => v,
            None => return Response::not_found(),
        };

        // Ejecuta el manejador correspondiente
        match handler {
            Handler::Callback(cb) => cb(req),
            Handler::File(fname) => Response::file(fname),
        }
    }

    /// Inserta un nuevo manejador de tipo Callback
    pub fn insert_callback(&mut self, pat: &str, cb: Callback) {
        self.handlers.insert(pat.to_string(), Handler::Callback(cb));
    }

    /// Inserta un nuevo manejador de tipo File
    pub fn insert_file(&mut self, pat: &str, fname: &str) {
        self.handlers
            .insert(pat.to_string(), Handler::File(fname.to_string()));
    }
}

#[cfg(test)]
mod tests {

    use super::{Headers, Request, Response, Router, StatusCode};

    #[test]
    // Prueba de router con archivo
    fn test_router_with_callback() {
        let mut router = Router::new();

        fn simple_callback(_req: &Request) -> Response {
            Response::ok("Callback response")
        }

        router.insert_callback("/test", simple_callback);

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

        let response = router.handle_request(&request);

        assert_eq!(String::from_utf8_lossy(&response.body), "Callback response");
        assert_eq!(response.status, StatusCode::OK);
    }

    #[test]
    // Prueba de router con archivo
    fn test_router_not_found() {
        let router = Router::new();

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

        let response = router.handle_request(&request);

        assert_eq!(response.status, StatusCode::NOTFOUND);
    }
}
