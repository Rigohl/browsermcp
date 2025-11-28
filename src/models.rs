/// Data models for BrowserMCP
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserInstance {
    pub id: String,
    pub user_agent: String,
    pub cookies: HashMap<String, String>,
    pub active_account: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthToken {
    pub id: String,
    pub provider: String,
    pub access_token: String,
    pub expires_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub email: String,
    pub password: String,
    pub site: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserSession {
    pub id: String,
    pub browser_id: String,
    pub account_id: String,
    pub site: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapedData {
    pub url: String,
    pub status_code: u16,
    pub title: String,
    pub meta_tags: HashMap<String, String>,
    pub links: Vec<LinkData>,
    pub images: Vec<ImageData>,
    pub emails: Vec<String>,
    pub phones: Vec<String>,
    pub scripts: Vec<String>,
    pub stylesheets: Vec<String>,
    pub headings: Vec<HeadingData>,
    pub forms: Vec<FormData>,
    pub body_text: String,
    pub scraped_data: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkData {
    pub href: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageData {
    pub src: String,
    pub alt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeadingData {
    pub level: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormData {
    pub action: String,
    pub method: String,
    pub inputs: Vec<InputField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputField {
    pub name: String,
    pub field_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeAnalysis {
    pub language: String,
    pub lines_of_code: usize,
    pub functions: usize,
    pub imports: Vec<String>,
    pub complexity: f64,
    pub security_issues: usize,
    pub functions_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectAnalysis {
    pub path: String,
    pub total_files: usize,
    pub languages: Vec<String>,
    pub size_mb: f64,
    pub structure: HashMap<String, usize>,
}
