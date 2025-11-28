// Request crafting for HTTP/2, TLS, and advanced protocols
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CraftedRequest {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
    pub version: String,
}

impl CraftedRequest {
    pub fn new(method: &str, url: &str) -> Self {
        Self {
            method: method.to_string(),
            url: url.to_string(),
            headers: HashMap::new(),
            body: None,
            version: "HTTP/1.1".to_string(),
        }
    }

    pub fn add_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    pub fn with_http2(mut self) -> Self {
        self.version = "HTTP/2.0".to_string();
        self
    }

    pub fn serialize(&self) -> Vec<u8> {
        tracing::debug!("Serializing request: {} {}", self.method, self.url);
        format!("{} / {}\r\n", self.method, self.version).into_bytes()
    }
}
