use std::convert::From; 

#[derive(Debug, PartialEq)] 
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH
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