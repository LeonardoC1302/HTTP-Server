# HTTP Server in RUST

This project implements a basic HTTP server in Rust. Below is a general overview of the process and structures involved. The following resources were used as guidance:
- [Rust Fundamentals](https://www.udemy.com/course/rust-fundamentals/)
- [The Rust Programming Language](https://doc.rust-lang.org/beta/book/ch20-02-multithreaded.html)
- [http-server-from-scratch](https://github.com/giuliano-macedo/http-server-from-scratch-rs/tree/master)

## Project Structure
The project is organized into modules, using the following structures and traits:
- **main.rs:** The main file that initializes and runs the server.
- **headers.rs:** Defines the structure and functions to handle HTTP headers.
- **method.rs:** Enumerates the supported HTTP methods.
- **mime_type.rs:** Determines the MIME type (media types) based on the file extension.
- **path.rs:** Manages the path of the HTTP request.
- **request.rs:** Represents an HTTP request and provides methods to read it from a stream.
- **response.rs:** Represents an HTTP response and provides methods to write it to a stream.
- **router.rs:** Handles routing of requests to the corresponding functions.
- **serve.rs:** Manages connections and processes requests and responses.
- **server.rs:** Defines the HTTP server and manages its initialization and execution.
- **status_code.rs:** Defines HTTP status codes.
- **traits.rs:** Defines the traits `ReadFrom` and `WriteTo` used for reading and writing data.

## How does the server work?

### Server Initialization
First, the command-line arguments for the port and the number of threads are retrieved. The program must be run with the following command:
```bash
cargo r <port> <thread_qty>
```
Based on these parameters, a `Server` instance is created at address `127.0.0.1` and the provided port. NOTE: this command should be executed from the *server* folder. It is recommended to run it on port 7878 with multiple threads.

In the *main.rs* file, routes are configured to handle requests:
- `/index.html` serves a static file.
- `/` redirects to `/index.html`.
- `/login` handles login requests (simulating a login process).
Finally, the server is started with the number of threads specified in the parameters.

### Connection Acceptance
The server accepts incoming connections using TCP. Each connection is handled in a separate thread to allow concurrency. Connections are managed in the *serve.rs* file, as seen in this code:
```rust
let ans = serve(&thread_name, &router_shared, stream);
```
The incoming TCP connection is wrapped in a `BufReader` for efficient data reading. Then, the HTTP request is read and parsed.
```rust
let req = Request::read_from(&mut reader).map_err(|e| ServeError::RequestRead(client_ip, e))?;
```

### Request Processing
The **Request::read_from** method reads the request from the stream. This includes the request line (method, path), headers, and body.
```rust
pub fn read_from<R: BufRead>(stream: &mut R) -> Result<Self, Self::Error> { ... }
```

### Route Handling
```rust
let res = router.handle_request(&req);
```
As seen here, the request is passed to the `Router`, which determines how to handle the request based on the path. The router uses `handle_request` to find and execute the appropriate handler, returning a `Response`.

### Response Generation
The response is built based on the path and method. There are several types of responses, such as redirect, success, not found, or internal error.
```rust
pub fn redirect(path: &str) -> Self { ... }
pub fn ok(body: &str) -> Self { ... }
pub fn not_found() -> Self { ... }
pub fn internal_err(body: &str) -> Self { ... }
pub fn file(path: &str) -> Self { ... }
```

### Writing the Response
The response is written to the stream using the *WriteTo* trait from the *serve.rs* file. This includes the status line, headers, and the response body.
```rust
fn write_to<W: Write>(&self, stream: &mut W) -> Result<(), Self::Error> { ... }
```

### Logging
A log is printed to the console with details about the request and the time it took to process.
```rust
println!("#{} [{}] {{{}}} {:?} '{}' -> {} {:.2}ms", ... );
```