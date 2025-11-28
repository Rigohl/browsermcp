pub mod facebook;
pub mod instagram;
pub mod linkedin;
pub mod tiktok;
pub mod twitter;

pub use facebook::FacebookBot;
pub use instagram::InstagramBot;
pub use linkedin::LinkedInBot;
pub use tiktok::TikTokBot;
pub use twitter::TwitterBot;

use crate::core::Result;
use crate::credentials::Credential;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialBotConfig {
    pub browser_id: String,
    pub credential: Credential,
    pub headless: bool,
    pub anti_detection_enabled: bool,
    pub proxy: Option<String>,
    pub delay_ms: u64,
    pub max_retries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotActionResult {
    pub success: bool,
    pub action: String,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl BotActionResult {
    pub fn new(success: bool, action: &str, message: &str) -> Self {
        Self {
            success,
            action: action.to_string(),
            message: message.to_string(),
            timestamp: chrono::Utc::now(),
        }
    }
}

#[async_trait::async_trait]
pub trait SocialBotTrait {
    async fn login(&mut self) -> Result<()>;
    async fn logout(&mut self) -> Result<()>;
    async fn is_logged_in(&self) -> Result<bool>;
}
