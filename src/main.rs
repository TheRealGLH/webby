use webby::ThreadPool;
use regex::Regex;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::{env, fs};

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

fn parse_request(request_line: &str) -> HttpResponse {
    //TODO: this will be done through a config option later.
    println!("{request_line}");
    let base_path = env::current_dir().unwrap();
    let regex = match Regex::new(
        "(GET|POST|PUT|PATCH|DELETE|TRACE|CONNECT|HEAD)\\s(\\S*)\\sHTTP\\/1\\.1",
    ) {
        Ok(r) => r,
        Err(_) => {
            //there was an error with compiling the regex
            return HttpResponse::internal_error();
        }
    };

    let caps = match regex.captures(request_line) {
        Some(c) => c,
        None => {
            //the request was borked
            return HttpResponse::internal_error();
        }
    };
    let uri = &caps[2];

    let filename = if !uri.ends_with('/') {
        uri.to_string()
    } else {
        format!("{uri}index.html")
    }
    .replacen("/", "", 1);
    let path = base_path.join(filename);
    println!("{:?}", &base_path);
    match fs::read_to_string(&path) {
        Ok(content) => HttpResponse {
            status_line: "HTTP/1.1 200 OK".to_string(),
            content,
        },
        Err(e) => {
            println!("File not found: {e}\n{path:?}");
            HttpResponse::not_found()
        }
    }
}

struct HttpResponse {
    status_line: String,
    content: String,
}

impl HttpResponse {
    fn internal_error() -> Self {
        Self {
            status_line: String::from("HTTP/1.1 500 INTERNAL ERROR"),
            content: String::from("Internal error."),
        }
    }

    fn not_found() -> Self {
        Self {
            status_line: String::from("HTTP/1.1 404 NOT FOUND"),
            content: String::from("Not found."),
        }
    }
}

impl ToString for HttpResponse {
    fn to_string(&self) -> String {
        format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            &self.status_line,
            self.content.len(),
            self.content
        )
    }
}
