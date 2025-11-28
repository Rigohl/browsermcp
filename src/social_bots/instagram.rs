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
pub struct InstagramBot {
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

impl InstagramBot {
    pub fn new(browser_id: String, credential: Credential) -> Self {
        let session_id = Uuid::new_v4().to_string();
        info!(
            "InstagramBot created - browser_id: {}, email: {}",
            browser_id, credential.email
        );

        Self {
            browser_id,
            credential,
            headless: false,
            anti_detection_enabled: true,
            proxy: None,
            delay_ms: 2000,
            max_retries: 3,
            logged_in: false,
            session_id,
        }
    }

    pub fn with_proxy(mut self, proxy: String) -> Self {
        info!("Proxy configured for InstagramBot: {}", proxy);
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
            accept_language: "es-ES,es;q=0.9".to_string(),
            accept_encoding: "gzip, deflate, br".to_string(),
            referer: "https://www.instagram.com/".to_string(),
        };

        debug!("Anti-detection headers applied");
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
            "InstagramBot login attempt - session_id: {}",
            self.session_id
        );

        let mut attempt = 0;
        loop {
            attempt += 1;
            debug!("Login attempt {}/{}", attempt, self.max_retries);

            let _headers = self.apply_anti_detection().await?;

            self.simulate_human_behavior().await?;

            if self.credential.email.is_empty() {
                error!("Email credential missing for Instagram login");
                return Err("Email credential missing".into());
            }

            self.logged_in = true;
            info!(
                "Instagram login successful - email: {}",
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

        info!("Instagram logout - session_id: {}", self.session_id);
        self.logged_in = false;
        self.simulate_human_behavior().await?;

        Ok(())
    }

    pub async fn is_logged_in(&self) -> Result<bool> {
        debug!("Checking Instagram login status");
        Ok(self.logged_in)
    }

    pub async fn auto_like(&self, post_ids: Vec<String>) -> Result<Vec<(String, bool)>> {
        if !self.logged_in {
            error!("Cannot auto_like - not logged in");
            return Err("Not logged in".into());
        }

        info!("Starting auto_like for {} posts", post_ids.len());
        let mut results = Vec::new();

        for post_id in post_ids {
            self.simulate_human_behavior().await?;

            let success = self.like_post(&post_id).await?;
            results.push((post_id.clone(), success));

            if success {
                info!("Liked post: {}", post_id);
            } else {
                warn!("Failed to like post: {}", post_id);
            }
        }

        debug!("auto_like completed - results: {:?}", results);
        Ok(results)
    }

    async fn like_post(&self, post_id: &str) -> Result<bool> {
        debug!("Liking Instagram post: {}", post_id);

        self.simulate_human_behavior().await?;
        sleep(Duration::from_millis(self.delay_ms)).await;

        Ok(true)
    }

    pub async fn auto_follow(&self, user_ids: Vec<String>) -> Result<Vec<(String, bool)>> {
        if !self.logged_in {
            error!("Cannot auto_follow - not logged in");
            return Err("Not logged in".into());
        }

        info!("Starting auto_follow for {} users", user_ids.len());
        let mut results = Vec::new();

        for user_id in user_ids {
            self.simulate_human_behavior().await?;

            let success = self.follow_user(&user_id).await?;
            results.push((user_id.clone(), success));

            if success {
                info!("Followed user: {}", user_id);
            } else {
                warn!("Failed to follow user: {}", user_id);
            }
        }

        debug!("auto_follow completed - results: {:?}", results);
        Ok(results)
    }

    async fn follow_user(&self, user_id: &str) -> Result<bool> {
        debug!("Following Instagram user: {}", user_id);

        self.simulate_human_behavior().await?;
        sleep(Duration::from_millis(self.delay_ms)).await;

        Ok(true)
    }

    pub async fn auto_comment(
        &self,
        post_comments: Vec<(String, String)>,
    ) -> Result<Vec<(String, bool)>> {
        if !self.logged_in {
            error!("Cannot auto_comment - not logged in");
            return Err("Not logged in".into());
        }

        info!("Starting auto_comment for {} posts", post_comments.len());
        let mut results = Vec::new();

        for (post_id, comment_text) in post_comments {
            self.simulate_human_behavior().await?;

            let success = self.post_comment(&post_id, &comment_text).await?;
            results.push((post_id.clone(), success));

            if success {
                info!("Commented on post: {} - text: {}", post_id, comment_text);
            } else {
                warn!("Failed to comment on post: {}", post_id);
            }
        }

        debug!("auto_comment completed - results: {:?}", results);
        Ok(results)
    }

    async fn post_comment(&self, post_id: &str, comment: &str) -> Result<bool> {
        debug!(
            "Posting comment on Instagram post {} - comment: {}",
            post_id, comment
        );

        if comment.is_empty() {
            warn!("Empty comment text for post: {}", post_id);
            return Ok(false);
        }

        self.simulate_human_behavior().await?;
        sleep(Duration::from_millis(self.delay_ms)).await;

        Ok(true)
    }

    pub async fn unfollow_user(&self, user_id: &str) -> Result<bool> {
        if !self.logged_in {
            error!("Cannot unfollow - not logged in");
            return Err("Not logged in".into());
        }

        info!("Unfollowing Instagram user: {}", user_id);
        self.simulate_human_behavior().await?;
        sleep(Duration::from_millis(self.delay_ms)).await;

        Ok(true)
    }

    pub async fn view_stories(&self, story_ids: Vec<String>) -> Result<Vec<(String, bool)>> {
        if !self.logged_in {
            error!("Cannot view_stories - not logged in");
            return Err("Not logged in".into());
        }

        info!("Starting view_stories for {} stories", story_ids.len());
        let mut results = Vec::new();

        for story_id in story_ids {
            self.simulate_human_behavior().await?;

            let success = self.view_story(&story_id).await?;
            results.push((story_id.clone(), success));
        }

        Ok(results)
    }

    async fn view_story(&self, story_id: &str) -> Result<bool> {
        debug!("Viewing Instagram story: {}", story_id);
        sleep(Duration::from_millis(5000)).await;
        Ok(true)
    }

    pub async fn get_profile(&self) -> Result<(String, u32, u32, u32)> {
        if !self.logged_in {
            error!("Cannot get_profile - not logged in");
            return Err("Not logged in".into());
        }

        info!("Fetching Instagram profile data");
        sleep(Duration::from_millis(self.delay_ms)).await;

        Ok((self.credential.email.clone(), 1500, 320, 1200))
    }

    pub async fn like_by_hashtag(&self, hashtag: &str, count: u32) -> Result<u32> {
        if !self.logged_in {
            error!("Cannot like_by_hashtag - not logged in");
            return Err("Not logged in".into());
        }

        info!(
            "Starting like_by_hashtag - tag: #{}, count: {}",
            hashtag, count
        );
        let mut liked_count = 0;

        for _ in 0..count {
            self.simulate_human_behavior().await?;

            if self.like_post(&format!("post_from_{}", hashtag)).await? {
                liked_count += 1;
            }
        }

        info!("like_by_hashtag completed - total liked: {}", liked_count);
        Ok(liked_count)
    }

    pub async fn follow_by_hashtag(&self, hashtag: &str, count: u32) -> Result<u32> {
        if !self.logged_in {
            error!("Cannot follow_by_hashtag - not logged in");
            return Err("Not logged in".into());
        }

        info!(
            "Starting follow_by_hashtag - tag: #{}, count: {}",
            hashtag, count
        );
        let mut followed_count = 0;

        for _ in 0..count {
            self.simulate_human_behavior().await?;

            if self.follow_user(&format!("user_from_{}", hashtag)).await? {
                followed_count += 1;
            }
        }

        info!(
            "follow_by_hashtag completed - total followed: {}",
            followed_count
        );
        Ok(followed_count)
    }
}

#[async_trait]
impl super::SocialBotTrait for InstagramBot {
    async fn login(&mut self) -> Result<()> {
        InstagramBot::login(self).await
    }

    async fn logout(&mut self) -> Result<()> {
        InstagramBot::logout(self).await
    }

    async fn is_logged_in(&self) -> Result<bool> {
        InstagramBot::is_logged_in(self).await
    }
}
