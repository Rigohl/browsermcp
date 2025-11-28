use regex::Regex;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    Text,
    Email,
    Password,
    Number,
    Tel,
    Url,
    Date,
    Time,
    DateTime,
    Select,
    Textarea,
    Checkbox,
    Radio,
    File,
    Hidden,
    Search,
    Range,
    Color,
    Week,
    Month,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormField {
    pub name: String,
    pub id: Option<String>,
    pub field_type: FieldType,
    pub label: Option<String>,
    pub placeholder: Option<String>,
    pub value: Option<String>,
    pub required: bool,
    pub options: Vec<String>,
    pub pattern: Option<String>,
    pub max_length: Option<usize>,
    pub min_length: Option<usize>,
    pub html: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormElement {
    pub id: Option<String>,
    pub name: Option<String>,
    pub action: Option<String>,
    pub method: String,
    pub enctype: Option<String>,
    pub fields: Vec<FormField>,
    pub submit_button: Option<String>,
    pub html: String,
}

pub struct FormDetector {
    #[allow(dead_code)]
    email_regex: Regex,
    #[allow(dead_code)]
    phone_regex: Regex,
    #[allow(dead_code)]
    url_regex: Regex,
    #[allow(dead_code)]
    date_regex: Regex,
}

impl FormDetector {
    pub fn new() -> Self {
        Self {
            email_regex: Regex::new(
                r"(?i)(?:email|mail|contact|cuenta|correo|mail_address|email_address)",
            )
            .unwrap(),
            phone_regex: Regex::new(r"(?i)(?:phone|tel|telephone|móvil|celular|numero)").unwrap(),
            url_regex: Regex::new(r"(?i)(?:website|url|site|web|sitio)").unwrap(),
            date_regex: Regex::new(r"(?i)(?:date|fecha|birth|nacimiento|dob)").unwrap(),
        }
    }

    pub fn detect_forms(&self, html: &str) -> Vec<FormElement> {
        let mut forms = Vec::new();

        let form_pattern = Regex::new(r"(?s)<form[^>]*>(.*?)</form>").unwrap();
        for form_match in form_pattern.captures_iter(html) {
            if let Some(form_html) = form_match.get(0) {
                let form_str = form_html.as_str();
                if let Some(form) = self.parse_form(form_str) {
                    debug!("Detected form: {:?}", form.name);
                    forms.push(form);
                }
            }
        }

        if forms.is_empty() {
            info!("No forms detected in HTML, attempting to extract implicit forms");
            forms.extend(self.detect_implicit_forms(html));
        }

        forms
    }

    fn parse_form(&self, form_html: &str) -> Option<FormElement> {
        let form_start_pattern = Regex::new(r#"<form\s+([^>]*)>"#).ok()?;

        let attributes = form_start_pattern
            .captures(form_html)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str())
            .unwrap_or("");

        let id = self.extract_attribute(attributes, "id");
        let name = self.extract_attribute(attributes, "name");
        let action = self.extract_attribute(attributes, "action");
        let method = self
            .extract_attribute(attributes, "method")
            .unwrap_or_else(|| "POST".to_string());
        let enctype = self.extract_attribute(attributes, "enctype");

        let fields = self.extract_fields(form_html);
        let submit_button = self.find_submit_button(form_html);

        Some(FormElement {
            id,
            name,
            action,
            method,
            enctype,
            fields,
            submit_button,
            html: form_html.to_string(),
        })
    }

    fn extract_fields(&self, form_html: &str) -> Vec<FormField> {
        let mut fields = Vec::new();

        let input_pattern = Regex::new(r#"<input\s+([^>]*)/?>"#).unwrap();
        for capture in input_pattern.captures_iter(form_html) {
            if let Some(attrs) = capture.get(1) {
                if let Some(field) = self.parse_input_field(attrs.as_str(), form_html) {
                    fields.push(field);
                }
            }
        }

        let select_pattern = Regex::new(r#"<select\s+([^>]*)>(.*?)</select>"#).unwrap();
        for capture in select_pattern.captures_iter(form_html) {
            if let Some(attrs) = capture.get(1) {
                if let Some(content) = capture.get(2) {
                    if let Some(field) = self.parse_select_field(attrs.as_str(), content.as_str()) {
                        fields.push(field);
                    }
                }
            }
        }

        let textarea_pattern = Regex::new(r#"<textarea\s+([^>]*)>(.*?)</textarea>"#).unwrap();
        for capture in textarea_pattern.captures_iter(form_html) {
            if let Some(attrs) = capture.get(1) {
                if let Some(field) = self.parse_textarea_field(attrs.as_str()) {
                    fields.push(field);
                }
            }
        }

        fields
    }

    fn parse_input_field(&self, attributes: &str, form_html: &str) -> Option<FormField> {
        let name = self.extract_attribute(attributes, "name")?;
        let id = self.extract_attribute(attributes, "id");
        let field_type_str = self
            .extract_attribute(attributes, "type")
            .unwrap_or_else(|| "text".to_string());
        let field_type = self.string_to_field_type(&field_type_str);
        let value = self.extract_attribute(attributes, "value");
        let placeholder = self.extract_attribute(attributes, "placeholder");
        let required = attributes.contains("required");
        let pattern = self.extract_attribute(attributes, "pattern");
        let max_length = self.extract_number_attribute(attributes, "maxlength");
        let min_length = self.extract_number_attribute(attributes, "minlength");

        let label = self.find_label_for_field(&name, &id, form_html);

        Some(FormField {
            name,
            id,
            field_type,
            label,
            placeholder,
            value,
            required,
            options: Vec::new(),
            pattern,
            max_length,
            min_length,
            html: format!("<input {}>", attributes),
        })
    }

    fn parse_select_field(&self, attributes: &str, content: &str) -> Option<FormField> {
        let name = self.extract_attribute(attributes, "name")?;
        let id = self.extract_attribute(attributes, "id");
        let value = self.extract_attribute(attributes, "value");
        let required = attributes.contains("required");

        let options = self.extract_select_options(content);

        Some(FormField {
            name,
            id,
            field_type: FieldType::Select,
            label: None,
            placeholder: None,
            value,
            required,
            options,
            pattern: None,
            max_length: None,
            min_length: None,
            html: format!("<select {}>{}</select>", attributes, content),
        })
    }

    fn parse_textarea_field(&self, attributes: &str) -> Option<FormField> {
        let name = self.extract_attribute(attributes, "name")?;
        let id = self.extract_attribute(attributes, "id");
        let required = attributes.contains("required");
        let max_length = self.extract_number_attribute(attributes, "maxlength");
        let min_length = self.extract_number_attribute(attributes, "minlength");

        Some(FormField {
            name,
            id,
            field_type: FieldType::Textarea,
            label: None,
            placeholder: None,
            value: None,
            required,
            options: Vec::new(),
            pattern: None,
            max_length,
            min_length,
            html: format!("<textarea {}>", attributes),
        })
    }

    fn extract_select_options(&self, content: &str) -> Vec<String> {
        let mut options = Vec::new();
        let option_pattern = Regex::new(r#"<option[^>]*>([^<]*)</option>"#).unwrap();

        for capture in option_pattern.captures_iter(content) {
            if let Some(text) = capture.get(1) {
                let option_text = text.as_str().trim().to_string();
                if !option_text.is_empty() {
                    options.push(option_text);
                }
            }
        }

        options
    }

    fn find_label_for_field(
        &self,
        name: &str,
        id: &Option<String>,
        form_html: &str,
    ) -> Option<String> {
        if let Some(field_id) = id {
            let label_pattern = Regex::new(&format!(
                r#"<label[^>]*for\s*=\s*['\"]?{}['\"]?[^>]*>([^<]*)</label>"#,
                regex::escape(field_id)
            ))
            .ok()?;
            if let Some(capture) = label_pattern.captures(form_html) {
                if let Some(text) = capture.get(1) {
                    return Some(text.as_str().trim().to_string());
                }
            }
        }

        let label_pattern = Regex::new(&format!(
            r#"<label[^>]*>([^<]*?{}[^<]*)</label>"#,
            regex::escape(name)
        ))
        .ok()?;

        if let Some(capture) = label_pattern.captures(form_html) {
            if let Some(text) = capture.get(1) {
                return Some(text.as_str().trim().to_string());
            }
        }

        None
    }

    fn find_submit_button(&self, form_html: &str) -> Option<String> {
        let button_pattern =
            Regex::new(r#"<button[^>]*type\s*=\s*['\"]?submit['\"]?[^>]*>([^<]*)</button>"#)
                .ok()?;

        if let Some(capture) = button_pattern.captures(form_html) {
            if let Some(text) = capture.get(1) {
                return Some(text.as_str().trim().to_string());
            }
        }

        let input_pattern = Regex::new(r#"<input[^>]*type\s*=\s*['\"]?submit['\"]?[^>]*>"#).ok()?;
        if input_pattern.is_match(form_html) {
            return Some("Submit".to_string());
        }

        None
    }

    fn detect_implicit_forms(&self, html: &str) -> Vec<FormElement> {
        let mut forms = Vec::new();
        let input_pattern = Regex::new(r#"<input\s+([^>]*)/?>"#).unwrap();

        if input_pattern.is_match(html) {
            let mut fields = Vec::new();
            for capture in input_pattern.captures_iter(html) {
                if let Some(attrs) = capture.get(1) {
                    if let Some(field) = self.parse_input_field(attrs.as_str(), html) {
                        fields.push(field);
                    }
                }
            }

            if !fields.is_empty() {
                forms.push(FormElement {
                    id: None,
                    name: Some("implicit_form".to_string()),
                    action: None,
                    method: "POST".to_string(),
                    enctype: None,
                    fields,
                    submit_button: None,
                    html: html.to_string(),
                });
            }
        }

        forms
    }

    fn extract_attribute(&self, attributes: &str, attr_name: &str) -> Option<String> {
        let patterns = vec![
            format!(r#"{}=["']([^"']*)["']"#, attr_name),
            format!(r#"{}=([^\s>]*)"#, attr_name),
        ];

        for pattern_str in patterns {
            if let Ok(pattern) = Regex::new(&pattern_str) {
                if let Some(capture) = pattern.captures(attributes) {
                    if let Some(value) = capture.get(1) {
                        return Some(value.as_str().to_string());
                    }
                }
            }
        }

        None
    }

    fn extract_number_attribute(&self, attributes: &str, attr_name: &str) -> Option<usize> {
        self.extract_attribute(attributes, attr_name)
            .and_then(|val| val.parse::<usize>().ok())
    }

    fn string_to_field_type(&self, type_str: &str) -> FieldType {
        match type_str.to_lowercase().as_str() {
            "email" => FieldType::Email,
            "password" => FieldType::Password,
            "number" => FieldType::Number,
            "tel" => FieldType::Tel,
            "url" => FieldType::Url,
            "date" => FieldType::Date,
            "time" => FieldType::Time,
            "datetime" | "datetime-local" => FieldType::DateTime,
            "checkbox" => FieldType::Checkbox,
            "radio" => FieldType::Radio,
            "file" => FieldType::File,
            "hidden" => FieldType::Hidden,
            "search" => FieldType::Search,
            "range" => FieldType::Range,
            "color" => FieldType::Color,
            "week" => FieldType::Week,
            "month" => FieldType::Month,
            "text" => FieldType::Text,
            _ => FieldType::Unknown,
        }
    }
}

impl Default for FormDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_detection() {
        let detector = FormDetector::new();
        let html = r#"
            <form id="login" method="post" action="/login">
                <input type="email" name="email" required />
                <input type="password" name="password" required />
                <button type="submit">Login</button>
            </form>
        "#;

        let forms = detector.detect_forms(html);
        assert_eq!(forms.len(), 1);
        assert_eq!(forms[0].fields.len(), 2);
    }

    #[test]
    fn test_field_type_detection() {
        let detector = FormDetector::new();
        assert_eq!(detector.string_to_field_type("email"), FieldType::Email);
        assert_eq!(
            detector.string_to_field_type("password"),
            FieldType::Password
        );
        assert_eq!(detector.string_to_field_type("text"), FieldType::Text);
    }
}
