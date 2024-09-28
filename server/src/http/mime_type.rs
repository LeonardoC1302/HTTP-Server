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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_known_extensions() {
        // Prueba extensiones de archivo conocidas
        assert_eq!(mime_type(Path::new("file.txt")), "text/plain");
        assert_eq!(mime_type(Path::new("index.html")), "text/html");
        assert_eq!(mime_type(Path::new("styles.css")), "text/css");
        assert_eq!(mime_type(Path::new("script.js")), "text/javascript");
        assert_eq!(mime_type(Path::new("image.png")), "image/png");
        assert_eq!(mime_type(Path::new("vector.svg")), "image/svg+xml");
        assert_eq!(mime_type(Path::new("photo.jpg")), "image/jpeg");
        assert_eq!(mime_type(Path::new("picture.jpeg")), "image/jpeg");
        assert_eq!(mime_type(Path::new("graphic.webp")), "image/webp");
    }

    #[test]
    fn test_unknown_extension() {
        // Prueba extensiones desconocidas
        assert_eq!(mime_type(Path::new("file.unknown")), UNKNOW_BINARY_MIME);
        assert_eq!(mime_type(Path::new("document.doc")), UNKNOW_BINARY_MIME);
        assert_eq!(mime_type(Path::new("archive.zip")), UNKNOW_BINARY_MIME);
        assert_eq!(mime_type(Path::new("README")), UNKNOW_BINARY_MIME);
        assert_eq!(mime_type(Path::new("Makefile")), UNKNOW_BINARY_MIME);
    }
}
