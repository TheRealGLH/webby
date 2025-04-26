pub mod config;
mod http;
mod threading;
use config::*;
use http::parse_request;
use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::Arc;
use threading::ThreadPool;

pub fn init(config: Configuration) -> Result<(), std::io::Error> {
    if config.help {
        print_help();
        return Ok(());
    }
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    let listener = TcpListener::bind(addr)?;
    println!("Starting server on: http://{}", addr);

    let pool = ThreadPool::new(4);
    let base_path = Arc::new(std::fs::canonicalize(PathBuf::from(config.directory))?);

    println!("Base path: {:?}", base_path);
    for stream in listener.incoming() {
        let base_path = Arc::clone(&base_path);
        match stream {
            Ok(tcp_stream) => {
                pool.execute(move || {
                    handle_connection(tcp_stream, &base_path);
                });
            }
            Err(e) => println!("error parsing tcp stream: {}", e),
        };
    }

    println!("Shutting down.");
    Ok(())
}

fn handle_connection(mut stream: TcpStream, base_path: &PathBuf) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let response = &request_line[..];

    stream
        .write_all(parse_request(response, base_path).to_string().as_bytes())
        .unwrap();
}
