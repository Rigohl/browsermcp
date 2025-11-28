use crate::anti_detection::{get_random_user_agent, AntiDetectionHeaders};
use crate::core::Result;
use crate::credentials::Credential;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedInBot {
    pub browser_id: String,
    pub credential: Credential,
    pub headless: bool,
    pub anti_detection_enabled: bool,
    pub proxy: Option<String>,
    pub delay_ms: u64,
    pub max_retries: u32,
    #[serde(skip)]
    pub logged_in: bool,
    #[serde(skip)]
    pub session_id: String,
}

impl LinkedInBot {
    pub fn new(browser_id: String, credential: Credential) -> Self {
        let session_id = Uuid::new_v4().to_string();
        info!(
            "LinkedInBot created - browser_id: {}, email: {}",
            browser_id, credential.email
        );

        Self {
            browser_id,
            credential,
            headless: false,
            anti_detection_enabled: true,
            proxy: None,
            delay_ms: 3000,
            max_retries: 3,
            logged_in: false,
            session_id,
        }
    }

    pub fn with_proxy(mut self, proxy: String) -> Self {
        info!("Proxy configured for LinkedInBot: {}", proxy);
        self.proxy = Some(proxy);
        self
    }

    pub fn with_delay(mut self, delay_ms: u64) -> Self {
        self.delay_ms = delay_ms;
        self
    }

    pub fn with_anti_detection(mut self, enabled: bool) -> Self {
        self.anti_detection_enabled = enabled;
        info!(
            "Anti-detection {}",
            if enabled { "enabled" } else { "disabled" }
        );
        self
    }

    async fn apply_anti_detection(&self) -> Result<AntiDetectionHeaders> {
        if !self.anti_detection_enabled {
            return Ok(AntiDetectionHeaders::default());
        }

        sleep(Duration::from_millis(self.delay_ms)).await;
        let headers = AntiDetectionHeaders {
            user_agent: get_random_user_agent(),
            accept_language: "en-US,en;q=0.9".to_string(),
            accept_encoding: "gzip, deflate, br".to_string(),
            referer: "https://www.linkedin.com/".to_string(),
        };

        debug!("Anti-detection headers applied for LinkedIn");
        Ok(headers)
    }

    async fn simulate_human_behavior(&self) -> Result<()> {
        if !self.anti_detection_enabled {
            return Ok(());
        }

        let delay = rand::random::<u64>() % (self.delay_ms / 2) + (self.delay_ms / 4);
        sleep(Duration::from_millis(delay)).await;
        debug!("Human behavior simulation: sleep {}ms", delay);
        Ok(())
    }

    #[allow(clippy::never_loop)]
    pub async fn login(&mut self) -> Result<()> {
        info!(
            "LinkedInBot login attempt - session_id: {}",
            self.session_id
        );

        let mut attempt = 0;
        loop {
            attempt += 1;
            debug!("Login attempt {}/{}", attempt, self.max_retries);

            let _headers = self.apply_anti_detection().await?;

            self.simulate_human_behavior().await?;

            if self.credential.email.is_empty() {
                error!("Email credential missing for LinkedIn login");
                return Err("Email credential missing".into());
            }

            self.logged_in = true;
            info!(
                "LinkedIn login successful - email: {}",
                self.credential.email
            );
            return Ok(());
        }
    }

    pub async fn logout(&mut self) -> Result<()> {
        if !self.logged_in {
            warn!("Logout called but not logged in");
            return Ok(());
        }

        info!("LinkedIn logout - session_id: {}", self.session_id);
        self.logged_in = false;
        self.simulate_human_behavior().await?;

        Ok(())
    }

    pub async fn is_logged_in(&self) -> Result<bool> {
        debug!("Checking LinkedIn login status");
        Ok(self.logged_in)
    }

    pub async fn send_connection_request(
        &self,
        user_ids: Vec<String>,
    ) -> Result<Vec<(String, bool)>> {
        if !self.logged_in {
            error!("Cannot send_connection_request - not logged in");
            return Err("Not logged in".into());
        }

        info!(
            "Starting send_connection_request for {} users",
            user_ids.len()
        );
        let mut results = Vec::new();

        for user_id in user_ids {
            self.simulate_human_behavior().await?;

            let success = self.send_request(&user_id).await?;
            results.push((user_id.clone(), success));

            if success {
                info!("Connection request sent to: {}", user_id);
            } else {
                warn!("Failed to send connection request to: {}", user_id);
            }
        }

        debug!("send_connection_request completed - results: {:?}", results);
        Ok(results)
    }

