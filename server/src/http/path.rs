use super::utils::parse_url_param;
use std::collections::HashMap;
use std::convert::From;
use std::fmt;

// Estructura que representa un Path (ruta principal y parámetros)
pub struct Path {
    data: String,
    params: Option<String>,
}

impl Path {
    // Convertir parámetros en un HashMap
    pub fn parse_params(&self) -> Result<HashMap<&str, &str>, &'static str> {
        // Si hay parámetros, los analizamos, sino devolvemos un HashMap vacío
        self.params.as_deref()
            .map_or(Ok(HashMap::new()), parse_url_param)
    }
}

// Construir un Path desde un string
impl From<&str> for Path {
    fn from(s: &str) -> Self {
        // Dividimos la ruta en la parte principal y los parámetros, si existen
        let (data, params) = s.split_once('?')
            .map_or((s.to_string(), None), |(d, p)| (d.to_string(), Some(p.to_string())));
        
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
