#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum StatusCode {
    OK = 200,
    REDIRECT = 301,
    UNAVAILABLE = 503,
    INTERNALERR = 500,
    NOTFOUND = 404,
}