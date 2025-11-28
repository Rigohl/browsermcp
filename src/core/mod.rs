// Core module - Common types and utilities
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserInstance {
    pub id: String,
    pub url: String,
    pub user_agent: String,
    pub proxy: Option<String>,
    pub cookies: Vec<Cookie>,
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationData {
    pub email: String,
    pub password: String,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String,
    pub remember_me: Option<bool>,
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
