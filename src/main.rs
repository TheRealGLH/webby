use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::Arc;
use webby::http::parse_request;
use webby::threading::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    //We're fine with exiting if there is no PWD at the moment.
    let base_path = Arc::new(std::env::current_dir().unwrap());

    for stream in listener.incoming() {
        let base_path = Arc::clone(&base_path);
        match stream {
            Ok(tcp_stream) => {
                pool.execute(move || {
                    handle_connection(tcp_stream, &base_path);
                });
            }
            Err(e) => println!("Parsing incoming stream failed {}", &e),
        };
    }
    println!("{:?}", base_path);

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream, base_path: &PathBuf) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let response = &request_line[..];

    stream
        .write_all(parse_request(response, base_path).to_string().as_bytes())
        .unwrap();
}
