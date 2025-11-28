// Anti-detection and fingerprinting evasion
use rand::seq::SliceRandom;

// Extensive list of realistic user agents
const USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0",
    "Mozilla/5.0 (X11; Linux x86_64; rv:121.0) Gecko/20100101 Firefox/121.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1 Safari/605.1.15",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Edge/121.0.0.0",
];

pub fn get_random_user_agent() -> String {
    USER_AGENTS
        .choose(&mut rand::thread_rng())
        .unwrap_or(&"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .to_string()
}

pub struct AntiDetectionHeaders {
    pub user_agent: String,
    pub accept_language: String,
    pub accept_encoding: String,
    pub referer: String,
}

impl Default for AntiDetectionHeaders {
    fn default() -> Self {
        Self {
            user_agent: get_random_user_agent(),
            accept_language: "es-ES,es;q=0.9,en;q=0.8".to_string(),
            accept_encoding: "gzip, deflate, br".to_string(),
            referer: "https://www.google.com/".to_string(),
        }
    }
}

impl AntiDetectionHeaders {
    pub fn to_hashmap(&self) -> std::collections::HashMap<String, String> {
        let mut map = std::collections::HashMap::new();
        map.insert("User-Agent".to_string(), self.user_agent.clone());
        map.insert("Accept-Language".to_string(), self.accept_language.clone());
        map.insert("Accept-Encoding".to_string(), self.accept_encoding.clone());
        map.insert("Referer".to_string(), self.referer.clone());
        map.insert("Sec-Fetch-Dest".to_string(), "document".to_string());
        map.insert("Sec-Fetch-Mode".to_string(), "navigate".to_string());
        map.insert("Sec-Fetch-Site".to_string(), "none".to_string());
        map.insert("Cache-Control".to_string(), "max-age=0".to_string());
        map
    }
}

pub fn stealth_mode_scripts() -> Vec<&'static str> {
    vec![
        // Override navigator.webdriver
        r#"
        Object.defineProperty(navigator, 'webdriver', {
            get: () => false,
        });
        "#,
        // Override chrome detection
        r#"
        window.chrome = {
            runtime: {}
        };
        "#,
        // Spoof plugins
        r#"
        Object.defineProperty(navigator, 'plugins', {
            get: () => [1, 2, 3, 4, 5],
        });
        "#,
    ]
}

pub struct ProxyManager {
    proxies: Vec<String>,
    current_index: usize,
}

impl ProxyManager {
    pub fn new(proxies: Vec<String>) -> Self {
        Self {
            proxies,
            current_index: 0,
        }
    }

    pub fn next_proxy(&mut self) -> Option<&String> {
        if self.proxies.is_empty() {
            return None;
        }
        let proxy = &self.proxies[self.current_index];
        self.current_index = (self.current_index + 1) % self.proxies.len();
        Some(proxy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_user_agent() {
        let ua1 = get_random_user_agent();
        let ua2 = get_random_user_agent();
        assert!(!ua1.is_empty());
        assert!(!ua2.is_empty());
    }

    #[test]
    fn test_anti_detection_headers() {
        let headers = AntiDetectionHeaders::default();
        let map = headers.to_hashmap();
        assert!(map.contains_key("User-Agent"));
        assert!(map.contains_key("Referer"));
    }
}
