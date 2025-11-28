use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tracing::{debug, error, span, Level};

#[derive(Error, Debug)]
pub enum ExtractionError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Pattern matching error: {0}")]
    PatternError(String),

    #[error("Type conversion error: {0}")]
    ConversionError(String),

    #[error("Missing required field: {0}")]
    MissingField(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    String,
    Integer,
    Float,
    Boolean,
    DateTime,
    Array(Box<DataType>),
    Object,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldRule {
    pub name: String,
    pub selector: String,
    pub attribute: Option<String>,
    pub data_type: DataType,
    pub required: bool,
    pub default_value: Option<Value>,
    pub transformations: Vec<String>,
    pub pattern: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionSchema {
    pub name: String,
    pub rules: Vec<FieldRule>,
    pub root_selector: Option<String>,
    pub multiple: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractorConfig {
    pub strict_mode: bool,
    pub case_sensitive: bool,
    pub trim_whitespace: bool,
    pub validate_patterns: bool,
    pub max_array_size: Option<usize>,
}

impl Default for ExtractorConfig {
    fn default() -> Self {
        Self {
            strict_mode: true,
            case_sensitive: false,
            trim_whitespace: true,
            validate_patterns: true,
            max_array_size: Some(1000),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionResult {
    pub data: Value,
    pub extracted_at: String,
    pub schema_name: String,
    pub fields_extracted: usize,
    pub fields_failed: usize,
    pub success_rate: f64,
}

#[derive(Debug)]
pub struct DataExtractor {
    config: ExtractorConfig,
    regex_cache: Arc<std::sync::Mutex<HashMap<String, Regex>>>,
}

impl DataExtractor {
    /// Creates a new DataExtractor with default configuration
    ///
    /// # Example
    /// ```ignore
    /// let extractor = DataExtractor::new();
    /// ```
    pub fn new() -> Self {
        Self {
            config: ExtractorConfig::default(),
            regex_cache: Arc::new(std::sync::Mutex::new(HashMap::new())),
        }
    }

    /// Creates a new DataExtractor with custom configuration
    ///
    /// # Example
    /// ```ignore
    /// let config = ExtractorConfig {
    ///     strict_mode: false,
    ///     case_sensitive: true,
    ///     trim_whitespace: true,
    ///     validate_patterns: true,
    ///     max_array_size: Some(500),
    /// };
    /// let extractor = DataExtractor::with_config(config);
    /// ```
    pub fn with_config(config: ExtractorConfig) -> Self {
        Self {
            config,
            regex_cache: Arc::new(std::sync::Mutex::new(HashMap::new())),
        }
    }

    /// Extracts data using a single schema
    ///
    /// # Example
    /// ```ignore
    /// let schema = ExtractionSchema {
    ///     name: "article".to_string(),
    ///     rules: vec![/* rules */],
    ///     root_selector: None,
    ///     multiple: false,
    /// };
    /// let result = extractor.extract(html, &schema)?;
    /// ```
    pub fn extract(
        &self,
        data: &str,
        schema: &ExtractionSchema,
    ) -> Result<ExtractionResult, ExtractionError> {
        let span = span!(Level::DEBUG, "extract", schema = schema.name);
        let _guard = span.enter();

        debug!("Extracting data with schema: {}", schema.name);

        let mut extracted = serde_json::Map::new();
        let mut success_count = 0;
        let mut fail_count = 0;

        for rule in &schema.rules {
            match self.extract_field(data, rule) {
                Ok(value) => {
                    extracted.insert(rule.name.clone(), value);
                    success_count += 1;
                    debug!("Successfully extracted field: {}", rule.name);
                }
                Err(e) => {
                    fail_count += 1;
                    if rule.required && self.config.strict_mode {
                        error!("Failed to extract required field {}: {}", rule.name, e);
                        return Err(e);
                    }
                    if let Some(default) = &rule.default_value {
                        extracted.insert(rule.name.clone(), default.clone());
                        debug!("Using default value for field: {}", rule.name);
                    } else {
                        debug!("Field {} not extracted and no default value", rule.name);
                    }
                }
            }
        }

        let total = success_count + fail_count;
        let success_rate = if total > 0 {
            (success_count as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        Ok(ExtractionResult {
            data: Value::Object(extracted),
            extracted_at: chrono::Utc::now().to_rfc3339(),
            schema_name: schema.name.clone(),
            fields_extracted: success_count,
            fields_failed: fail_count,
            success_rate,
        })
    }

    /// Extracts multiple items using a schema
    ///
    /// # Example
    /// ```ignore
    /// let schema = ExtractionSchema {
    ///     name: "items".to_string(),
    ///     rules: vec![/* rules */],
    ///     root_selector: Some(".item"),
    ///     multiple: true,
    /// };
    /// let results = extractor.extract_multiple(html, &schema)?;
    /// ```
    pub fn extract_multiple(
        &self,
        data: &str,
        schema: &ExtractionSchema,
    ) -> Result<Vec<ExtractionResult>, ExtractionError> {
        let span = span!(Level::DEBUG, "extract_multiple", schema = schema.name);
        let _guard = span.enter();

        if !schema.multiple {
            return Err(ExtractionError::ValidationError(
                "Schema is not configured for multiple extraction".to_string(),
            ));
        }

        debug!("Extracting multiple items with schema: {}", schema.name);

        let mut results = Vec::new();

        // Simplified: would need full parser integration
        // For now, extracting single result
        results.push(self.extract(data, schema)?);

        if let Some(max_size) = self.config.max_array_size {
            if results.len() > max_size {
                debug!("Truncating results from {} to {}", results.len(), max_size);
                results.truncate(max_size);
            }
        }

        Ok(results)
    }

    /// Extracts a single field value
    pub fn extract_field(&self, data: &str, rule: &FieldRule) -> Result<Value, ExtractionError> {
        let span = span!(Level::DEBUG, "extract_field", field = rule.name);
        let _guard = span.enter();

        debug!(
            "Extracting field: {} with selector: {}",
            rule.name, rule.selector
        );

        // Simulate field extraction (would use actual DOM parser)
        let mut value = self.extract_raw_value(data, rule)?;

        // Apply transformations
        for transformation in &rule.transformations {
            value = self.apply_transformation(&value, transformation)?;
        }

        // Validate pattern if specified
        if let Some(pattern) = &rule.pattern {
            if self.config.validate_patterns {
                self.validate_pattern(&value, pattern)?;
            }
        }

        // Convert to target data type
        value = self.convert_type(&value, &rule.data_type)?;

        Ok(value)
    }

    /// Extracts raw value from data
    fn extract_raw_value(&self, _data: &str, rule: &FieldRule) -> Result<Value, ExtractionError> {
        // Placeholder for actual DOM extraction
        // In production, would use the DOM parser
        Ok(Value::String(format!("extracted_{}", rule.name)))
    }

    /// Applies a transformation to a value
    fn apply_transformation(
        &self,
        value: &Value,
        transformation: &str,
    ) -> Result<Value, ExtractionError> {
        let span = span!(Level::DEBUG, "apply_transformation", transformation);
        let _guard = span.enter();

        match transformation {
            "trim" => Ok(Value::String(
                value.as_str().unwrap_or("").trim().to_string(),
            )),
            "lowercase" => Ok(Value::String(value.as_str().unwrap_or("").to_lowercase())),
            "uppercase" => Ok(Value::String(value.as_str().unwrap_or("").to_uppercase())),
            "remove_whitespace" => Ok(Value::String(value.as_str().unwrap_or("").replace(" ", ""))),
            t if t.starts_with("regex:") => {
                let pattern = &t[6..];
                self.apply_regex(value, pattern)
            }
            t if t.starts_with("replace:") => {
                let parts: Vec<&str> = t[8..].split(":").collect();
                if parts.len() >= 2 {
                    let search = parts[0];
                    let replace = parts[1];
                    Ok(Value::String(
                        value.as_str().unwrap_or("").replace(search, replace),
                    ))
                } else {
                    Err(ExtractionError::PatternError(format!(
                        "Invalid replace transformation: {}",
                        t
                    )))
                }
            }
            _ => {
                error!("Unknown transformation: {}", transformation);
                Err(ExtractionError::PatternError(format!(
                    "Unknown transformation: {}",
                    transformation
                )))
            }
        }
    }

    /// Applies regex transformation
    fn apply_regex(&self, value: &Value, pattern: &str) -> Result<Value, ExtractionError> {
        let mut cache = self.regex_cache.lock().map_err(|_| {
            ExtractionError::PatternError("Failed to acquire regex cache lock".to_string())
        })?;

        let regex = cache.entry(pattern.to_string()).or_insert_with(|| {
            Regex::new(pattern).unwrap_or_else(|_| {
                Regex::new(".*").expect("Fallback regex pattern '.*' should always be valid")
            })
        });

        let text = value.as_str().unwrap_or("");

        if let Some(caps) = regex.captures(text) {
            if caps.len() > 1 {
                Ok(Value::String(caps[1].to_string()))
            } else {
                Ok(Value::String(caps[0].to_string()))
            }
        } else {
            Ok(Value::Null)
        }
    }

    /// Validates value against pattern
    fn validate_pattern(&self, value: &Value, pattern: &str) -> Result<(), ExtractionError> {
        let mut cache = self.regex_cache.lock().map_err(|_| {
            ExtractionError::PatternError("Failed to acquire regex cache lock".to_string())
        })?;

        let regex = cache.entry(pattern.to_string()).or_insert_with(|| {
            Regex::new(pattern).unwrap_or_else(|_| {
                Regex::new(".*").expect("Fallback regex pattern '.*' should always be valid")
            })
        });

        let text = value.as_str().unwrap_or("");

        if regex.is_match(text) {
            Ok(())
        } else {
            error!("Validation failed for pattern: {}", pattern);
            Err(ExtractionError::ValidationError(format!(
                "Value '{}' does not match pattern '{}'",
                text, pattern
            )))
        }
    }

    /// Converts value to target data type
    fn convert_type(
        &self,
        value: &Value,
        target_type: &DataType,
    ) -> Result<Value, ExtractionError> {
        match target_type {
            DataType::String => Ok(Value::String(value.to_string())),
            DataType::Integer => {
                if let Some(n) = value.as_i64() {
                    Ok(Value::Number(n.into()))
                } else if let Some(s) = value.as_str() {
                    s.parse::<i64>()
                        .map(|n| Value::Number(n.into()))
                        .map_err(|_| {
                            ExtractionError::ConversionError(format!(
                                "Cannot convert '{}' to integer",
                                s
                            ))
                        })
                } else {
                    Err(ExtractionError::ConversionError(
                        "Cannot convert to integer".to_string(),
                    ))
                }
            }
            DataType::Float => {
                if let Some(n) = value.as_f64() {
                    Ok(Value::Number(
                        serde_json::Number::from_f64(n).unwrap_or_else(|| 0.into()),
                    ))
                } else if let Some(s) = value.as_str() {
                    s.parse::<f64>()
                        .ok()
                        .and_then(serde_json::Number::from_f64)
                        .map(Value::Number)
                        .ok_or_else(|| {
                            ExtractionError::ConversionError(format!(
                                "Cannot convert '{}' to float",
                                s
                            ))
                        })
                } else {
                    Err(ExtractionError::ConversionError(
                        "Cannot convert to float".to_string(),
                    ))
                }
            }
            DataType::Boolean => {
                if let Some(b) = value.as_bool() {
                    Ok(Value::Bool(b))
                } else if let Some(s) = value.as_str() {
                    Ok(Value::Bool(matches!(
                        s.to_lowercase().as_str(),
                        "true" | "1" | "yes"
                    )))
                } else {
                    Err(ExtractionError::ConversionError(
                        "Cannot convert to boolean".to_string(),
                    ))
                }
            }
            DataType::DateTime => {
                if let Some(s) = value.as_str() {
                    Ok(Value::String(s.to_string()))
                } else {
                    Err(ExtractionError::ConversionError(
                        "Cannot convert to datetime".to_string(),
                    ))
                }
            }
            DataType::Array(_) => Ok(Value::Array(vec![value.clone()])),
            DataType::Object => Ok(value.clone()),
        }
    }
}

impl Default for DataExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extractor_creation() {
        let extractor = DataExtractor::new();
        assert_eq!(extractor.config.strict_mode, true);
    }

    #[test]
    fn test_field_rule_creation() {
        let rule = FieldRule {
            name: "title".to_string(),
            selector: ".title".to_string(),
            attribute: None,
            data_type: DataType::String,
            required: true,
            default_value: None,
            transformations: vec!["trim".to_string()],
            pattern: None,
        };
        assert_eq!(rule.name, "title");
    }
}
