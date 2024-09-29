use super::{serve, Callback, Response, Router, StreamType};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::panic;
use std::process;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

#[derive(Clone)]
/// Representa un servidor HTTP
pub struct Server {
    addr: SocketAddr,
    router: Router,
}

impl Server {
    /// Crea una nueva instancia de Server
    pub fn new(ip: &str, port: &str) -> Self {
        Self {
            addr: SocketAddr::new(
                IpAddr::V4(ip.parse::<Ipv4Addr>().unwrap()),
                port.parse::<u16>().unwrap(),
            ),
            router: Router::new(),
        }
    }

    /// Inicia el servidor con un número específico de hilos
    pub fn run(&self, no_threads: usize) {
        // Configura un manejador de pánico para terminar el proceso si un hilo entra en pánico
        let orig_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            println!("A thread has panicked, ending server");
            orig_hook(panic_info);
            process::exit(-1);
        }));

        // Crea el listener TCP y lo envuelve en un Arc<Mutex>
        let listener = Arc::new(Mutex::new(TcpListener::bind(self.addr).unwrap()));
        let router = Arc::new(self.router.clone());
        println!(
            "Listening on http://{} with {} threads.",
            self.addr, no_threads
        );

        let mut children = Vec::with_capacity(no_threads);
        let (tx, rx) = mpsc::channel();

        // Crea los hilos de trabajo
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

        // Procesa los resultados de los hilos
        for ans in rx {
            match ans {
                Ok(_) => (),
                Err(e) => {
                    println!("[Error] {}", e)
                }
            }
        }
    }

    /// Registra un callback para una ruta específica
    pub fn on(&mut self, pat: &str, cb: Callback) {
        self.router.insert_callback(pat, cb);
    }

    /// Registra un archivo para ser servido en una ruta específica
    pub fn on_file(&mut self, pat: &str, fname: &str) {
        self.router.insert_file(pat, fname);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;

    #[test]
    // prueba de creación de un servidor
    fn test_server_new() {
        let server = Server::new("127.0.0.1", "8080");
        assert_eq!(
            server.addr,
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)
        );
    }

    #[test]
    // prueba de registro de un callback
    fn test_server_on() {
        let mut server = Server::new("127.0.0.1", "8080");
        assert_eq!(server.router.route_count(), 0);
        server.on("/test", |_req| Response::ok("Test response"));
        assert_eq!(server.router.route_count(), 1);
        assert!(server.router.has_route("/test"));
    }

    #[test]
    // prueba de registro de un archivo
    fn test_server_on_file() {
        let mut server = Server::new("127.0.0.1", "8080");

        // deberia estar en 0 pq no hay archivs registrados
        assert_eq!(server.router.route_count(), 0);
        assert!(!server.router.has_route("/test"));

        server.on_file("/test", "test.html");

        // deberia estar en 1 pq hay un archivo registrado
        assert_eq!(server.router.route_count(), 1);
        assert!(server.router.has_route("/test"));

        server.on_file("/another", "another.html");

        // deberia estar en 2 pq hay dos archivos registrados
        assert_eq!(server.router.route_count(), 2);
        assert!(server.router.has_route("/test"));
        assert!(server.router.has_route("/another"));
    }
}
