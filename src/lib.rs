/// WASM SCRAPER - Ultra avanzado anti-detección + DOM parsing
/// Compilable con: wasm-pack build --target web

pub mod scraper;  // Marketing scraper con DB support
pub mod scraper_marketing;  // Scraper REAL con emails, phones, etc
pub mod wasm_automation;
pub mod wasm_examples_1;  // Parallel Form Filling
pub mod wasm_examples_2;  // Parallel CAPTCHA Solving
pub mod wasm_examples_3;  // Parallel Data Extraction
pub mod wasm_examples_4;  // Parallel Vulnerability Scanning
pub mod wasm_tool_local_testing;  // Local server testing
pub mod wasm_tool_safe_navigation;  // Free navigation
pub mod wasm_tool_security_sandbox;  // Security sandbox

// NEW EPIC FEATURES
pub mod workflow_orchestrator;  // Task scheduling + automation
pub mod vulnerability_scanner;  // OWASP Top 10 detection
pub mod social_media_intelligence;  // Twitter, LinkedIn, Instagram intel
pub mod intelligent_content_extractor;  // Valuable data finder
pub mod geolocation_security_intelligence;  // Geographic security hotspots
pub mod database_persistence;  // RocksDB Native Rust persistence
pub mod cloud_persistence;  // GitHub Gists PRIVADOS (gratis + seguro)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapedElement {
    pub tag: String,
    pub text: String,
    pub attributes: HashMap<String, String>,
    pub classes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetRow {
    pub fields: HashMap<String, String>,
}

/// Ultra-rápido DOM parser SIN regex (compilado a WASM)
pub struct DOMScraper {
    html: String,
}

impl DOMScraper {
    pub fn new(html: &str) -> Self {
        Self {
            html: html.to_string(),
        }
    }

    /// Extrae emails (patrón: palabra@palabra.ext)
    pub fn extract_emails(&self) -> Vec<String> {
        let mut emails = Vec::new();
        let bytes = self.html.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            if let Some(at_idx) = self.find_char_from(&bytes, b'@', i) {
                // Busca inicio: alphanumeric o ._%+-
                let mut start = at_idx;
                while start > 0 && self.is_email_char(bytes[start - 1]) {
                    start -= 1;
                }
                // Busca fin: alphanumeric o .
                let mut end = at_idx + 1;
                while end < bytes.len() && self.is_email_char(bytes[end]) {
                    end += 1;
                }

                if start < at_idx && end > at_idx + 1 {
                    let email_str = String::from_utf8_lossy(&bytes[start..end]);
                    if email_str.contains('.') {
                        emails.push(email_str.to_string());
                        i = end;
                        continue;
                    }
                }
            }
            i += 1;
        }

        emails.into_iter().collect::<std::collections::HashSet<_>>().into_iter().collect()
    }

    /// Extrae teléfonos (patrón: +XX XXX-XXX-XXXX)
    pub fn extract_phones(&self) -> Vec<String> {
        let mut phones = Vec::new();
        let bytes = self.html.as_bytes();

        for i in 0..bytes.len() {
            if bytes[i] == b'+' || (i + 9 < bytes.len() && self.is_digit(bytes[i])) {
                let start = i;
                let mut j = i;
                let mut digit_count = 0;

                while j < bytes.len() && digit_count < 11 {
                    if self.is_digit(bytes[j]) {
                        digit_count += 1;
                    } else if bytes[j] != b' ' && bytes[j] != b'-' && bytes[j] != b'(' && bytes[j] != b')' {
                        break;
                    }
                    j += 1;
                }

                if digit_count >= 10 {
                    phones.push(String::from_utf8_lossy(&bytes[start..j]).to_string());
                }
            }
        }

        phones.into_iter().collect::<std::collections::HashSet<_>>().into_iter().collect()
    }

    /// Extrae links
    pub fn extract_links(&self) -> Vec<String> {
        let mut links = Vec::new();
        let mut i = 0;
        let bytes = self.html.as_bytes();

        while i < bytes.len() - 5 {
            // Busca "href="
            if bytes[i] == b'h' && bytes.get(i + 4) == Some(&b'=') {
                if &bytes[i..i + 5] == b"href=" {
                    let start = i + 5;
                    let quote = bytes[start];

                    // Busca cierre de quote
                    let mut j = start + 1;
                    while j < bytes.len() && bytes[j] != quote {
                        j += 1;
                    }

                    if j < bytes.len() {
                        let url = String::from_utf8_lossy(&bytes[start + 1..j]);
                        links.push(url.to_string());
                        i = j + 1;
                        continue;
                    }
                }
            }
            i += 1;
        }

        links
    }

    /// Extrae tablas (simplificado)
    pub fn extract_tables(&self) -> Vec<Vec<HashMap<String, String>>> {
        vec![] // Simplificado por ahora - el scraping está en web_scrape
    }

    /// Helpers
    fn find_char_from(&self, bytes: &[u8], ch: u8, start: usize) -> Option<usize> {
        bytes[start..].iter().position(|&b| b == ch).map(|p| start + p)
    }

    fn is_email_char(&self, b: u8) -> bool {
        (b >= b'a' && b <= b'z') || (b >= b'A' && b <= b'Z') || (b >= b'0' && b <= b'9') || b == b'.' || b == b'_' || b == b'%' || b == b'+' || b == b'-'
    }

    fn is_digit(&self, b: u8) -> bool {
        b >= b'0' && b <= b'9'
    }
}

/// Anti-detection stealth
pub struct StealthMode;

impl StealthMode {
    pub fn random_user_agent() -> String {
        let agents = vec![
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36",
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36",
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_2_1 like Mac OS X) AppleWebKit/605.1.15",
        ];
        let idx = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as usize)
            .unwrap_or(0)) % agents.len();
        agents[idx].to_string()
    }

    pub fn fake_headers() -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("User-Agent".to_string(), Self::random_user_agent());
        headers.insert("Accept".to_string(), "text/html,application/xhtml+xml".to_string());
        headers.insert("Accept-Language".to_string(), "en-US,en;q=0.9".to_string());
        headers.insert("DNT".to_string(), "1".to_string());
        headers.insert("Connection".to_string(), "keep-alive".to_string());
        headers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_emails() {
        let html = "Contact: test@example.com or info@test.org";
        let scraper = DOMScraper::new(html);
        let emails = scraper.extract_emails();
        assert!(emails.len() >= 2);
    }

    #[test]
    fn test_extract_links() {
        let html = r#"<a href="https://example.com">Link</a>"#;
        let scraper = DOMScraper::new(html);
        let links = scraper.extract_links();
        assert!(links.len() >= 1);
    }
}