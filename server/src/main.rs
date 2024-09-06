use std::{
    fs, // Standard library’s filesystem module
    io::{prelude::*, BufReader}, // Read from and write to the stream
    net::{TcpListener, TcpStream}, // Listen for incoming connections
};

mod http;
use http::{Request, Method, ReadFrom};

fn handle_connection(mut stream: TcpStream){
    let mut buf_reader = BufReader::new(&mut stream);
    let request = Request::read_from(&mut buf_reader);
    println!("{:#?}", request);
    
    let (status_line, filename) = if request.unwrap().method == Method::GET {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream)
    }
}