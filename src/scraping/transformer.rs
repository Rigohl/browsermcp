use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;
use tracing::{debug, error, span, Level};

#[derive(Error, Debug)]
pub enum TransformationError {
    #[error("Invalid filter: {0}")]
    InvalidFilter(String),

    #[error("Mapping error: {0}")]
    MappingError(String),

    #[error("Aggregation error: {0}")]
    AggregationError(String),

    #[error("Normalization error: {0}")]
    NormalizationError(String),

    #[error("JSON serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Contains,
    NotContains,
    StartsWith,
    EndsWith,
    Regex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    pub field: String,
    pub operator: FilterOperator,
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mapping {
    pub from_field: String,
    pub to_field: String,
    pub transform_fn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AggregationFn {
    Sum,
    Average,
    Count,
    Min,
    Max,
    Concat,
    Join(String),
    Unique,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aggregation {
    pub field: String,
    pub function: AggregationFn,
    pub output_field: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizationRule {
    pub field: String,
    pub rule_type: String,
    pub options: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationStep {
    pub name: String,
    pub step_type: String,
    pub config: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationPipeline {
    pub name: String,
    pub steps: Vec<TransformationStep>,
    pub parallel: bool,
    pub cache_results: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTransformer {
    pub normalization_cache: HashMap<String, Value>,
}

impl DataTransformer {
    /// Creates a new DataTransformer
    ///
    /// # Example
    /// ```ignore
    /// let transformer = DataTransformer::new();
    /// ```
    pub fn new() -> Self {
        Self {
            normalization_cache: HashMap::new(),
        }
    }

    /// Applies filters to data
    ///
    /// # Example
    /// ```ignore
    /// let filters = vec![
    ///     Filter {
    ///         field: "price".to_string(),
    ///         operator: FilterOperator::GreaterThan,
    ///         value: Value::Number(100.into()),
    ///     }
    /// ];
    /// let filtered = transformer.filter(data, &filters)?;
    /// ```
    pub fn filter(
        &self,
        data: &[Value],
        filters: &[Filter],
    ) -> Result<Vec<Value>, TransformationError> {
        let span = span!(Level::DEBUG, "filter", filter_count = filters.len());
        let _guard = span.enter();

        debug!("Applying {} filters to {} items", filters.len(), data.len());

        let mut result = data.to_vec();

        for filter in filters {
            result.retain(|item| self.apply_filter(item, filter));
        }

        debug!("Filtered result: {} items", result.len());
        Ok(result)
    }

    /// Applies field mappings and transformations
    ///
    /// # Example
    /// ```ignore
    /// let mappings = vec![
    ///     Mapping {
    ///         from_field: "product_name".to_string(),
    ///         to_field: "name".to_string(),
    ///         transform_fn: Some("uppercase".to_string()),
    ///     }
    /// ];
    /// let mapped = transformer.map_fields(data, &mappings)?;
    /// ```
    pub fn map_fields(
        &self,
        data: &[Value],
        mappings: &[Mapping],
    ) -> Result<Vec<Value>, TransformationError> {
        let span = span!(Level::DEBUG, "map_fields", mapping_count = mappings.len());
        let _guard = span.enter();

        debug!(
            "Mapping {} fields across {} items",
            mappings.len(),
            data.len()
        );

        let result: Result<Vec<_>, _> = data
            .iter()
            .map(|item| self.apply_mappings(item, mappings))
            .collect();

        result
    }

    /// Aggregates data
    ///
    /// # Example
    /// ```ignore
    /// let aggregations = vec![
    ///     Aggregation {
    ///         field: "price".to_string(),
    ///         function: AggregationFn::Sum,
    ///         output_field: "total_price".to_string(),
    ///     }
    /// ];
    /// let aggregated = transformer.aggregate(data, &aggregations)?;
    /// ```
    pub fn aggregate(
        &self,
        data: &[Value],
        aggregations: &[Aggregation],
    ) -> Result<Value, TransformationError> {
        let span = span!(
            Level::DEBUG,
            "aggregate",
            aggregation_count = aggregations.len()
        );
        let _guard = span.enter();

        debug!(
            "Aggregating {} items with {} aggregations",
            data.len(),
            aggregations.len()
        );

        let mut result = serde_json::Map::new();

        for agg in aggregations {
            let values: Vec<Value> = data
                .iter()
                .filter_map(|item| item.get(&agg.field).cloned())
                .collect();

            let agg_value = match &agg.function {
                AggregationFn::Sum => self.aggregate_sum(&values)?,
                AggregationFn::Average => self.aggregate_average(&values)?,
                AggregationFn::Count => Value::Number(values.len().into()),
                AggregationFn::Min => self.aggregate_min(&values)?,
                AggregationFn::Max => self.aggregate_max(&values)?,
                AggregationFn::Concat => self.aggregate_concat(&values)?,
                AggregationFn::Join(sep) => self.aggregate_join(&values, sep)?,
                AggregationFn::Unique => self.aggregate_unique(&values)?,
            };

            result.insert(agg.output_field.clone(), agg_value);
        }

        Ok(Value::Object(result))
    }

    /// Normalizes data based on rules
    ///
    /// # Example
    /// ```ignore
    /// let rules = vec![
    ///     NormalizationRule {
    ///         field: "phone".to_string(),
    ///         rule_type: "phone_format".to_string(),
    ///         options: HashMap::new(),
    ///     }
    /// ];
    /// let normalized = transformer.normalize(data, &rules)?;
    /// ```
    pub fn normalize(
        &self,
        data: &[Value],
        rules: &[NormalizationRule],
    ) -> Result<Vec<Value>, TransformationError> {
        let span = span!(Level::DEBUG, "normalize", rule_count = rules.len());
        let _guard = span.enter();

        debug!(
            "Normalizing {} items with {} rules",
            data.len(),
            rules.len()
        );

        let result: Result<Vec<_>, _> = data
            .iter()
            .map(|item| self.apply_normalizations(item, rules))
            .collect();

        result
    }

    /// Executes a transformation pipeline
    ///
    /// # Example
    /// ```ignore
    /// let pipeline = TransformationPipeline {
    ///     name: "data_pipeline".to_string(),
    ///     steps: vec![
    ///         TransformationStep {
    ///             name: "filter".to_string(),
    ///             step_type: "filter".to_string(),
    ///             config: json!({ "field": "price", "operator": "gt", "value": 100 }),
    ///         }
    ///     ],
    ///     parallel: false,
    ///     cache_results: false,
    /// };
    /// let result = transformer.execute_pipeline(data, &pipeline)?;
    /// ```
    pub fn execute_pipeline(
        &self,
        mut data: Vec<Value>,
        pipeline: &TransformationPipeline,
    ) -> Result<Vec<Value>, TransformationError> {
        let span = span!(
            Level::DEBUG,
            "execute_pipeline",
            pipeline = pipeline.name,
            step_count = pipeline.steps.len()
        );
        let _guard = span.enter();

        debug!("Executing pipeline with {} steps", pipeline.steps.len());

        for step in &pipeline.steps {
            debug!("Executing step: {} ({})", step.name, step.step_type);

            data = match step.step_type.as_str() {
                "filter" => {
                    let filters = self.parse_filter_config(&step.config)?;
                    self.filter(&data, &filters)?
                }
                "map" => {
                    let mappings = self.parse_mapping_config(&step.config)?;
                    self.map_fields(&data, &mappings)?
                }
                "normalize" => {
                    let rules = self.parse_normalization_config(&step.config)?;
                    self.normalize(&data, &rules)?
                }
                _ => {
                    error!("Unknown transformation step type: {}", step.step_type);
                    return Err(TransformationError::InvalidFilter(format!(
                        "Unknown step type: {}",
                        step.step_type
                    )));
                }
            };
        }

        Ok(data)
    }

    /// Converts data to JSON string
    ///
    /// # Example
    /// ```ignore
    /// let json_str = transformer.to_json(&data)?;
    /// ```
    pub fn to_json(&self, data: &[Value]) -> Result<String, TransformationError> {
        serde_json::to_string_pretty(&data).map_err(TransformationError::SerializationError)
    }

    /// Converts data to JSON with custom formatting
    ///
    /// # Example
    /// ```ignore
    /// let json_compact = transformer.to_json_compact(&data)?;
    /// ```
    pub fn to_json_compact(&self, data: &[Value]) -> Result<String, TransformationError> {
        serde_json::to_string(&data).map_err(TransformationError::SerializationError)
    }

    /// Flattens nested JSON structures
    ///
    /// # Example
    /// ```ignore
    /// let flattened = transformer.flatten_json(nested_data)?;
    /// ```
    pub fn flatten_json(
        &self,
        data: &Value,
        prefix: Option<&str>,
    ) -> Result<Value, TransformationError> {
        let span = span!(Level::DEBUG, "flatten_json");
        let _guard = span.enter();

        debug!("Flattening JSON structure");

        let mut result = serde_json::Map::new();
        self.flatten_recursive(data, prefix.unwrap_or(""), &mut result);
        Ok(Value::Object(result))
    }

    // Helper methods

    fn apply_filter(&self, item: &Value, filter: &Filter) -> bool {
        let field_value = match item.get(&filter.field) {
            Some(v) => v,
            None => return false,
        };

        match filter.operator {
            FilterOperator::Equals => field_value == &filter.value,
            FilterOperator::NotEquals => field_value != &filter.value,
            FilterOperator::GreaterThan => {
                if let (Some(fv), Some(fv2)) = (field_value.as_f64(), filter.value.as_f64()) {
                    fv > fv2
                } else {
                    false
                }
            }
            FilterOperator::LessThan => {
                if let (Some(fv), Some(fv2)) = (field_value.as_f64(), filter.value.as_f64()) {
                    fv < fv2
                } else {
                    false
                }
            }
            FilterOperator::GreaterThanOrEqual => {
                if let (Some(fv), Some(fv2)) = (field_value.as_f64(), filter.value.as_f64()) {
                    fv >= fv2
                } else {
                    false
                }
            }
            FilterOperator::LessThanOrEqual => {
                if let (Some(fv), Some(fv2)) = (field_value.as_f64(), filter.value.as_f64()) {
                    fv <= fv2
                } else {
                    false
                }
            }
            FilterOperator::Contains => {
                if let (Some(fv), Some(fv2)) = (field_value.as_str(), filter.value.as_str()) {
                    fv.contains(fv2)
                } else {
                    false
                }
            }
            FilterOperator::NotContains => {
                if let (Some(fv), Some(fv2)) = (field_value.as_str(), filter.value.as_str()) {
                    !fv.contains(fv2)
                } else {
                    true
                }
            }
            FilterOperator::StartsWith => {
                if let (Some(fv), Some(fv2)) = (field_value.as_str(), filter.value.as_str()) {
                    fv.starts_with(fv2)
                } else {
                    false
                }
            }
            FilterOperator::EndsWith => {
                if let (Some(fv), Some(fv2)) = (field_value.as_str(), filter.value.as_str()) {
                    fv.ends_with(fv2)
                } else {
                    false
                }
            }
            FilterOperator::Regex => {
                if let Some(fv) = field_value.as_str() {
                    if let Some(pattern) = filter.value.as_str() {
                        regex::Regex::new(pattern)
                            .map(|r| r.is_match(fv))
                            .unwrap_or(false)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        }
    }

    fn apply_mappings(
        &self,
        item: &Value,
        mappings: &[Mapping],
    ) -> Result<Value, TransformationError> {
        let mut result = if let Some(obj) = item.as_object() {
            obj.clone()
        } else {
            serde_json::Map::new()
        };

        for mapping in mappings {
            if let Some(value) = result.get(&mapping.from_field).cloned() {
                let transformed_value = if let Some(fn_name) = &mapping.transform_fn {
                    self.apply_simple_transform(&value, fn_name)?
                } else {
                    value
                };

                result.insert(mapping.to_field.clone(), transformed_value);
            }
        }

        Ok(Value::Object(result))
    }

    fn apply_simple_transform(
        &self,
        value: &Value,
        fn_name: &str,
    ) -> Result<Value, TransformationError> {
        match fn_name {
            "uppercase" => {
                if let Some(s) = value.as_str() {
                    Ok(Value::String(s.to_uppercase()))
                } else {
                    Ok(value.clone())
                }
            }
            "lowercase" => {
                if let Some(s) = value.as_str() {
                    Ok(Value::String(s.to_lowercase()))
                } else {
                    Ok(value.clone())
                }
            }
            "trim" => {
                if let Some(s) = value.as_str() {
                    Ok(Value::String(s.trim().to_string()))
                } else {
                    Ok(value.clone())
                }
            }
            "to_string" => Ok(Value::String(value.to_string())),
            "to_number" => {
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
                            TransformationError::MappingError(format!(
                                "Cannot convert '{}' to number",
                                s
                            ))
                        })
                } else {
                    Ok(Value::Null)
                }
            }
            _ => Err(TransformationError::MappingError(format!(
                "Unknown transformation: {}",
                fn_name
            ))),
        }
    }

    fn aggregate_sum(&self, values: &[Value]) -> Result<Value, TransformationError> {
        let sum: f64 = values.iter().filter_map(|v| v.as_f64()).sum();
        serde_json::Number::from_f64(sum)
            .map(Value::Number)
            .ok_or_else(|| {
                TransformationError::AggregationError("Cannot aggregate sum".to_string())
            })
    }

    fn aggregate_average(&self, values: &[Value]) -> Result<Value, TransformationError> {
        if values.is_empty() {
            return Ok(Value::Null);
        }
        let sum: f64 = values.iter().filter_map(|v| v.as_f64()).sum();
        let count = values.len() as f64;
        let avg = sum / count;
        serde_json::Number::from_f64(avg)
            .map(Value::Number)
            .ok_or_else(|| {
                TransformationError::AggregationError("Cannot aggregate average".to_string())
            })
    }

    fn aggregate_min(&self, values: &[Value]) -> Result<Value, TransformationError> {
        values
            .iter()
            .filter_map(|v| v.as_f64())
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .and_then(|n| serde_json::Number::from_f64(n).map(Value::Number))
            .ok_or_else(|| {
                TransformationError::AggregationError("Cannot aggregate min".to_string())
            })
    }

    fn aggregate_max(&self, values: &[Value]) -> Result<Value, TransformationError> {
        values
            .iter()
            .filter_map(|v| v.as_f64())
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .and_then(|n| serde_json::Number::from_f64(n).map(Value::Number))
            .ok_or_else(|| {
                TransformationError::AggregationError("Cannot aggregate max".to_string())
            })
    }

    fn aggregate_concat(&self, values: &[Value]) -> Result<Value, TransformationError> {
        let concatenated = values
            .iter()
            .filter_map(|v| v.as_str())
            .collect::<Vec<_>>()
            .join("");
        Ok(Value::String(concatenated))
    }

    fn aggregate_join(
        &self,
        values: &[Value],
        separator: &str,
    ) -> Result<Value, TransformationError> {
        let joined = values
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(separator);
        Ok(Value::String(joined))
    }

    fn aggregate_unique(&self, values: &[Value]) -> Result<Value, TransformationError> {
        let mut unique_vals = std::collections::HashSet::new();
        let result: Vec<Value> = values
            .iter()
            .filter(|v| unique_vals.insert(v.to_string()))
            .cloned()
            .collect();
        Ok(Value::Array(result))
    }

    fn apply_normalizations(
        &self,
        item: &Value,
        rules: &[NormalizationRule],
    ) -> Result<Value, TransformationError> {
        let mut result = if let Some(obj) = item.as_object() {
            obj.clone()
        } else {
            serde_json::Map::new()
        };

        for rule in rules {
            if let Some(value) = result.get(&rule.field).cloned() {
                let normalized = self.normalize_value(&value, &rule.rule_type, &rule.options)?;
                result.insert(rule.field.clone(), normalized);
            }
        }

        Ok(Value::Object(result))
    }

    fn normalize_value(
        &self,
        value: &Value,
        rule_type: &str,
        _options: &HashMap<String, Value>,
    ) -> Result<Value, TransformationError> {
        match rule_type {
            "trim" => {
                if let Some(s) = value.as_str() {
                    Ok(Value::String(s.trim().to_string()))
                } else {
                    Ok(value.clone())
                }
            }
            "phone_format" => {
                if let Some(s) = value.as_str() {
                    let digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
                    Ok(Value::String(digits))
                } else {
                    Ok(value.clone())
                }
            }
            "email_normalize" => {
                if let Some(s) = value.as_str() {
                    Ok(Value::String(s.to_lowercase().trim().to_string()))
                } else {
                    Ok(value.clone())
                }
            }
            _ => Ok(value.clone()),
        }
    }

    fn parse_filter_config(&self, _config: &Value) -> Result<Vec<Filter>, TransformationError> {
        // Placeholder implementation
        Ok(vec![])
    }

    fn parse_mapping_config(&self, _config: &Value) -> Result<Vec<Mapping>, TransformationError> {
        // Placeholder implementation
        Ok(vec![])
    }

    fn parse_normalization_config(
        &self,
        _config: &Value,
    ) -> Result<Vec<NormalizationRule>, TransformationError> {
        // Placeholder implementation
        Ok(vec![])
    }

    fn flatten_recursive(
        &self,
        value: &Value,
        prefix: &str,
        result: &mut serde_json::Map<String, Value>,
    ) {
        match value {
            Value::Object(obj) => {
                for (k, v) in obj {
                    let key = if prefix.is_empty() {
                        k.clone()
                    } else {
                        format!("{}_{}", prefix, k)
                    };
                    self.flatten_recursive(v, &key, result);
                }
            }
            Value::Array(arr) => {
                for (i, v) in arr.iter().enumerate() {
                    let key = format!("{}_{}", prefix, i);
                    self.flatten_recursive(v, &key, result);
                }
            }
            _ => {
                result.insert(prefix.to_string(), value.clone());
            }
        }
    }
}

impl Default for DataTransformer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_transformer_creation() {
        let transformer = DataTransformer::new();
        assert!(transformer.normalization_cache.is_empty());
    }

    #[test]
    fn test_filter_operation() {
        let transformer = DataTransformer::new();
        let data = vec![json!({"price": 50}), json!({"price": 150})];
        let filters = vec![Filter {
            field: "price".to_string(),
            operator: FilterOperator::GreaterThan,
            value: Value::Number(100.into()),
        }];
        let result = transformer.filter(&data, &filters);
        assert!(result.is_ok());
    }
}
