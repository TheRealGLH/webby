use std::{fs, path::PathBuf};

pub fn parse_request(request_line: &str, base_path: &PathBuf) -> HttpResponse {
    println!("{request_line}");

    let (method, uri) = match parse_http_request_line(request_line) {
        Some(s) => s,
        None => return HttpResponse::internal_error(),
    };
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

#[derive(Debug)]
enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Options,
    Connect,
    Trace,
    Patch,
}

impl HttpMethod {
    fn from_str(s: &str) -> Option<HttpMethod> {
        match s {
            "GET" => Some(HttpMethod::Get),
            "POST" => Some(HttpMethod::Post),
            "PUT" => Some(HttpMethod::Put),
            "DELETE" => Some(HttpMethod::Delete),
            "HEAD" => Some(HttpMethod::Head),
            "OPTIONS" => Some(HttpMethod::Options),
            "CONNECT" => Some(HttpMethod::Connect),
            "TRACE" => Some(HttpMethod::Trace),
            "PATCH" => Some(HttpMethod::Patch),
            _ => None, // Unknown method
        }
    }
}

fn parse_http_request_line(request_line: &str) -> Option<(HttpMethod, &str)> {
    // Expected format: "METHOD PATH HTTP/VERSION"
    let mut parts = request_line.splitn(3, ' ');

    // Extract and validate the method
    let method_str = parts.next()?;
    let method = HttpMethod::from_str(method_str)?;

    // Extract the path
    let path = parts.next()?;

    Some((method, path))
}

pub struct HttpResponse {
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
