use std::{
    fs, // Standard library’s filesystem module
    io::{prelude::*, BufReader}, // Read from and write to the stream
    net::{TcpListener, TcpStream}, // Listen for incoming connections
};

mod http;
use http::{Request, Method, Response, ReadFrom, WriteTo};

fn handle_connection(mut stream: TcpStream){
    let mut buf_reader = BufReader::new(&mut stream);
    let request = Request::read_from(&mut buf_reader);
    
    let (status_line, filename) = if request.unwrap().method == Method::GET {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = Response::ok(&contents);

    // Send response
    match response.write_to(&mut stream){
        Ok(_) => println!("Response sent"),
        Err(e) => eprintln!("Failed sending response: {}", e)
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream)
    }
}