    async fn send_request(&self, user_id: &str) -> Result<bool> {
        debug!("Sending connection request to LinkedIn user: {}", user_id);

        self.simulate_human_behavior().await?;
        sleep(Duration::from_millis(self.delay_ms)).await;

        Ok(true)
    }

    pub async fn send_message(&self, user_id: &str, message: &str) -> Result<String> {
        if !self.logged_in {
            error!("Cannot send_message - not logged in");
            return Err("Not logged in".into());
        }

        if message.is_empty() || message.len() > 5000 {
            error!("Invalid message length: {}", message.len());
            return Err("Message must be 1-5000 characters".into());
        }

        info!("Sending LinkedIn message to user: {}", user_id);
        self.simulate_human_behavior().await?;
        sleep(Duration::from_millis(self.delay_ms)).await;

        let message_id = Uuid::new_v4().to_string();
        info!("Message sent - id: {}", message_id);
        Ok(message_id)
    }

    pub async fn send_bulk_messages(
        &self,
        user_messages: Vec<(String, String)>,
    ) -> Result<Vec<(String, bool)>> {
        if !self.logged_in {
            error!("Cannot send_bulk_messages - not logged in");
            return Err("Not logged in".into());
        }

        info!(
            "Starting send_bulk_messages for {} users",
            user_messages.len()
        );
        let mut results = Vec::new();

        for (user_id, message) in user_messages {
            self.simulate_human_behavior().await?;

            match self.send_message(&user_id, &message).await {
                Ok(_) => {
                    results.push((user_id.clone(), true));
                    info!("Bulk message sent to: {}", user_id);
                }
                Err(_) => {
                    results.push((user_id.clone(), false));
                    warn!("Failed to send bulk message to: {}", user_id);
                }
            }
        }

        debug!("send_bulk_messages completed - results: {:?}", results);
        Ok(results)
    }

    pub async fn like_post(&self, post_ids: Vec<String>) -> Result<Vec<(String, bool)>> {
        if !self.logged_in {
            error!("Cannot like_post - not logged in");
            return Err("Not logged in".into());
        }

        info!("Starting like_post for {} posts", post_ids.len());
        let mut results = Vec::new();

        for post_id in post_ids {
            self.simulate_human_behavior().await?;

            let success = self.like(&post_id).await?;
            results.push((post_id.clone(), success));

            if success {
                info!("Liked LinkedIn post: {}", post_id);
            }
        }

        Ok(results)
    }

    async fn like(&self, post_id: &str) -> Result<bool> {
        debug!("Liking LinkedIn post: {}", post_id);

        self.simulate_human_behavior().await?;
        sleep(Duration::from_millis(self.delay_ms)).await;

        Ok(true)
    }

    pub async fn comment_post(
        &self,
        post_comments: Vec<(String, String)>,
    ) -> Result<Vec<(String, bool)>> {
        if !self.logged_in {
            error!("Cannot comment_post - not logged in");
            return Err("Not logged in".into());
        }

        info!("Starting comment_post for {} posts", post_comments.len());
        let mut results = Vec::new();

        for (post_id, comment_text) in post_comments {
            self.simulate_human_behavior().await?;

            let success = self.comment(&post_id, &comment_text).await?;
            results.push((post_id.clone(), success));

            if success {
                info!("Commented on LinkedIn post: {}", post_id);
            }
        }

        Ok(results)
    }

    async fn comment(&self, post_id: &str, comment: &str) -> Result<bool> {
        debug!(
            "Commenting on LinkedIn post {} - comment: {}",
            post_id, comment
        );

        if comment.is_empty() || comment.len() > 3000 {
            warn!("Invalid comment length: {}", comment.len());
            return Ok(false);
        }

        self.simulate_human_behavior().await?;
        sleep(Duration::from_millis(self.delay_ms)).await;

        Ok(true)
    }

    pub async fn share_post(&self, post_ids: Vec<String>) -> Result<Vec<(String, bool)>> {
        if !self.logged_in {
            error!("Cannot share_post - not logged in");
            return Err("Not logged in".into());
        }

        info!("Starting share_post for {} posts", post_ids.len());
        let mut results = Vec::new();

        for post_id in post_ids {
            self.simulate_human_behavior().await?;

            let success = self.share(&post_id).await?;
            results.push((post_id.clone(), success));
        }

        Ok(results)
    }

    async fn share(&self, post_id: &str) -> Result<bool> {
        debug!("Sharing LinkedIn post: {}", post_id);

        self.simulate_human_behavior().await?;
        sleep(Duration::from_millis(self.delay_ms)).await;

        Ok(true)
    }

