use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;
use webby::http::parse_request;
use webby::threading::ThreadPool;

fn main() -> ExitCode {
    match init() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{:?}", e);
            ExitCode::from(5)
        }
    }
}

fn init() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:3")?;

    let pool = ThreadPool::new(4);
    //We're fine with exiting if there is no PWD at the moment.
    let base_path = Arc::new(std::env::current_dir()?);

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
    println!("{:?}", base_path);

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
