use std::collections::HashMap;
pub fn parse_url_param(input: &str) -> Result<HashMap<&str, &str>, &'static str> {
    let mut ans: HashMap<&str, &str> = HashMap::new();
    for keyvalue in input.split_terminator('&') {
        let (key, value) = match keyvalue.split_once('=') {
            Some((k, v)) => (k, v),
            None => return Err("Invalid URL param"),
        };
        ans.insert(key, value);
    }
    Ok(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // prueba de parámetros de URL válidos
    fn test_parse_url_param_valid() {
        let input = "key1=value1&key2=value2&key3=value3";
        let result = parse_url_param(input).unwrap();

        assert_eq!(result.get("key1"), Some(&"value1"));
        assert_eq!(result.get("key2"), Some(&"value2"));
        assert_eq!(result.get("key3"), Some(&"value3"));
        assert_eq!(result.len(), 3);
    }

    #[test]
    // prueba de parámetros de URL con un valor vacío
    fn test_parse_url_param_missing_value() {
        let input = "key1=&key2=value2";
        let result = parse_url_param(input).unwrap();

        assert_eq!(result.get("key1"), Some(&""));
        assert_eq!(result.get("key2"), Some(&"value2"));
        assert_eq!(result.len(), 2);
    }

    #[test]
    // prueba de parámetros de URL con una clave vacía
    fn test_parse_url_param_invalid() {
        let input = "key1=value1&key2"; // Missing value for key2
        let result = parse_url_param(input);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid URL param");
    }
}