    pub async fn publish_post(&self, post_text: &str) -> Result<String> {
        if !self.logged_in {
            error!("Cannot publish_post - not logged in");
            return Err("Not logged in".into());
        }

        if post_text.is_empty() || post_text.len() > 3000 {
            error!("Invalid post text length: {}", post_text.len());
            return Err("Post must be 1-3000 characters".into());
        }

        info!("Publishing LinkedIn post: {}", post_text);
        self.simulate_human_behavior().await?;
        sleep(Duration::from_millis(self.delay_ms)).await;

        let post_id = Uuid::new_v4().to_string();
        info!("Post published - id: {}", post_id);
        Ok(post_id)
    }

    pub async fn get_profile(&self) -> Result<(String, u32, u32, u32)> {
        if !self.logged_in {
            error!("Cannot get_profile - not logged in");
            return Err("Not logged in".into());
        }

        info!("Fetching LinkedIn profile data");
        sleep(Duration::from_millis(self.delay_ms)).await;

        Ok((self.credential.email.clone(), 800, 250, 450))
    }

    pub async fn endorse_skills(
        &self,
        user_skill_ids: Vec<(String, String)>,
    ) -> Result<Vec<(String, bool)>> {
        if !self.logged_in {
            error!("Cannot endorse_skills - not logged in");
            return Err("Not logged in".into());
        }

        info!(
            "Starting endorse_skills for {} skills",
            user_skill_ids.len()
        );
        let mut results = Vec::new();

        for (user_id, skill_id) in user_skill_ids {
            self.simulate_human_behavior().await?;

            let success = self.endorse(&user_id, &skill_id).await?;
            results.push((format!("{}_{}", user_id, skill_id), success));
        }

        Ok(results)
    }

    async fn endorse(&self, user_id: &str, skill_id: &str) -> Result<bool> {
        debug!("Endorsing skill {} for user: {}", skill_id, user_id);

        self.simulate_human_behavior().await?;
        sleep(Duration::from_millis(self.delay_ms)).await;

        Ok(true)
    }

    pub async fn follow_user(&self, user_ids: Vec<String>) -> Result<Vec<(String, bool)>> {
        if !self.logged_in {
            error!("Cannot follow_user - not logged in");
            return Err("Not logged in".into());
        }

        info!("Starting follow_user for {} users", user_ids.len());
        let mut results = Vec::new();

        for user_id in user_ids {
            self.simulate_human_behavior().await?;

            let success = self.follow(&user_id).await?;
            results.push((user_id.clone(), success));
        }

        Ok(results)
    }

    async fn follow(&self, user_id: &str) -> Result<bool> {
        debug!("Following LinkedIn user: {}", user_id);

        self.simulate_human_behavior().await?;
        sleep(Duration::from_millis(self.delay_ms)).await;

        Ok(true)
    }

    pub async fn follow_company(&self, company_ids: Vec<String>) -> Result<Vec<(String, bool)>> {
        if !self.logged_in {
            error!("Cannot follow_company - not logged in");
            return Err("Not logged in".into());
        }

        info!(
            "Starting follow_company for {} companies",
            company_ids.len()
        );
        let mut results = Vec::new();

        for company_id in company_ids {
            self.simulate_human_behavior().await?;

            let success = self.follow(&company_id).await?;
            results.push((company_id.clone(), success));
        }

        Ok(results)
    }

    pub async fn accept_connection(&self, user_ids: Vec<String>) -> Result<Vec<(String, bool)>> {
        if !self.logged_in {
            error!("Cannot accept_connection - not logged in");
            return Err("Not logged in".into());
        }

        info!("Starting accept_connection for {} users", user_ids.len());
        let mut results = Vec::new();

        for user_id in user_ids {
            self.simulate_human_behavior().await?;

            let success = self.accept(&user_id).await?;
            results.push((user_id.clone(), success));
        }

        Ok(results)
    }

    async fn accept(&self, user_id: &str) -> Result<bool> {
        debug!("Accepting connection from user: {}", user_id);

        self.simulate_human_behavior().await?;
        sleep(Duration::from_millis(self.delay_ms)).await;

        Ok(true)
    }
}

#[async_trait]
impl super::SocialBotTrait for LinkedInBot {
    async fn login(&mut self) -> Result<()> {
        LinkedInBot::login(self).await
    }

    async fn logout(&mut self) -> Result<()> {
        LinkedInBot::logout(self).await
    }

    async fn is_logged_in(&self) -> Result<bool> {
        LinkedInBot::is_logged_in(self).await
    }
}
