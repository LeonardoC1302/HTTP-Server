use std::convert::From;

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl From<&str> for Method {
    fn from(input: &str) -> Self {
        match input {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "PATCH" => Method::PATCH,
            unrecognized => {
                println!(
                    "Warning: unrecognized method in request '{}', using GET",
                    unrecognized
                );
                Method::GET
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_methods() {
        // Prueba la conversión de cadenas válidas a métodos HTTP
        assert_eq!(Method::from("GET"), Method::GET);
        assert_eq!(Method::from("POST"), Method::POST);
        assert_eq!(Method::from("PUT"), Method::PUT);
        assert_eq!(Method::from("DELETE"), Method::DELETE);
        assert_eq!(Method::from("PATCH"), Method::PATCH);
    }

    #[test]
    fn test_unrecognized_method() {
        // Prueba el comportamiento con métodos no reconocidos
        assert_eq!(Method::from("UNKNOWN"), Method::GET);
        assert_eq!(Method::from("HEAD"), Method::GET);
        assert_eq!(Method::from("OPTIONS"), Method::GET);
        assert_eq!(Method::from(""), Method::GET);
    }
}
