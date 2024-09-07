use std::path::Path;

const UNKNOW_BINARY_MIME: &'_ str = "application/octet-stream";

pub fn mime_type(path: &Path) -> &str {
    let ext = match path.extension() {
        Some(e) => e,
        None => return UNKNOW_BINARY_MIME,
    };
    match ext.to_str().unwrap_or("") {
        "txt" => "text/plain",
        "html" => "text/html",
        "css" => "text/css",
        "js" => "text/javascript",
        "png" => "image/png",
        "svg" => "image/svg+xml",
        "jpeg" | "jpg" | "jfif" | "pjpeg" | "pjp" => "image/jpeg",
        "webp" => "image/webp",
        _ => UNKNOW_BINARY_MIME,
    }
}