//! http server
use std::fs::File;
use std::io::{prelude::*, Read};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

use log::{error, info, warn};

use crate::errors::ChabloError;

pub fn serve() -> Result<(), ChabloError> {
    let address = "localhost:8080";
    let listener = TcpListener::bind(address)?;

    let url = format!("http://{}", address);

    let blue_color = "\x1b[34m";
    let reset_color = "\x1b[0m";
    println!("Listening on: {}{}{}", blue_color, url, reset_color);
    info!("Listening on: {}", url);

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_connection(stream) {
                    error!("error: {}", e)
                }
            }
            Err(e) => {
                error!("error: {}", e)
            }
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<(), ChabloError> {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    if bytes_read > 0 {
        info!(
            "Request: {}",
            String::from_utf8_lossy(&buffer[..bytes_read])
        );
    } else {
        warn!("No data received from the stream.");
    }

    // Parse the request
    let binding = String::from_utf8_lossy(&buffer[..]);
    let request_line = binding.lines().next().unwrap_or("");
    let request_path = request_line.split_whitespace().nth(1).unwrap_or("/");
    let decoded_path = decode_percent_encoded_string(request_path)?;

    let request_path = if decoded_path == "/" {
        "/index.html"
    } else {
        &decoded_path
    };
    let filename = format!("./public{}", request_path);

    // Check if the file exists and is a .html file
    let (status_line, filename) = if Path::new(&filename).exists() && filename.ends_with(".html") {
        ("HTTP/1.1 200 OK\r\n\r\n", filename)
    } else {
        (
            "HTTP/1.1 404 NOT FOUND\r\n\r\n",
            "./public/404.html".to_string(),
        )
    };

    let mut file = File::open(filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let response = format!("{}{}", status_line, contents);

    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}

fn decode_percent_encoded_string(encoded: &str) -> Result<String, ChabloError> {
    let mut bytes = Vec::new();
    let mut chars = encoded.chars();

    while let Some(ch) = chars.next() {
        if ch == '%' {
            let hex = chars.next().unwrap().to_string() + &chars.next().unwrap().to_string();
            let byte = u8::from_str_radix(&hex, 16)?;
            bytes.push(byte);
        } else {
            bytes.push(ch as u8);
        }
    }

    Ok(String::from_utf8_lossy(&bytes).to_string())
}
