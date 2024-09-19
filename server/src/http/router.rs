use super::{Request, Response};
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