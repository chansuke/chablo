//! http server
use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use crate::errors::ChabloError;

pub fn run() -> Result<(), ChabloError> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_connection(stream) {
                    eprintln!("error: {}", e)
                }
            }
            Err(e) => {
                eprintln!("error: {}", e)
            }
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<(), ChabloError> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "./public/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "./public/404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}
