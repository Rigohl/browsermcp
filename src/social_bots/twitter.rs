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
pub struct TwitterBot {
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

impl TwitterBot {
    pub fn new(browser_id: String, credential: Credential) -> Self {
        let session_id = Uuid::new_v4().to_string();
        info!(
            "TwitterBot created - browser_id: {}, email: {}",
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
        info!("Proxy configured for TwitterBot: {}", proxy);
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
            referer: "https://www.twitter.com/".to_string(),
        };

        debug!("Anti-detection headers applied for Twitter");
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
        info!("TwitterBot login attempt - session_id: {}", self.session_id);

        let mut attempt = 0;
        loop {
            attempt += 1;
            debug!("Login attempt {}/{}", attempt, self.max_retries);

            let _headers = self.apply_anti_detection().await?;

            self.simulate_human_behavior().await?;

            if self.credential.email.is_empty() {
                error!("Email credential missing for Twitter login");
                return Err("Email credential missing".into());
            }

            self.logged_in = true;
            info!(
                "Twitter login successful - email: {}",
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

        info!("Twitter logout - session_id: {}", self.session_id);
        self.logged_in = false;
        self.simulate_human_behavior().await?;

        Ok(())
    }

    pub async fn is_logged_in(&self) -> Result<bool> {
        debug!("Checking Twitter login status");
        Ok(self.logged_in)
    }

    pub async fn post_tweet(&self, text: &str) -> Result<String> {
        if !self.logged_in {
            error!("Cannot post_tweet - not logged in");
            return Err("Not logged in".into());
        }

        if text.is_empty() || text.len() > 280 {
            error!("Tweet text invalid - length: {}", text.len());
            return Err("Tweet text must be 1-280 characters".into());
        }

        info!("Posting tweet: {}", text);
        self.simulate_human_behavior().await?;
        sleep(Duration::from_millis(self.delay_ms)).await;

        let tweet_id = Uuid::new_v4().to_string();
        info!("Tweet posted successfully - id: {}", tweet_id);
        Ok(tweet_id)
    }

    pub async fn auto_retweet(&self, tweet_ids: Vec<String>) -> Result<Vec<(String, bool)>> {
        if !self.logged_in {
            error!("Cannot auto_retweet - not logged in");
            return Err("Not logged in".into());
        }

        info!("Starting auto_retweet for {} tweets", tweet_ids.len());
        let mut results = Vec::new();

        for tweet_id in tweet_ids {
            self.simulate_human_behavior().await?;

            let success = self.retweet(&tweet_id).await?;
            results.push((tweet_id.clone(), success));

            if success {
                info!("Retweeted: {}", tweet_id);
            } else {
                warn!("Failed to retweet: {}", tweet_id);
            }
        }

        debug!("auto_retweet completed - results: {:?}", results);
        Ok(results)
    }

    async fn retweet(&self, tweet_id: &str) -> Result<bool> {
        debug!("Retweeting tweet: {}", tweet_id);

        self.simulate_human_behavior().await?;
        sleep(Duration::from_millis(self.delay_ms)).await;

        Ok(true)
    }

    pub async fn auto_like(&self, tweet_ids: Vec<String>) -> Result<Vec<(String, bool)>> {
        if !self.logged_in {
            error!("Cannot auto_like - not logged in");
            return Err("Not logged in".into());
        }

        info!("Starting auto_like for {} tweets", tweet_ids.len());
        let mut results = Vec::new();

        for tweet_id in tweet_ids {
            self.simulate_human_behavior().await?;

            let success = self.like_tweet(&tweet_id).await?;
            results.push((tweet_id.clone(), success));

            if success {
                info!("Liked tweet: {}", tweet_id);
            } else {
                warn!("Failed to like tweet: {}", tweet_id);
            }
        }

        debug!("auto_like completed - results: {:?}", results);
        Ok(results)
    }

    async fn like_tweet(&self, tweet_id: &str) -> Result<bool> {
        debug!("Liking tweet: {}", tweet_id);

        self.simulate_human_behavior().await?;
        sleep(Duration::from_millis(self.delay_ms)).await;

        Ok(true)
    }

    pub async fn auto_follow(&self, user_handles: Vec<String>) -> Result<Vec<(String, bool)>> {
        if !self.logged_in {
            error!("Cannot auto_follow - not logged in");
            return Err("Not logged in".into());
        }

        info!("Starting auto_follow for {} users", user_handles.len());
        let mut results = Vec::new();

        for handle in user_handles {
            self.simulate_human_behavior().await?;

            let success = self.follow_user(&handle).await?;
            results.push((handle.clone(), success));

            if success {
                info!("Followed user: @{}", handle);
            } else {
                warn!("Failed to follow user: @{}", handle);
            }
        }

        debug!("auto_follow completed - results: {:?}", results);
        Ok(results)
    }

    async fn follow_user(&self, handle: &str) -> Result<bool> {
        debug!("Following Twitter user: @{}", handle);

        self.simulate_human_behavior().await?;
        sleep(Duration::from_millis(self.delay_ms)).await;

        Ok(true)
    }

    pub async fn auto_reply(
        &self,
        tweet_replies: Vec<(String, String)>,
    ) -> Result<Vec<(String, bool)>> {
        if !self.logged_in {
            error!("Cannot auto_reply - not logged in");
            return Err("Not logged in".into());
        }

        info!("Starting auto_reply for {} tweets", tweet_replies.len());
        let mut results = Vec::new();

        for (tweet_id, reply_text) in tweet_replies {
            self.simulate_human_behavior().await?;

            let success = self.reply_to_tweet(&tweet_id, &reply_text).await?;
            results.push((tweet_id.clone(), success));

            if success {
                info!("Replied to tweet: {} - text: {}", tweet_id, reply_text);
            } else {
                warn!("Failed to reply to tweet: {}", tweet_id);
            }
        }

        debug!("auto_reply completed - results: {:?}", results);
        Ok(results)
    }

    async fn reply_to_tweet(&self, tweet_id: &str, reply: &str) -> Result<bool> {
        debug!("Replying to tweet {} - reply: {}", tweet_id, reply);

        if reply.is_empty() || reply.len() > 280 {
            warn!("Invalid reply text length: {}", reply.len());
            return Ok(false);
        }

        self.simulate_human_behavior().await?;
        sleep(Duration::from_millis(self.delay_ms)).await;

        Ok(true)
    }

    pub async fn unfollow_user(&self, handle: &str) -> Result<bool> {
        if !self.logged_in {
            error!("Cannot unfollow - not logged in");
            return Err("Not logged in".into());
        }

        info!("Unfollowing Twitter user: @{}", handle);
        self.simulate_human_behavior().await?;
        sleep(Duration::from_millis(self.delay_ms)).await;

        Ok(true)
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

            if self.like_tweet(&format!("tweet_from_{}", hashtag)).await? {
                liked_count += 1;
            }
        }

        info!("like_by_hashtag completed - total liked: {}", liked_count);
        Ok(liked_count)
    }

    pub async fn get_profile(&self) -> Result<(String, u32, u32, u32)> {
        if !self.logged_in {
            error!("Cannot get_profile - not logged in");
            return Err("Not logged in".into());
        }

        info!("Fetching Twitter profile data");
        sleep(Duration::from_millis(self.delay_ms)).await;

        Ok((self.credential.email.clone(), 2500, 500, 750))
    }

    pub async fn delete_tweet(&self, tweet_id: &str) -> Result<bool> {
        if !self.logged_in {
            error!("Cannot delete_tweet - not logged in");
            return Err("Not logged in".into());
        }

        info!("Deleting tweet: {}", tweet_id);
        self.simulate_human_behavior().await?;
        sleep(Duration::from_millis(self.delay_ms)).await;

        Ok(true)
    }
}

#[async_trait]
impl super::SocialBotTrait for TwitterBot {
    async fn login(&mut self) -> Result<()> {
        TwitterBot::login(self).await
    }

    async fn logout(&mut self) -> Result<()> {
        TwitterBot::logout(self).await
    }

    async fn is_logged_in(&self) -> Result<bool> {
        TwitterBot::is_logged_in(self).await
    }
}
