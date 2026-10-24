use serde::Serialize;
use std::io::Write;
use std::net::TcpStream;

#[derive(Debug)]
pub struct HttpResponse {
    pub status_code: u16,
    pub status_text: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn new(status_code: u16, status_text: &str, body: Vec<u8>, content_type: &str) -> Self {
        Self {
            status_code,
            status_text: status_text.to_string(),
            headers: vec![
                ("Content-Type".to_string(), content_type.to_string()),
                ("Content-Length".to_string(), body.len().to_string()),
                ("Connection".to_string(), "close".to_string()),
            ],
            body,
        }
    }

    pub fn json<T: Serialize>(status_code: u16, status_text: &str, payload: &T) -> Self {
        let body = serde_json::to_vec(payload).unwrap_or_else(|_| b"{}".to_vec());
        Self::new(status_code, status_text, body, "application/json")
    }

    pub fn ok_json<T: Serialize>(payload: &T) -> Self {
        Self::json(200, "OK", payload)
    }

    pub fn created_json<T: Serialize>(payload: &T) -> Self {
        Self::json(201, "Created", payload)
    }

    pub fn error_json(status_code: u16, status_text: &str, message: &str) -> Self {
        #[derive(Serialize)]
        struct ErrorResponse<'a> {
            error: &'a str,
        }
        Self::json(status_code, status_text, &ErrorResponse { error: message })
    }

    pub fn send(&self, mut stream: &TcpStream) -> std::io::Result<()> {
        let mut response_bytes = Vec::new();
        let header_str = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_text);
        response_bytes.extend_from_slice(header_str.as_bytes());

        for (k, v) in &self.headers {
            let line = format!("{}: {}\r\n", k, v);
            response_bytes.extend_from_slice(line.as_bytes());
        }

        response_bytes.extend_from_slice(b"\r\n");
        response_bytes.extend_from_slice(&self.body);

        stream.write_all(&response_bytes)?;
        stream.flush()?;
        Ok(())
    }
}
