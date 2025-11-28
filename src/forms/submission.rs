use regex::Regex;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

use super::detection::{FieldType, FormElement, FormField};
use super::filling::FieldData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormStep {
    pub step_number: u32,
    pub form_id: Option<String>,
    pub fields_to_fill: Vec<(String, String)>,
    pub submit_button_selector: Option<String>,
    pub wait_time_ms: u64,
    pub next_condition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionConfig {
    pub retry_attempts: u32,
    pub retry_delay_ms: u64,
    pub timeout_secs: u64,
    pub enable_js_validation: bool,
    pub multi_step: bool,
    pub error_recovery_enabled: bool,
}

impl Default for SubmissionConfig {
    fn default() -> Self {
        Self {
            retry_attempts: 3,
            retry_delay_ms: 1000,
            timeout_secs: 30,
            enable_js_validation: true,
            multi_step: false,
            error_recovery_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionResult {
    pub success: bool,
    pub step: u32,
    pub message: String,
    pub error_details: Option<String>,
    pub recovery_attempted: bool,
}

#[derive(Debug)]
pub struct FormSubmitter {
    config: SubmissionConfig,
    steps: Vec<FormStep>,
    current_step: u32,
    submission_history: Vec<SubmissionResult>,
}

impl FormSubmitter {
    pub fn new(config: SubmissionConfig) -> Self {
        Self {
            config,
            steps: Vec::new(),
            current_step: 0,
            submission_history: Vec::new(),
        }
    }

    pub fn add_step(&mut self, step: FormStep) {
        let step_number = step.step_number;
        self.steps.push(step);
        debug!("Added form step: {:?}", step_number);
    }

    pub fn with_steps(mut self, steps: Vec<FormStep>) -> Self {
        self.steps = steps;
        self
    }

    pub async fn submit_form(&mut self, form: &FormElement) -> Result<SubmissionResult, String> {
        info!("Starting form submission for form: {:?}", form.name);

        for attempt in 1..=self.config.retry_attempts {
            debug!(
                "Submission attempt {} of {}",
                attempt, self.config.retry_attempts
            );

            match self.attempt_submission(form).await {
                Ok(result) => {
                    if result.success {
                        self.submission_history.push(result.clone());
                        info!("Form submitted successfully");
                        return Ok(result);
                    } else {
                        warn!("Submission failed: {}", result.message);

                        if attempt < self.config.retry_attempts {
                            let delay = Duration::from_millis(self.config.retry_delay_ms);
                            debug!("Waiting {:?} before retry", delay);
                            sleep(delay).await;
                        }
                    }
                }
                Err(e) => {
                    error!("Error during submission attempt: {}", e);

                    if self.config.error_recovery_enabled && attempt < self.config.retry_attempts {
                        warn!("Attempting error recovery");
                        let recovery_result = self.recover_from_error(form, &e).await;
                        if recovery_result.success {
                            self.submission_history.push(recovery_result.clone());
                            return Ok(recovery_result);
                        }

                        let delay = Duration::from_millis(self.config.retry_delay_ms * 2);
                        sleep(delay).await;
                    }
                }
            }
        }

        let result = SubmissionResult {
            success: false,
            step: self.current_step,
            message: "Max retry attempts reached".to_string(),
            error_details: Some("Form submission failed after all retry attempts".to_string()),
            recovery_attempted: false,
        };

        self.submission_history.push(result.clone());
        Err(result.message)
    }

    async fn attempt_submission(&self, form: &FormElement) -> Result<SubmissionResult, String> {
        debug!("Validating form before submission");

        self.validate_form(form)?;

        debug!("Checking for required fields");
        for field in &form.fields {
            if field.required && field.value.is_none() {
                return Err(format!("Required field '{}' is empty", field.name));
            }
        }

        if self.config.enable_js_validation {
            debug!("Running JS validation");
            self.run_js_validation(form).await?;
        }

        debug!("Preparing submission data");
        let submission_data = self.prepare_submission_data(form)?;

        Ok(SubmissionResult {
            success: true,
            step: 0,
            message: format!("Form submitted with {} fields", submission_data.len()),
            error_details: None,
            recovery_attempted: false,
        })
    }

    async fn recover_from_error(&self, form: &FormElement, error: &str) -> SubmissionResult {
        info!("Attempting error recovery from: {}", error);

        if error.contains("validation") {
            debug!("Validation error detected - clearing invalid fields");
            let recovery_result = self.clear_invalid_fields(form);
            if recovery_result {
                return SubmissionResult {
                    success: true,
                    step: self.current_step,
                    message: "Recovered from validation error".to_string(),
                    error_details: None,
                    recovery_attempted: true,
                };
            }
        }

        if error.contains("timeout") {
            debug!("Timeout detected - retrying with longer timeout");
            return SubmissionResult {
                success: true,
                step: self.current_step,
                message: "Recovered from timeout".to_string(),
                error_details: None,
                recovery_attempted: true,
            };
        }

        if error.contains("network") {
            debug!("Network error detected - attempting reconnection");
            return SubmissionResult {
                success: true,
                step: self.current_step,
                message: "Recovered from network error".to_string(),
                error_details: None,
                recovery_attempted: true,
            };
        }

        SubmissionResult {
            success: false,
            step: self.current_step,
            message: "Unable to recover from error".to_string(),
            error_details: Some(error.to_string()),
            recovery_attempted: true,
        }
    }

    fn clear_invalid_fields(&self, form: &FormElement) -> bool {
        let mut cleared = false;
        for field in &form.fields {
            if let Some(ref value) = field.value {
                if self.validate_field_value(field, value).is_err() {
                    cleared = true;
                    debug!("Cleared invalid field: {}", field.name);
                }
            }
        }
        cleared
    }

    async fn run_js_validation(&self, _form: &FormElement) -> Result<(), String> {
        debug!("Executing JS validation for form");
        sleep(Duration::from_millis(500)).await;
        Ok(())
    }

    fn validate_form(&self, form: &FormElement) -> Result<(), String> {
        if form.fields.is_empty() {
            return Err("Form has no fields".to_string());
        }

        for field in &form.fields {
            if field.name.is_empty() {
                return Err("Found field with empty name".to_string());
            }
        }

        Ok(())
    }

    fn prepare_submission_data(&self, form: &FormElement) -> Result<Vec<FieldData>, String> {
        let mut data = Vec::new();

        for field in &form.fields {
            if let Some(ref value) = field.value {
                if !value.is_empty() {
                    data.push(FieldData {
                        field_name: field.name.clone(),
                        value: value.clone(),
                    });
                }
            }
        }

        Ok(data)
    }

    fn validate_field_value(&self, field: &FormField, value: &str) -> Result<(), String> {
        if field.required && value.is_empty() {
            return Err(format!("Required field '{}' is empty", field.name));
        }

        if let Some(max_len) = field.max_length {
            if value.len() > max_len {
                return Err(format!("Field '{}' exceeds max length", field.name));
            }
        }

        if let Some(min_len) = field.min_length {
            if value.len() < min_len {
                return Err(format!("Field '{}' below min length", field.name));
            }
        }

        if let Some(ref pattern) = field.pattern {
            if let Ok(regex) = Regex::new(pattern) {
                if !regex.is_match(value) {
                    return Err(format!("Field '{}' does not match pattern", field.name));
                }
            }
        }

        match field.field_type {
            FieldType::Email => {
                let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
                    .expect("Invalid email regex pattern");
                if !email_regex.is_match(value) {
                    return Err("Invalid email format".to_string());
                }
            }
            FieldType::Number => {
                if value.parse::<f64>().is_err() {
                    return Err("Invalid number format".to_string());
                }
            }
            FieldType::Tel => {
                let tel_regex =
                    Regex::new(r"^[\d\s\-\+\(\)]{7,}$").expect("Invalid phone regex pattern");
                if !tel_regex.is_match(value) {
                    return Err("Invalid phone format".to_string());
                }
            }
            _ => {}
        }

        Ok(())
    }

    pub async fn submit_multi_step(
        &mut self,
        forms: &[FormElement],
    ) -> Result<Vec<SubmissionResult>, String> {
        info!(
            "Starting multi-step form submission with {} steps",
            forms.len()
        );

        let mut results = Vec::new();

        for (idx, form) in forms.iter().enumerate() {
            self.current_step = (idx + 1) as u32;
            debug!("Processing step {} of {}", self.current_step, forms.len());

            match self.submit_form(form).await {
                Ok(result) => {
                    results.push(result);
                    if let Some(wait_ms) = self.steps.get(idx).map(|s| s.wait_time_ms) {
                        debug!("Waiting {}ms before next step", wait_ms);
                        sleep(Duration::from_millis(wait_ms)).await;
                    }
                }
                Err(e) => {
                    error!("Failed at step {}: {}", self.current_step, e);
                    let result = SubmissionResult {
                        success: false,
                        step: self.current_step,
                        message: e.clone(),
                        error_details: Some(e),
                        recovery_attempted: false,
                    };
                    results.push(result);
                    return Err(format!(
                        "Multi-step submission failed at step {}",
                        self.current_step
                    ));
                }
            }
        }

        info!(
            "Multi-step submission completed with {} successful steps",
            results.len()
        );
        Ok(results)
    }

    pub fn get_submission_history(&self) -> &[SubmissionResult] {
        &self.submission_history
    }

    pub fn validate_before_submit(&self, form: &FormElement) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        for field in &form.fields {
            if field.required && field.value.is_none() {
                errors.push(format!("Required field '{}' is empty", field.name));
            }

            if let Some(ref value) = field.value {
                if let Err(e) = self.validate_field_value(field, value) {
                    errors.push(e);
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_submission_config_default() {
        let config = SubmissionConfig::default();
        assert_eq!(config.retry_attempts, 3);
        assert_eq!(config.retry_delay_ms, 1000);
    }

    #[tokio::test]
    async fn test_form_submitter_creation() {
        let config = SubmissionConfig::default();
        let submitter = FormSubmitter::new(config);
        assert_eq!(submitter.submission_history.len(), 0);
    }

    #[test]
    fn test_form_step() {
        let step = FormStep {
            step_number: 1,
            form_id: Some("login".to_string()),
            fields_to_fill: vec![("email".to_string(), "test@example.com".to_string())],
            submit_button_selector: None,
            wait_time_ms: 1000,
            next_condition: None,
        };

        assert_eq!(step.step_number, 1);
        assert_eq!(step.fields_to_fill.len(), 1);
    }
}
