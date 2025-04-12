use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use webby::http::parse_request;
use webby::threading::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(tcp_stream) => {
                pool.execute(|| {
                    handle_connection(tcp_stream);
                });
            }
            Err(e) => println!("Parsing incoming stream failed {}", &e),
        };
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let response = &request_line[..];

    let response = stream
        .write_all(parse_request(response).to_string().as_bytes())
        .unwrap();

    //println!("Request: {http_request:#?}");
}
