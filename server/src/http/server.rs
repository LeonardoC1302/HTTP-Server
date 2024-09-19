use super::{serve, Callback, Router, StreamType};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::panic;
use std::process;

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
        println!("Listening on http://{} with {} threads.", self.addr, no_threads);

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