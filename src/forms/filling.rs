use super::detection::{FieldType, FormElement, FormField};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, error, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldData {
    pub field_name: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct FieldMatcher {
    email_patterns: Vec<Regex>,
    password_patterns: Vec<Regex>,
    name_patterns: Vec<Regex>,
    phone_patterns: Vec<Regex>,
    url_patterns: Vec<Regex>,
    date_patterns: Vec<Regex>,
    address_patterns: Vec<Regex>,
    city_patterns: Vec<Regex>,
    country_patterns: Vec<Regex>,
    postal_patterns: Vec<Regex>,
}

impl FieldMatcher {
    pub fn new() -> Self {
        Self {
            email_patterns: vec![
                Regex::new(r"(?i)(?:email|mail|contact|cuenta|correo|mail_address|email_address|e-mail)")
                    .expect("Invalid email pattern regex"),
                Regex::new(r"(?i)user.*email")
                    .expect("Invalid user email pattern regex"),
            ],
            password_patterns: vec![
                Regex::new(r"(?i)(?:password|pass|pwd|passwd|contraseña)")
                    .expect("Invalid password pattern regex"),
                Regex::new(r"(?i)user.*password")
                    .expect("Invalid user password pattern regex"),
            ],
            name_patterns: vec![
                Regex::new(r"(?i)(?:fullname|full_name|nombre|name|firstname|first_name|lastname|last_name)")
                    .expect("Invalid name pattern regex"),
                Regex::new(r"(?i)(?:user|usuario)(?:name)?")
                    .expect("Invalid user name pattern regex"),
            ],
            phone_patterns: vec![
                Regex::new(r"(?i)(?:phone|tel|telephone|móvil|celular|numero|numero_telefono)")
                    .expect("Invalid phone pattern regex"),
            ],
            url_patterns: vec![
                Regex::new(r"(?i)(?:website|url|site|web|sitio|page)")
                    .expect("Invalid URL pattern regex"),
            ],
            date_patterns: vec![
                Regex::new(r"(?i)(?:date|fecha|birth|nacimiento|dob|birthday)")
                    .expect("Invalid date pattern regex"),
            ],
            address_patterns: vec![
                Regex::new(r"(?i)(?:address|direccion|street|calle)")
                    .expect("Invalid address pattern regex"),
            ],
            city_patterns: vec![
                Regex::new(r"(?i)(?:city|ciudad|town|pueblo)")
                    .expect("Invalid city pattern regex"),
            ],
            country_patterns: vec![
                Regex::new(r"(?i)(?:country|pais|nation)")
                    .expect("Invalid country pattern regex"),
            ],
            postal_patterns: vec![
                Regex::new(r"(?i)(?:postal|zip|code|postal_code|codigo_postal)")
                    .expect("Invalid postal pattern regex"),
            ],
        }
    }

    pub fn match_field_type(&self, field: &FormField) -> Option<String> {
        let field_identifier = format!(
            "{} {} {}",
            field.name,
            field.label.as_deref().unwrap_or(""),
            field.placeholder.as_deref().unwrap_or("")
        );

        if matches!(field.field_type, FieldType::Email) {
            return Some("email".to_string());
        }

        if matches!(field.field_type, FieldType::Password) {
            return Some("password".to_string());
        }

        if self
            .email_patterns
            .iter()
            .any(|p| p.is_match(&field_identifier))
        {
            return Some("email".to_string());
        }

        if self
            .password_patterns
            .iter()
            .any(|p| p.is_match(&field_identifier))
        {
            return Some("password".to_string());
        }

        if self
            .name_patterns
            .iter()
            .any(|p| p.is_match(&field_identifier))
        {
            return Some("name".to_string());
        }

        if self
            .phone_patterns
            .iter()
            .any(|p| p.is_match(&field_identifier))
        {
            return Some("phone".to_string());
        }

        if self
            .url_patterns
            .iter()
            .any(|p| p.is_match(&field_identifier))
        {
            return Some("url".to_string());
        }

        if self
            .date_patterns
            .iter()
            .any(|p| p.is_match(&field_identifier))
        {
            return Some("date".to_string());
        }

        if self
            .address_patterns
            .iter()
            .any(|p| p.is_match(&field_identifier))
        {
            return Some("address".to_string());
        }

        if self
            .city_patterns
            .iter()
            .any(|p| p.is_match(&field_identifier))
        {
            return Some("city".to_string());
        }

        if self
            .country_patterns
            .iter()
            .any(|p| p.is_match(&field_identifier))
        {
            return Some("country".to_string());
        }

        if self
            .postal_patterns
            .iter()
            .any(|p| p.is_match(&field_identifier))
        {
            return Some("postal".to_string());
        }

        if matches!(field.field_type, FieldType::Select) {
            return Some("select".to_string());
        }

        if matches!(field.field_type, FieldType::Textarea) {
            return Some("textarea".to_string());
        }

        None
    }
}

impl Default for FieldMatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct SmartFiller {
    matcher: FieldMatcher,
    data_map: HashMap<String, String>,
}

impl SmartFiller {
    pub fn new() -> Self {
        Self {
            matcher: FieldMatcher::new(),
            data_map: HashMap::new(),
        }
    }

    pub fn with_data(mut self, data: HashMap<String, String>) -> Self {
        self.data_map = data;
        self
    }

    pub fn add_data(&mut self, key: &str, value: &str) {
        self.data_map.insert(key.to_string(), value.to_string());
        debug!("Added data mapping: {} -> {}", key, value);
    }

    pub fn fill_form(&self, form: &mut FormElement) -> Result<Vec<FieldData>, String> {
        let mut filled_data = Vec::new();

        for field in &mut form.fields {
            if let Some(matched_type) = self.matcher.match_field_type(field) {
                debug!("Matched field '{}' to type: {}", field.name, matched_type);

                if let Some(value) = self.get_value_for_field(&matched_type, field) {
                    debug!("Filling field '{}' with value", field.name);
                    field.value = Some(value.clone());
                    filled_data.push(FieldData {
                        field_name: field.name.clone(),
                        value,
                    });
                } else {
                    warn!(
                        "No value found for field: {} (type: {})",
                        field.name, matched_type
                    );
                }
            }
        }

        if filled_data.is_empty() {
            error!("No fields were filled in the form");
            return Err("No matching data for form fields".to_string());
        }

        info!("Successfully filled {} fields", filled_data.len());
        Ok(filled_data)
    }

    fn get_value_for_field(&self, field_type: &str, field: &FormField) -> Option<String> {
        if let Some(value) = self.data_map.get(field_type) {
            return Some(value.clone());
        }

        if let Some(value) = self.data_map.get(&field.name) {
            return Some(value.clone());
        }

        if let Some(label) = &field.label {
            if let Some(value) = self.data_map.get(label) {
                return Some(value.clone());
            }
        }

        None
    }

    pub fn fill_select_field(&self, field: &mut FormField, value: &str) -> Result<(), String> {
        if !matches!(field.field_type, FieldType::Select) {
            return Err("Field is not a select field".to_string());
        }

        if field.options.is_empty() {
            warn!("Select field has no options");
            return Err("No options available".to_string());
        }

        let matching_option = field
            .options
            .iter()
            .find(|opt| opt.to_lowercase().contains(&value.to_lowercase()));

        if let Some(option) = matching_option {
            field.value = Some(option.clone());
            debug!("Selected option '{}' for field '{}'", option, field.name);
            Ok(())
        } else {
            warn!(
                "No matching option for value '{}' in field '{}'",
                value, field.name
            );
            if !field.options.is_empty() {
                field.value = Some(field.options[0].clone());
                debug!(
                    "Selected default option '{}' for field '{}'",
                    field.options[0], field.name
                );
                Ok(())
            } else {
                Err("No options to select from".to_string())
            }
        }
    }

    pub fn validate_field(&self, field: &FormField) -> Result<(), String> {
        if field.required && field.value.is_none() {
            return Err(format!("Required field '{}' is empty", field.name));
        }

        if let Some(ref value) = field.value {
            if let Some(max_len) = field.max_length {
                if value.len() > max_len {
                    return Err(format!(
                        "Field '{}' exceeds max length: {} > {}",
                        field.name,
                        value.len(),
                        max_len
                    ));
                }
            }

            if let Some(min_len) = field.min_length {
                if value.len() < min_len {
                    return Err(format!(
                        "Field '{}' below min length: {} < {}",
                        field.name,
                        value.len(),
                        min_len
                    ));
                }
            }

            if let Some(ref pattern) = field.pattern {
                if let Ok(regex) = Regex::new(pattern) {
                    if !regex.is_match(value) {
                        return Err(format!(
                            "Field '{}' does not match pattern: {}",
                            field.name, pattern
                        ));
                    }
                }
            }

            match field.field_type {
                FieldType::Email => {
                    let email_regex =
                        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
                            .expect("Invalid email validation regex");
                    if !email_regex.is_match(value) {
                        return Err(format!("Field '{}' is not a valid email", field.name));
                    }
                }
                FieldType::Url => {
                    let url_regex =
                        Regex::new(r"^https?://").expect("Invalid URL validation regex");
                    if !url_regex.is_match(value) {
                        return Err(format!("Field '{}' is not a valid URL", field.name));
                    }
                }
                FieldType::Number => {
                    if value.parse::<f64>().is_err() {
                        return Err(format!("Field '{}' is not a valid number", field.name));
                    }
                }
                FieldType::Tel => {
                    let tel_regex =
                        Regex::new(r"^[\d\s\-\+\(\)]+$").expect("Invalid phone validation regex");
                    if !tel_regex.is_match(value) {
                        return Err(format!(
                            "Field '{}' is not a valid phone number",
                            field.name
                        ));
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn fill_batch(
        &self,
        form: &mut FormElement,
        data: &HashMap<String, String>,
    ) -> Result<Vec<FieldData>, String> {
        let mut filled_data = Vec::new();

        for field in &mut form.fields {
            if let Some(value) = data.get(&field.name) {
                debug!("Filling field '{}' from batch data", field.name);
                field.value = Some(value.clone());
                filled_data.push(FieldData {
                    field_name: field.name.clone(),
                    value: value.clone(),
                });
            }
        }

        Ok(filled_data)
    }
}

impl Default for SmartFiller {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FormFiller;

impl FormFiller {
    pub fn create_smart_filler() -> SmartFiller {
        SmartFiller::new()
    }

    pub fn fill_simple(
        form: &mut FormElement,
        data: &HashMap<String, String>,
    ) -> Result<Vec<FieldData>, String> {
        let mut filled_data = Vec::new();

        for field in &mut form.fields {
            for (key, value) in data.iter() {
                if field.name.to_lowercase() == key.to_lowercase()
                    || field
                        .label
                        .as_ref()
                        .map(|l| l.to_lowercase())
                        .map(|l| l.contains(&key.to_lowercase()))
                        .unwrap_or(false)
                {
                    field.value = Some(value.clone());
                    filled_data.push(FieldData {
                        field_name: field.name.clone(),
                        value: value.clone(),
                    });
                    break;
                }
            }
        }

        Ok(filled_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_matcher() {
        let matcher = FieldMatcher::new();
        let field = FormField {
            name: "user_email".to_string(),
            id: None,
            field_type: FieldType::Text,
            label: Some("Email Address".to_string()),
            placeholder: None,
            value: None,
            required: true,
            options: Vec::new(),
            pattern: None,
            max_length: None,
            min_length: None,
            html: String::new(),
        };

        let matched = matcher.match_field_type(&field);
        assert_eq!(matched, Some("email".to_string()));
    }

    #[test]
    fn test_smart_filler() {
        let mut data = HashMap::new();
        data.insert("email".to_string(), "test@example.com".to_string());

        let filler = SmartFiller::new().with_data(data);
        assert_eq!(filler.data_map.len(), 1);
    }
}
