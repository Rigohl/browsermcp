// Browser automation module
use crate::core::{BrowserInstance, Cookie, Result};
use std::collections::HashMap;
use uuid::Uuid;

pub struct BrowserPool {
    browsers: Vec<BrowserInstance>,
    max_instances: usize,
}

impl BrowserPool {
    pub fn new(max_instances: usize) -> Self {
        Self {
            browsers: Vec::new(),
            max_instances,
        }
    }

    pub async fn create_instance(&mut self, url: &str) -> Result<String> {
        if self.browsers.len() >= self.max_instances {
            return Err("Max browser instances reached".into());
        }

        let id = format!("browser_{}", Uuid::new_v4());
        let instance = BrowserInstance {
            id: id.clone(),
            url: url.to_string(),
            user_agent: super::anti_detection::get_random_user_agent(),
            proxy: None,
            cookies: Vec::new(),
            headers: HashMap::new(),
        };

        self.browsers.push(instance);
        Ok(id)
    }

    pub fn get_instance(&self, id: &str) -> Option<&BrowserInstance> {
        self.browsers.iter().find(|b| b.id == id)
    }

    pub fn list_instances(&self) -> Vec<BrowserInstance> {
        self.browsers.clone()
    }

    pub async fn navigate(&mut self, id: &str, url: &str) -> Result<()> {
        if let Some(browser) = self.browsers.iter_mut().find(|b| b.id == id) {
            browser.url = url.to_string();
            Ok(())
        } else {
            Err("Browser not found".into())
        }
    }

    pub async fn set_cookies(&mut self, id: &str, cookies: Vec<Cookie>) -> Result<()> {
        if let Some(browser) = self.browsers.iter_mut().find(|b| b.id == id) {
            browser.cookies = cookies;
            Ok(())
        } else {
            Err("Browser not found".into())
        }
    }
}

pub mod automation;
pub mod headless;
