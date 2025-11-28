// WebAssembly integration
// Pure functions only - no async, no tokio dependencies

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

/// Pure WASM function: Simple greeting
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// Browser automation wrapper for WASM
/// Note: Async operations should use wasm-bindgen-futures or JavaScript promises
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct BrowserAutomation {
    url: String,
    user_agent: String,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl BrowserAutomation {
    /// Create new BrowserAutomation instance
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new(url: String) -> BrowserAutomation {
        BrowserAutomation {
            url,
            user_agent: get_random_user_agent(),
        }
    }

    /// Navigate to a new URL (returns new instance)
    pub fn navigate(&self, new_url: &str) -> BrowserAutomation {
        BrowserAutomation {
            url: new_url.to_string(),
            user_agent: self.user_agent.clone(),
        }
    }

    /// Get current URL
    pub fn get_url(&self) -> String {
        self.url.clone()
    }

    /// Get current user agent
    pub fn get_user_agent(&self) -> String {
        self.user_agent.clone()
    }

    /// Prepare login data (sync - actual login handled by JavaScript)
    /// Returns JSON with credentials for JavaScript to process
    pub fn prepare_login(&self, email: &str, password: &str) -> String {
        serde_json::json!({
            "url": self.url,
            "email": email,
            "password": password,
            "user_agent": self.user_agent,
            "headers": create_stealth_headers_json(),
        })
        .to_string()
    }

    /// Prepare register data (sync - actual registration handled by JavaScript)
    pub fn prepare_register(&self, email: &str, password: &str) -> String {
        serde_json::json!({
            "url": self.url,
            "email": email,
            "password": password,
            "user_agent": self.user_agent,
            "headers": create_stealth_headers_json(),
        })
        .to_string()
    }
}

/// Get random user agent for anti-detection
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn get_random_user_agent() -> String {
    crate::anti_detection::get_random_user_agent()
}

/// Create stealth headers as JSON string
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn create_stealth_headers_json() -> String {
    let headers = crate::anti_detection::AntiDetectionHeaders::default();
    let map = headers.to_hashmap();
    serde_json::to_string(&map).unwrap_or_default()
}
