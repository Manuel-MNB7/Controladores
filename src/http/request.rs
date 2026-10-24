use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpRequest {
    pub fn parse(stream: &TcpStream) -> Result<HttpRequest, String> {
        let mut reader = BufReader::new(stream);
        let mut first_line = String::new();
        if reader.read_line(&mut first_line).map_err(|e| e.to_string())? == 0 {
            return Err("Empty request".to_string());
        }

        let parts: Vec<&str> = first_line.split_whitespace().collect();
        if parts.len() < 2 {
            return Err("Invalid HTTP request line".to_string());
        }

        let method = parts[0].to_string();
        let path = parts[1].to_string();

        let mut headers = HashMap::new();
        let mut content_length = 0;

        loop {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line).map_err(|e| e.to_string())?;
            if bytes_read == 0 || line == "\r\n" || line == "\n" {
                break;
            }

            if let Some((key, value)) = line.split_once(':') {
                let header_name = key.trim().to_lowercase();
                let header_val = value.trim().to_string();
                if header_name == "content-length" {
                    if let Ok(len) = header_val.parse::<usize>() {
                        content_length = len;
                    }
                }
                headers.insert(header_name, header_val);
            }
        }

        let mut body = vec![0u8; content_length];
        if content_length > 0 {
            reader.read_exact(&mut body).map_err(|e| e.to_string())?;
        }

        Ok(HttpRequest {
            method,
            path,
            headers,
            body,
        })
    }
}
