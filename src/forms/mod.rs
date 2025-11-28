pub mod captcha_handler;
pub mod detection;
pub mod filling;
pub mod submission;

pub use captcha_handler::{BypassStrategy, CaptchaDetector, CaptchaHandler, CaptchaType};
pub use detection::{FieldType, FormDetector, FormElement, FormField};
pub use filling::{FieldMatcher, FormFiller, SmartFiller};
pub use submission::{FormStep, FormSubmitter, SubmissionConfig};

#[derive(Debug, Clone)]
pub struct FormConfig {
    pub enable_logging: bool,
    pub retry_attempts: u32,
    pub timeout_secs: u64,
    pub enable_js_execution: bool,
}

impl Default for FormConfig {
    fn default() -> Self {
        Self {
            enable_logging: true,
            retry_attempts: 3,
            timeout_secs: 30,
            enable_js_execution: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = FormConfig::default();
        assert_eq!(config.retry_attempts, 3);
        assert_eq!(config.timeout_secs, 30);
    }
}
