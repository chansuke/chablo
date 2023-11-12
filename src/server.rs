//! http server
use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::errors::ChabloError;

pub fn run() -> Result<(), ChabloError> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Err(e) => {
                eprintln!("error: {}", e)
            }
            Ok(stream) => {
                println!("stream: {:?}", stream);
            }
        }
    }

    Ok(())
}
