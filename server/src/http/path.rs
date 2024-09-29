use super::utils::parse_url_param;
use std::collections::HashMap;
use std::convert::From;
use std::fmt;

// Estructura que representa un Path (ruta principal y parámetros)
#[derive(Debug)]

pub struct Path {
    data: String,
    params: Option<String>,
}

impl Path {
    // Convertir parámetros en un HashMap
    pub fn parse_params(&self) -> Result<HashMap<&str, &str>, &'static str> {
        // Si hay parámetros, los analizamos, sino devolvemos un HashMap vacío
        self.params
            .as_deref()
            .map_or(Ok(HashMap::new()), parse_url_param)
    }
}

// Construir un Path desde un string
impl From<&str> for Path {
    fn from(s: &str) -> Self {
        // Dividimos la ruta en la parte principal y los parámetros, si existen
        let (data, params) = s.split_once('?').map_or((s.to_string(), None), |(d, p)| {
            (d.to_string(), Some(p.to_string()))
        });

        Path { data, params }
    }
}

// Comparar Path con un string
impl std::cmp::PartialEq<String> for Path {
    fn eq(&self, other: &String) -> bool {
        self.data == *other
    }
}

// Print de Path
impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Si hay parámetros, los incluimos en la salida, sino solo la ruta principal
        match &self.params {
            Some(p) => write!(f, "{}?{}", &self.data, p),
            None => write!(f, "{}", &self.data),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // prueba que se pueda construir un Path desde un string
    fn test_from_str_to_path() {
        let path = Path::from("/home?user=admin&theme=dark");

        assert_eq!(path.data, "/home");
        assert_eq!(path.params, Some("user=admin&theme=dark".to_string()));
    }

    #[test]
    //prueba que se pueda construir un Path sin parámetros
    fn test_from_str_no_params() {
        let path = Path::from("/about");

        assert_eq!(path.data, "/about");
        assert_eq!(path.params, None);
    }
    //prueba que se pueda convertir un Path en un HashMap
    #[test]
    fn test_parse_params() {
        let path = Path::from("/home?user=admin&theme=dark");

        let parsed_params = path.parse_params().unwrap();
        assert_eq!(parsed_params.get("user"), Some(&"admin"));
        assert_eq!(parsed_params.get("theme"), Some(&"dark"));
    }

    //prueba que se pueda convertir un Path sin parámetros en un HashMap vacío
    #[test]
    fn test_parse_params_empty() {
        let path = Path::from("/home");

        let parsed_params = path.parse_params().unwrap();
        assert!(parsed_params.is_empty());
    }

    //prueba que se pueda imprimir un Path con parámetros
    #[test]
    fn test_partial_eq() {
        let path = Path::from("/about");
        let other = "/about".to_string();

        // Use assert! instead of assert_eq! to avoid the need for Debug
        assert!(path == other, "Expected path to be equal to {}", other);
    }
}
