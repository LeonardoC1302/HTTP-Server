# HTTP Server in RUST

This project implements a basic HTTP server in Rust. Below is a general overview of the process and structures involved. The following resources were used as guidance:
- [Rust Fundamentals](https://www.udemy.com/course/rust-fundamentals/)
- [The Rust Programming Language](https://doc.rust-lang.org/beta/book/ch20-02-multithreaded.html)
- [http-server-from-scratch](https://github.com/giuliano-macedo/http-server-from-scratch-rs/tree/master)

## Project Contributors
- Leonardo Céspedes Tenorio - [LeonardoC1302](https://github.com/LeonardoC1302)
- Kevin Chang Chang - [Swonk888](https://github.com/Swonk888)
- Frankmin Feng Zhong - [frank-the-tankk](https://github.com/frank-the-tankk)


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
The server accepts incoming connections using TCP. Each connection is handled in a separate thread to allow concurrency. A basic fixed thread pool is implemented with the amount of threads passed as parameter when running the server.
```rust
for id in 0..no_threads {
    let thread_name = format!("Thread {}", id);
    let thread_tx = tx.clone();
    let listener_shared = Arc::clone(&listener);
    let router_shared = Arc::clone(&router);

    children.push(
        thread::Builder::new()
            .name(thread_name.to_string())
            .spawn(move || loop {
                let stream: StreamType;
                {
                    stream = listener_shared.lock().unwrap().accept();
                }
                let ans = serve(&thread_name, &router_shared, stream);
                thread_tx.send(ans).unwrap();
            })
            .unwrap(),
    );
}
```
Connections are managed in the *serve.rs* file, as seen in this code:
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

## How is concurrency managed?
Concurrency in this server is handled using several Rust primitives to ensure safe, efficient multi-threading.

### Arc & Mutex
Arc (Atomic Reference Counting) allows sharing resources across threads without copying them, and Mutex ensures that only one thread can access the shared resource at a time, preventing race conditions.
```rust
let listener = Arc::new(Mutex::new(TcpListener::bind(self.addr).unwrap()));
let router = Arc::new(self.router.clone());
```
The listener and router are shared across multiple threads with Arc, and listener is wrapped in a Mutex to safely accept connections across threads.

### Mpsc Channels
The server uses a multiple-producer, single-consumer (mpsc) channel to communicate between the threads handling requests and the main thread, which collects and handles the results.
```rust
let (tx, rx) = mpsc::channel();
```
Each worker thread sends the result of the request processing back to the main thread via this channel.

### Fixed Thread Pool
A pool of fixed threads is created during server startup. Each thread runs an infinite loop, accepting and processing connections. The number of threads is set via command-line arguments.
```rust
for id in 0..no_threads {
    let thread_name = format!("Thread {}", id);
    let thread_tx = tx.clone();
    let listener_shared = Arc::clone(&listener);
    let router_shared = Arc::clone(&router);

    children.push(
        thread::Builder::new()
            .name(thread_name.to_string())
            .spawn(move || loop {
                let stream: StreamType;
                {
                    stream = listener_shared.lock().unwrap().accept();
                }
                let ans = serve(&thread_name, &router_shared, stream);
                thread_tx.send(ans).unwrap();
            })
            .unwrap(),
    );
}
```
Each thread accepts connections from the shared listener and processes requests concurrently.

### Panic Managing
To ensure that the server handles panics gracefully, a custom panic hook is used. If any thread panics, the server terminates cleanly, avoiding inconsistent states.
```rust
panic::set_hook(Box::new(move |panic_info| {
    println!("A thread has panicked, ending server");
    process::exit(-1);
}));
```
This ensures that if a thread encounters a fatal error, the entire server shuts down properly.

## How are cookies managed?
Cookies are handled by parsing the Cookie header from incoming requests and then setting cookies in the HTTP response when needed.
### Receiving Cookies
When a request is received, the server checks if the Cookie header exists. If it does, the header is parsed to extract individual cookie key-value pairs, which are stored in a HashMap.
### Setting Cookies
Once cookies are parsed and processed, they can be added to the HTTP response using a method like set_cookie. This ensures the browser receives the cookies.
```rust
pub fn set_cookie(&mut self, cookies: HashMap<String, String>) {
    let mut cookie_string = String::new();
    for (key, value) in cookies {
        if !cookie_string.is_empty() {
            cookie_string.push_str("; "); // Separate multiple cookies with a semicolon
        }
        cookie_string.push_str(&format!("{}={}", key, value));
    }

    // Store the concatenated cookie string in the Set-Cookie header
    self.headers.insert("Set-Cookie".to_string(), cookie_string);
}
```
The set_cookie method adds the cookies to the Set-Cookie header of the response, which instructs the client (browser) to store these cookies.
## Project Testing
### Unit Testing
Unit tests have been implemented for the HTTP server using Rust's built-in testing framework. These tests ensure the correct functionality of various server components and endpoints. To run the test suite, simply execute the command cargo test in the `server` directory. This will compile the code and run all defined tests, providing a summary of the results.
### Other Tests
A Python script was created ([test_request](./test_request.py)) to perform a concurrency test by sending multiple simultaneous GET requests to the HTTP server using threads. This test simulates concurrent client interactions, enabling an evaluation of the server's ability to manage multiple connections in parallel. By increasing the number of threads, the test can also serve as a stress test, pushing the server to its performance limits. The results provide valuable insights into potential bottlenecks, response latency, and overall server stability under high traffic conditions.

Postman was also used to test the endpoints for both functional and performance evaluation by sending multiple requests. To run the collection for testing, simply import the provided Postman collection ([postman-collection](./postman-collection.json)), select the desired environment and use the "Run Colection" option to execute multiple iterations of the requests for performance testing or single runs for functional testing. You can also configure delays between requests and set the number of iterations to simulate different load conditions.
## How to run the server?
To run the server follow this steps:
- Clone the github repository or download the compressed version.
- Make sure you have rust installed on your device. You can install it from here: [Install Rust](https://www.rust-lang.org/tools/install)
- Head to the `server` directory, since that folder contains all the files from the HTTP Server.
- Run `cargo r <port> <no_threads>` to compile and execute the server. Its recommended to use port 7878.
- Open `localhost:port` on your preferred browser. You can visit the following routes: '/', '/index.html', '/login'.