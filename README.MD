# Servidor HTTP en RUST
Este proyecto implementa un servidor HTTP básico en Rust. A continuación se describe el proceso general y las estructuras involucradas. Se utilizaron como guía los siguientes recursos:
- [Rust Fundamentals](https://www.udemy.com/course/rust-fundamentals/)
- [The Rust Programming Language](https://doc.rust-lang.org/beta/book/ch20-02-multithreaded.html)
- [http-server-from-scratch](https://github.com/giuliano-macedo/http-server-from-scratch-rs/tree/master)

## Estructura del Proyecto
El proyecto esté organizado por módulos, utilizando las siguientes estructuras y traits:
- **main.rs:** Archivo principal que inicializa y ejecuta el servidor.
- **headers.rs:** Define la estructura y funciones para manejar encabezados HTTP.
- **method.rs:** Enumera los métodos HTTP soportados.
- **mime_type.rs:** Determina el tipo MIME (media types) basado en la extensión de archivo.
- **path.rs:** Maneja la ruta de la solicitud HTTP.
- **request.rs:** Representa una solicitud HTTP y proporciona métodos para leerla desde un stream.
- **response.rs:** Representa una respuesta HTTP y proporciona métodos para escribirla en un stream.
- **router.rs:** Maneja el enrutamiento de las solicitudes a las funciones correspondientes.
- **serve.rs:** Maneja la conexión y el procesamiento de solicitudes y respuestas.
- **server.rs:** Define el servidor HTTP y maneja la inicialización y ejecución del mismo.
- **status_code.rs:** Define los códigos de estado HTTP.
- **traits.rs:** Define los traits ReadFrom y WriteTo utilizados para leer y escribir datos.
## ¿Cómo funciona el servidor?
### Inicialización del servidor
Primeramente se obtienen los argumentos de la línea de comandos para el puerto y la cantidad de hilos. El programa se debe correr mediante el uso del siguiente comando:
```bash
cargo r <port> <thread_qty>
```
A partir de estos parámetros, se crea una instancia de Server en la dirección 127.0.0.1 y el puerto proporcionado.

En el archivo *main.rs* se configuran las rutas para manejar las solicitudes:
- /index.html sirve un archivo estático.
- / redirige a /index.html.
- /login maneja las solicitudes de inicio de sesión (simula lo que sería un login)
Finalmente se inicia el servidor con el número de hilos especificado en los parámetros
### Aceptación de Conexiones
El servidor acepta conexiones entrantes utilizando TCP. Cada conexión se maneja en un hilo separado para permitir la concurrencia. Las conexiones son manejadas en el archivo *serve.rs*, como se puede apreciar en este código:
```rust
let ans = serve(&thread_name, &router_shared, stream);
```
La conexión TCP entrante se envuelve en un BufReader para leer los datos de manera eficiente. Luego, se lee y parsea la solicitud HTTP.
```rust
let req = Request::read_from(&mut reader).map_err(|e| ServeError::RequestRead(client_ip, e))?;
```
### Procesamiento de Solicitudes
El método **Request::read_from** lee la solicitud del stream. Esto incluye la línea de solicitud (método, ruta), los encabezados y el cuerpo.
```rust
pub fn read_from<R: BufRead>(stream: &mut R) -> Result<Self, Self::Error> { ... }
```
### Manejo de Rutas
```rust
let res = router.handle_request(&req);
```
Como se puede ver, la solicitud se pasa al Router, que determina cómo manejar la solicitud según la ruta. El Router usa el *handle_request* para encontrar y ejecutar el manejador apropiado, devolviendo un Response.
### Generación de Respuesta
La respuesta se construye en función de la ruta y el método. Existen varios tipos de respuesta, como redirección, éxito, no encontrado, o error interno.
```rust
pub fn redirect(path: &str) -> Self { ... }
pub fn ok(body: &str) -> Self { ... }
pub fn not_found() -> Self { ... }
pub fn internal_err(body: &str) -> Self { ... }
pub fn file(path: &str) -> Self { ... }
```
### Escritura de Respuesta
La respuesta se escribe en el stream utilizando el trait *WriteTo* desde el archivo *serve.rs*. Esto incluye la línea de estado, los encabezados y el cuerpo de la respuesta.
```rust
fn write_to<W: Write>(&self, stream: &mut W) -> Result<(), Self::Error> { ... }
```
### Registro
Se imprime un registro en la consola con detalles sobre la solicitud y el tiempo que tardó en procesarse.
```rust
println!("#{} [{}] {{{}}} {:?} '{}' -> {} {:.2}ms", ... );
```