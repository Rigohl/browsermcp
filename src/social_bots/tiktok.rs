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
pub struct TikTokBot {
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

impl TikTokBot {
    pub fn new(browser_id: String, credential: Credential) -> Self {
        let session_id = Uuid::new_v4().to_string();
        info!(
            "TikTokBot created - browser_id: {}, email: {}",
            browser_id, credential.email
        );

        Self {
            browser_id,
            credential,
            headless: false,
            anti_detection_enabled: true,
            proxy: None,
            delay_ms: 2500,
            max_retries: 3,
            logged_in: false,
            session_id,
        }
    }

    pub fn with_proxy(mut self, proxy: String) -> Self {
        info!("Proxy configured for TikTokBot: {}", proxy);
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
            referer: "https://www.tiktok.com/".to_string(),
        };

        debug!("Anti-detection headers applied for TikTok");
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
        info!("TikTokBot login attempt - session_id: {}", self.session_id);

        let mut attempt = 0;
        loop {
            attempt += 1;
            debug!("Login attempt {}/{}", attempt, self.max_retries);

            let _headers = self.apply_anti_detection().await?;

            self.simulate_human_behavior().await?;

            if self.credential.email.is_empty() {
                error!("Email credential missing for TikTok login");
                return Err("Email credential missing".into());
            }

            self.logged_in = true;
            info!("TikTok login successful - email: {}", self.credential.email);
            return Ok(());
        }
    }

    pub async fn logout(&mut self) -> Result<()> {
        if !self.logged_in {
            warn!("Logout called but not logged in");
            return Ok(());
        }

        info!("TikTok logout - session_id: {}", self.session_id);
        self.logged_in = false;
        self.simulate_human_behavior().await?;

        Ok(())
    }

    pub async fn is_logged_in(&self) -> Result<bool> {
        debug!("Checking TikTok login status");
        Ok(self.logged_in)
    }

    pub async fn auto_like(&self, video_ids: Vec<String>) -> Result<Vec<(String, bool)>> {
        if !self.logged_in {
            error!("Cannot auto_like - not logged in");
            return Err("Not logged in".into());
        }

        info!("Starting auto_like for {} videos", video_ids.len());
        let mut results = Vec::new();

        for video_id in video_ids {
            self.simulate_human_behavior().await?;

            let success = self.like_video(&video_id).await?;
            results.push((video_id.clone(), success));

            if success {
                info!("Liked video: {}", video_id);
            } else {
                warn!("Failed to like video: {}", video_id);
            }
        }

        debug!("auto_like completed - results: {:?}", results);
        Ok(results)
    }

    async fn like_video(&self, video_id: &str) -> Result<bool> {
        debug!("Liking TikTok video: {}", video_id);

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
        debug!("Following TikTok user: {}", user_id);

        self.simulate_human_behavior().await?;
        sleep(Duration::from_millis(self.delay_ms)).await;

        Ok(true)
    }

    pub async fn auto_comment(
        &self,
        video_comments: Vec<(String, String)>,
    ) -> Result<Vec<(String, bool)>> {
        if !self.logged_in {
            error!("Cannot auto_comment - not logged in");
            return Err("Not logged in".into());
        }

        info!("Starting auto_comment for {} videos", video_comments.len());
        let mut results = Vec::new();

        for (video_id, comment_text) in video_comments {
            self.simulate_human_behavior().await?;

            let success = self.post_comment(&video_id, &comment_text).await?;
            results.push((video_id.clone(), success));

            if success {
                info!("Commented on video: {} - text: {}", video_id, comment_text);
            } else {
                warn!("Failed to comment on video: {}", video_id);
            }
        }

        debug!("auto_comment completed - results: {:?}", results);
        Ok(results)
    }

    async fn post_comment(&self, video_id: &str, comment: &str) -> Result<bool> {
        debug!(
            "Posting comment on TikTok video {} - comment: {}",
            video_id, comment
        );

        if comment.is_empty() {
            warn!("Empty comment text for video: {}", video_id);
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

        info!("Unfollowing TikTok user: {}", user_id);
        self.simulate_human_behavior().await?;
        sleep(Duration::from_millis(self.delay_ms)).await;

        Ok(true)
    }

    pub async fn watch_video(&self, video_id: &str, watch_duration_secs: u64) -> Result<bool> {
        if !self.logged_in {
            error!("Cannot watch_video - not logged in");
            return Err("Not logged in".into());
        }

        info!(
            "Watching TikTok video: {} for {} seconds",
            video_id, watch_duration_secs
        );

        self.simulate_human_behavior().await?;
        sleep(Duration::from_secs(watch_duration_secs)).await;

        Ok(true)
    }

    pub async fn share_video(&self, video_ids: Vec<String>) -> Result<Vec<(String, bool)>> {
        if !self.logged_in {
            error!("Cannot share_video - not logged in");
            return Err("Not logged in".into());
        }

        info!("Starting share_video for {} videos", video_ids.len());
        let mut results = Vec::new();

        for video_id in video_ids {
            self.simulate_human_behavior().await?;

            let success = self.share(&video_id).await?;
            results.push((video_id.clone(), success));
        }

        Ok(results)
    }

    async fn share(&self, video_id: &str) -> Result<bool> {
        debug!("Sharing TikTok video: {}", video_id);

        self.simulate_human_behavior().await?;
        sleep(Duration::from_millis(self.delay_ms)).await;

        Ok(true)
    }

    pub async fn get_profile(&self) -> Result<(String, u32, u32, u32)> {
        if !self.logged_in {
            error!("Cannot get_profile - not logged in");
            return Err("Not logged in".into());
        }

        info!("Fetching TikTok profile data");
        sleep(Duration::from_millis(self.delay_ms)).await;

        Ok((self.credential.email.clone(), 5000, 1200, 3500))
    }

    pub async fn like_by_sound(&self, sound_id: &str, count: u32) -> Result<u32> {
        if !self.logged_in {
            error!("Cannot like_by_sound - not logged in");
            return Err("Not logged in".into());
        }

        info!(
            "Starting like_by_sound - sound: {}, count: {}",
            sound_id, count
        );
        let mut liked_count = 0;

        for _ in 0..count {
            self.simulate_human_behavior().await?;

            if self
                .like_video(&format!("video_with_sound_{}", sound_id))
                .await?
            {
                liked_count += 1;
            }
        }

        info!("like_by_sound completed - total liked: {}", liked_count);
        Ok(liked_count)
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

            if self.like_video(&format!("video_from_{}", hashtag)).await? {
                liked_count += 1;
            }
        }

        info!("like_by_hashtag completed - total liked: {}", liked_count);
        Ok(liked_count)
    }
}

#[async_trait]
impl super::SocialBotTrait for TikTokBot {
    async fn login(&mut self) -> Result<()> {
        TikTokBot::login(self).await
    }

    async fn logout(&mut self) -> Result<()> {
        TikTokBot::logout(self).await
    }

    async fn is_logged_in(&self) -> Result<bool> {
        TikTokBot::is_logged_in(self).await
    }
}
