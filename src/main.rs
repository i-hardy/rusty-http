use rusty_http::handle_connection;
use std::net::TcpListener;
use log::{error, info};

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
