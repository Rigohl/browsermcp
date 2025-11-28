use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, error, info};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CaptchaType {
    ReCaptchaV2,
    ReCaptchaV3,
    HCaptcha,
    ImageCaptcha,
    AudioCaptcha,
    FunCaptcha,
    GeeTest,
    Cloudflare,
    AwsCaptcha,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptchaDetectionResult {
    pub detected: bool,
    pub captcha_type: CaptchaType,
    pub element_id: Option<String>,
    pub element_class: Option<String>,
    pub iframe_src: Option<String>,
    pub confidence: f32,
    pub location: Option<(u32, u32)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BypassStrategy {
    AudioTranscription,
    ImageProcessing,
    ThirdPartyService,
    BrowserAutomation,
    TokenInjection,
    ProxyRotation,
    UserInteraction,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptchaConfig {
    pub strategy: BypassStrategy,
    pub third_party_api_key: Option<String>,
    pub max_attempts: u32,
    pub timeout_secs: u64,
    pub enable_audio_fallback: bool,
    pub enable_image_processing: bool,
}

impl Default for CaptchaConfig {
    fn default() -> Self {
        Self {
            strategy: BypassStrategy::None,
            third_party_api_key: None,
            max_attempts: 5,
            timeout_secs: 60,
            enable_audio_fallback: true,
            enable_image_processing: true,
        }
    }
}

#[derive(Debug)]
pub struct CaptchaDetector {
    #[allow(dead_code)]
    recaptcha_v2_patterns: Vec<Regex>,
    #[allow(dead_code)]
    recaptcha_v3_patterns: Vec<Regex>,
    #[allow(dead_code)]
    hcaptcha_patterns: Vec<Regex>,
    #[allow(dead_code)]
    image_captcha_patterns: Vec<Regex>,
    #[allow(dead_code)]
    audio_captcha_patterns: Vec<Regex>,
    #[allow(dead_code)]
    funcaptcha_patterns: Vec<Regex>,
    #[allow(dead_code)]
    geetest_patterns: Vec<Regex>,
    #[allow(dead_code)]
    cloudflare_patterns: Vec<Regex>,
}

impl CaptchaDetector {
    pub fn new() -> Self {
        Self {
            recaptcha_v2_patterns: vec![
                Regex::new(r#"class="g-recaptcha"#).unwrap(),
                Regex::new(r#"data-sitekey"#).unwrap(),
            ],
            recaptcha_v3_patterns: vec![
                Regex::new(r#"grecaptcha\.ready"#).unwrap(),
                Regex::new(r#"grecaptcha\.execute"#).unwrap(),
            ],
            hcaptcha_patterns: vec![
                Regex::new(r#"class="h-captcha"#).unwrap(),
                Regex::new(r#"hcaptcha\.render"#).unwrap(),
            ],
            image_captcha_patterns: vec![Regex::new(r#"captcha.*image"#).unwrap()],
            audio_captcha_patterns: vec![Regex::new(r#"captcha.*audio"#).unwrap()],
            funcaptcha_patterns: vec![Regex::new(r#"class="funcaptcha"#).unwrap()],
            geetest_patterns: vec![Regex::new(r#"geetest"#).unwrap()],
            cloudflare_patterns: vec![Regex::new(r#"__cf_chl"#).unwrap()],
        }
    }

    pub fn detect(&self, html: &str) -> CaptchaDetectionResult {
        debug!("Scanning HTML for CAPTCHA detection");

        if self.matches_patterns(&self.recaptcha_v2_patterns, html) {
            info!("Detected reCAPTCHA v2");
            return CaptchaDetectionResult {
                detected: true,
                captcha_type: CaptchaType::ReCaptchaV2,
                element_id: None,
                element_class: Some("g-recaptcha".to_string()),
                iframe_src: None,
                confidence: 0.95,
                location: None,
            };
        }

        CaptchaDetectionResult {
            detected: false,
            captcha_type: CaptchaType::Unknown,
            element_id: None,
            element_class: None,
            iframe_src: None,
            confidence: 0.0,
            location: None,
        }
    }

    fn matches_patterns(&self, patterns: &[Regex], text: &str) -> bool {
        patterns.iter().any(|p| p.is_match(text))
    }
}

impl Default for CaptchaDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct CaptchaHandler {
    detector: CaptchaDetector,
    config: CaptchaConfig,
    attempt_count: u32,
    #[allow(dead_code)]
    token_cache: HashMap<String, String>,
}

impl CaptchaHandler {
    pub fn new(config: CaptchaConfig) -> Self {
        Self {
            detector: CaptchaDetector::new(),
            config,
            attempt_count: 0,
            token_cache: HashMap::new(),
        }
    }

    pub async fn handle_captcha(&mut self, html: &str) -> Result<Option<String>, String> {
        debug!("Starting CAPTCHA handling process");

        let detection = self.detector.detect(html);

        if !detection.detected {
            debug!("No CAPTCHA detected");
            return Ok(None);
        }

        info!("CAPTCHA detected: {:?}", detection.captcha_type);

        if self.attempt_count >= self.config.max_attempts {
            error!("Max CAPTCHA attempts exceeded");
            return Err("Max CAPTCHA bypass attempts exceeded".to_string());
        }

        self.attempt_count += 1;
        Ok(None)
    }
}
