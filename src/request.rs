use std::fmt;
use std::net::{TcpStream};
use std::io::{BufRead, BufReader};

use super::header::Header;

pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: Vec<Header>,
    pub body: String,
}

impl Request {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn parse(&mut self, stream: &TcpStream) -> Result<(), std::io::Error> {
        let mut reader = BufReader::new(stream.try_clone().unwrap());

        // Parse request line
        let mut request_line = String::new();
        let _ = reader.read_line(&mut request_line)?;
        let tokenized_request_line: Vec<&str> = request_line.split_whitespace().collect();
        self.method.push_str(tokenized_request_line[0]);
        self.path.push_str(tokenized_request_line[1]);
        self.version.push_str(tokenized_request_line[2]);

        // Parse headers
        let mut buffer: String;
        while {
            buffer = String::new();
            let _ = reader.read_line(&mut buffer)?;
            buffer.trim() != ""
        } {
            let tokenized_header: Vec<&str> = buffer.splitn(2, ": ").collect();
            self.headers.push(Header::new(String::from(tokenized_header[0].trim()), String::from(tokenized_header[1].trim())));
        }

        // Parse body 
        // let mut buffer: String;
        // while {
        //     buffer = String::new();
        //     let data_size = reader.read_line(&mut buffer)?;
        //     data_size != 0 || buffer.trim() != ""
        // } {
        //     self.body.push_str(&buffer);
        // }

        Ok(())
    }
}

impl Default for Request {
    fn default() -> Self {
        Request {
            method: String::new(),
            path: String::new(),
            version: String::new(),
            headers: vec![],
            body: String::new()
        }
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.method, self.path, self.version)
    }
}