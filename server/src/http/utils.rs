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