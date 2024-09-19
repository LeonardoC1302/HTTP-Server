use std::collections::HashMap;
use std::convert::{From, TryFrom};
use std::iter::Iterator;
use std::fmt;
use std::str::Split;

// Tipo de dato para simplificar la declaración del HashMap
type HeadersDataType = HashMap<String, String>;

// Definimos la estructura principal
pub struct Headers {
    data: HeadersDataType,
}

impl Headers {
    // Método para obtener el valor del encabezado "User-Agent"
    pub fn user_agent(&self) -> Option<&String> {
        self.get("User-Agent")
    }

    // Método genérico para obtener el valor de un encabezado por su clave
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    // Método para obtener un iterador sobre las entradas (clave, valor) del HashMap
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> + '_ {
        self.data.iter()
    }

    // Método para agregar un nuevo encabezado
    pub fn insert(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    // Método para convertir una línea en clave-valor
    fn parse_header_line(line: &str) -> Result<(String, String), &'static str> {
        let mut line_splitted = line.splitn(2, ':');
        let key = line_splitted
            .next()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .ok_or("Invalid header")?;
        let value = line_splitted
            .next()
            .map(|s| s.trim().to_string())
            .ok_or("Invalid header, no ': ' found")?;
        Ok((key, value))
    }
}

// Implementamos `TryFrom` para convertir desde un iterador de `Split` a `Headers`
impl TryFrom<&mut Split<'_, char>> for Headers {
    type Error = &'static str;

    fn try_from(str_iter: &mut Split<'_, char>) -> Result<Self, Self::Error> {
        let mut headers = Headers {
            data: HeadersDataType::new(),
        };

        // Iteramos por cada línea y la convertimos en clave-valor
        for line in str_iter {
            if line.trim().is_empty() {
                break;  // Terminamos si la línea está vacía
            }

            // Usamos el nuevo método `parse_header_line` para obtener clave y valor
            match Headers::parse_header_line(line) {
                Ok((key, value)) => headers.insert(key, value),
                Err(e) => return Err(e),  // Retornamos error si no se pudo parsear
            }
        }

        Ok(headers)  // Retornamos la instancia de `Headers` creada
    }
}

// Construir `Headers` desde un vector de pares (&str, &str)
impl From<&Vec<(&str, &str)>> for Headers {
    fn from(vec: &Vec<(&str, &str)>) -> Self {
        let mut headers = Headers {
            data: HeadersDataType::new(),
        };

        // Iteramos sobre el vector y agregamos cada par (clave-valor) al HashMap
        for &(key, value) in vec.iter() {
            headers.insert(key.to_string(), value.to_string());
        }

        headers  // Retornamos la instancia de `Headers` creada
    }
}

// Print de la estructura `Headers`
impl fmt::Debug for Headers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.data.iter()).finish()
    }
}
