use std::collections::HashMap;
use std::convert::{From, TryFrom};
use std::fmt;
use std::iter::Iterator;
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
                break; // Terminamos si la línea está vacía
            }

            // Usamos el nuevo método `parse_header_line` para obtener clave y valor
            match Headers::parse_header_line(line) {
                Ok((key, value)) => headers.insert(key, value),
                Err(e) => return Err(e), // Retornamos error si no se pudo parsear
            }
        }

        Ok(headers) // Retornamos la instancia de `Headers` creada
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

        headers // Retornamos la instancia de `Headers` creada
    }
}

// Print de la estructura `Headers`
impl fmt::Debug for Headers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.data.iter()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    // Prueba  user_agent ()
    fn test_user_agent() {
        let mut headers_data = HashMap::new();
        headers_data.insert("User-Agent".to_string(), "Prueba".to_string());
        let headers = Headers { data: headers_data };

        assert_eq!(headers.user_agent(), Some(&"Prueba".to_string()));
    }

    #[test]
    // Prueba get () . Se espera que devuelva el valor de la clave dada
    //donde el valor de Content-Type debe ser igual a application/json
    // y el valor de Nonexistent no debe existir

    fn test_get() {
        let mut headers = Headers {
            data: HeadersDataType::new(),
        };
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        assert_eq!(
            headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
        assert_eq!(headers.get("No_existe"), None);
    }

    #[test]
    // Prueba  iter ()
    fn test_iter() {
        // Prueba el método iter para iterar sobre los encabezados
        let mut headers = Headers {
            data: HeadersDataType::new(),
        };
        headers.insert("Key1".to_string(), "Value1".to_string());
        headers.insert("Key2".to_string(), "Value2".to_string());
        let mut iter = headers.iter();
        assert!(iter.any(|(k, v)| k == "Key1" && v == "Value1"));
        assert!(iter.any(|(k, v)| k == "Key2" && v == "Value2"));
    }

    #[test]
    // Prueba  insert ()
    fn test_insert() {
        let mut headers = Headers {
            data: HeadersDataType::new(),
        };
        headers.insert("key".to_string(), "val".to_string());
        assert_eq!(headers.get("key"), Some(&"val".to_string()));
    }
    #[test]
    // Prueba parse_header_line con 1 entrada válidas e 2 inválidas
    fn test_parse_header_line() {
        assert_eq!(
            Headers::parse_header_line("Content-Type: application/json"),
            Ok(("Content-Type".to_string(), "application/json".to_string()))
        );
        assert_eq!(
            Headers::parse_header_line("No_existe"),
            Err("Invalid header, no ': ' found")
        );
        assert_eq!(
            Headers::parse_header_line(": No Key"),
            Err("Invalid header")
        );
    }

    #[test]
    // Prueba try_from_split para crear Headers desde un Split
    fn test_try_from_split() {
        let mut split = "Host: example.com\nUser-Agent: app/5.0\n\n".split('\n');
        let headers = Headers::try_from(&mut split).unwrap();
        assert_eq!(headers.get("Host"), Some(&"example.com".to_string()));
        assert_eq!(headers.get("User-Agent"), Some(&"app/5.0".to_string()));
    }

    #[test]
    // Prueba el caso de error de TryFrom
    fn test_try_from_split_error() {
        let mut split = "Invalid\n".split('\n');
        assert!(Headers::try_from(&mut split).is_err());
    }

    #[test]
    // Prueba From para crear Headers desde un vector de tuplas
    fn test_from_vec() {
        let vec = vec![
            ("Content-Type", "application/json"),
            ("Authorization", "Bearer token"),
        ];
        let headers = Headers::from(&vec);
        assert_eq!(
            headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
        assert_eq!(
            headers.get("Authorization"),
            Some(&"Bearer token".to_string())
        );
    }
}
