use std::io::prelude::*;
use std::error::Error;
use std::path::Path;
use std::net::TcpStream;
use std::net::TcpListener;
use log::{error, info};

#[derive(Debug)]
struct Request<'a> {
    method: &'a str,
    uri: &'a Path,
    http_version: &'a str,
}

fn parse_request_line(request: &str) -> Result<Request, Box<dyn Error>> {
    let mut parts = request.split_whitespace();

    let method = parts.next().ok_or("Method not specified")?;
    // GET in or get out
    if method != "GET" {
        Err("Unsupported method")?;
    }

    let uri = Path::new(parts.next().ok_or("URI not specified")?);
    
    let http_version = parts.next().ok_or("HTTP version not specified").unwrap();

    if http_version != "HTTP/1.1" {
        Err("Unsupported HTTP version")?;
    }

    Ok(Request {
        method,
        uri,
        http_version
    })
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap();

    match parse_request_line(&request_line) {
        Ok(request) => {
            info!("\n{:?}", request);

            let message = "hello, world!";
            let response = format!("{}{}", "HTTP/1.1 200 OK\r\n\r\n", message);
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(err) => error!("Badly formatted request: {} {}", &request_line, err),
    }
    Ok(())
}

fn main() {
    simple_logger::init().unwrap();
    info!("Starting server...");
    
    let ip = "127.0.0.1:8080";

    let listener = TcpListener::bind(ip).expect("Unable to create TCP listener");
    info!("Server started on: {}{}", "http://", ip);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => match handle_connection(stream) {
                Ok(_) => (),
                Err(e) => error!("Error handling connection: {}", e),
            },
            Err(e) => error!("Connection failed: {}", e),
        }
    }
}
