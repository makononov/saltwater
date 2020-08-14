use chrono::Utc;
use super::request::Request;
use super::header::Header;
use std::io::{Read, Cursor};

pub struct Response {
    pub version: String,
    pub status: i32,
    pub headers: Vec<Header>,
    pub content: Cursor<Vec<u8>>,
}

impl Response {
    pub fn new(req: &Request) -> Self {
        Response {
            version: String::from(req.version.as_str()),
            status: 200,
            headers: vec![
                Header { key: String::from("Server"), value: String::from("Saltwater/1.0") },
                Header { key: String::from("Date"), value: format!("{}", Utc::now().format("%a, %d %h %Y %T %Z")) }
            ],
            content: Cursor::new(Vec::new()),
        }
    }

    pub fn write_content<R: Read>(&mut self, buffer: &mut R) -> Result<u64, std::io::Error> {
        std::io::copy(buffer, &mut self.content)
    }

    pub fn status_text(&self) -> String {
        match self.status {
            200 => String::from("OK"),
            404 => String::from("Not Found"),
            500 => String::from("Internal Server Error"),
            _ => String::from("UNKNOWN")
        } 
    }
}