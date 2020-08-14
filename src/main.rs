use std::net::{TcpListener, TcpStream};
use std::thread;
use chrono::Utc;
use std::io::{Write, ErrorKind};
use std::path::Path;
use std::fs::File;

mod request;
mod response;
mod header;

use request::Request;
use response::Response;
use header::Header;

const HTML_ROOT: &str = "/Users/misha/work/saltwater/html";

fn default_handler(resp: &mut Response, r: &Request) -> Result<(), std::io::Error> {
    let root = Path::new(HTML_ROOT);
    let relative_path = Path::new(&r.path).strip_prefix("/").unwrap();
    let mut full_path = root.join(relative_path);

    if full_path.is_dir() {
        full_path = full_path.join("index.html");
    }

    let mut file = match File::open(&full_path) {
        Ok(file) => file,
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => {
                    resp.status = 404;
                    return Ok(());
                },
                _ => {
                    return Err(err);
                }
            }
        }
    };
    let content_length = resp.write_content(&mut file)?;
    resp.headers.push(Header::new(String::from("Content-length"), format!("{}", content_length)));

    // Content Headers
    let content_type = match full_path.extension().unwrap().to_str().unwrap() {
        "html" => "text/html",
        "jpg" => "image/jpeg",
        _ => "text/plain",
    };
    resp.headers.push(Header::new(String::from("Content-type"), String::from(content_type)));

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut req = Request::new();
    let _ = req.parse(&stream)?;

    // build response
    let mut resp = Response::new(&req);
    let _ = match default_handler(&mut resp, &req) {
        Ok(size) => size,
        Err(err) => {
            resp.status = 500;
            println!("Unhandled exception: {}", err);
        },
    };
    resp.content.set_position(0);

    // write response
    write!(stream, "{} {} {}\r\n", resp.version, resp.status, resp.status_text()).unwrap();
    for header in &resp.headers {
        let _ = write!(stream, "{}: {}\r\n", header.key, header.value)?;
    }
    let _ = write!(stream, "\r\n")?;
    let content_length = std::io::copy(&mut resp.content, &mut stream).unwrap();
    stream.flush().unwrap();

    let peer_addr = match stream.peer_addr() {
        Ok(addr) => addr,
        Err(_) => "0.0.0.0".parse().unwrap(),
    };
    println!("{} - - [{}] \"{}\" {} {}", peer_addr, Utc::now().format("%d/%b/%Y:%H:%M:%S%z"), req, resp.status, content_length);
    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("Initializing listener");
    let addr = "127.0.0.1:8080";
    let listener = match TcpListener::bind(addr) {
        Ok(l) => {
            println!("Listening on {}...", addr);
            l
        },
        Err(err) => return Err(err)
    };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move||{ handle_client(stream)});
            },
            Err(err) => return Err(err)
        };
    }

    drop(listener);
    Ok(())
}
