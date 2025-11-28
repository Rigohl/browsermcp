use kuchikiki::traits::TendrilSink;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use tracing::{debug, error, span, Level};

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid HTML: {0}")]
    InvalidHtml(String),

    #[error("CSS selector error: {0}")]
    InvalidSelector(String),

    #[error("XPath error: {0}")]
    InvalidXPath(String),

    #[error("Element not found: {0}")]
    ElementNotFound(String),

    #[error("Parse error: {0}")]
    ParseFailed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserConfig {
    pub encoding: String,
    pub preserve_whitespace: bool,
    pub normalize_spaces: bool,
    pub max_depth: Option<usize>,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            encoding: "utf-8".to_string(),
            preserve_whitespace: false,
            normalize_spaces: true,
            max_depth: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    pub case_sensitive: bool,
    pub whole_words: bool,
    pub max_context_length: usize,
    pub include_line_numbers: bool,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            case_sensitive: false,
            whole_words: false,
            max_context_length: 100,
            include_line_numbers: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMatch {
    pub term: String,
    pub text: String,
    pub start_position: usize,
    pub end_position: usize,
    pub context: String,
    pub line_number: Option<usize>,
    pub element_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub term: String,
    pub matches: Vec<SearchMatch>,
    pub total_matches: usize,
}

#[derive(Debug, Clone)]
pub struct ParsedElement {
    pub tag: String,
    pub text: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<ParsedElement>,
    pub depth: usize,
}

#[derive(Debug, Clone)]
pub struct DomParser {
    config: ParserConfig,
}

impl DomParser {
    /// Creates a new DomParser with default configuration
    ///
    /// # Example
    /// ```ignore
    /// let parser = DomParser::new();
    /// ```
    pub fn new() -> Self {
        Self {
            config: ParserConfig::default(),
        }
    }

    /// Creates a new DomParser with custom configuration
    ///
    /// # Example
    /// ```ignore
    /// let config = ParserConfig {
    ///     encoding: "utf-8".to_string(),
    ///     preserve_whitespace: false,
    ///     normalize_spaces: true,
    ///     max_depth: Some(10),
    /// };
    /// let parser = DomParser::with_config(config);
    /// ```
    pub fn with_config(config: ParserConfig) -> Self {
        Self { config }
    }

    /// Parses HTML content into a DOM tree
    ///
    /// # Example
    /// ```ignore
    /// let html = r#"<html><body><div class="content">Hello</div></body></html>"#;
    /// let parsed = parser.parse_html(html)?;
    /// ```
    pub fn parse_html(&self, html: &str) -> Result<ParsedElement, ParseError> {
        let span = span!(Level::DEBUG, "parse_html", html_len = html.len());
        let _guard = span.enter();

        debug!("Parsing HTML content");

        if html.is_empty() {
            return Err(ParseError::InvalidHtml("Empty HTML content".to_string()));
        }

        // Use html5ever for robust HTML parsing
        let document = kuchikiki::parse_html().one(html.to_string());

        self.build_dom_tree(&document, 0)
    }

    /// Selects elements using CSS selector
    ///
    /// # Example
    /// ```ignore
    /// let elements = parser.select_css(".article > .title")?;
    /// ```
    pub fn select_css(&self, html: &str, selector: &str) -> Result<Vec<ParsedElement>, ParseError> {
        let span = span!(Level::DEBUG, "select_css", selector);
        let _guard = span.enter();

        debug!("Selecting elements with CSS selector: {}", selector);

        let document = kuchikiki::parse_html().one(html.to_string());

        let mut results = Vec::new();

        // Simple CSS selector matching - in production would use selectors crate
        if let Some(class_name) = selector.strip_prefix('.') {
            self.find_by_class(&document, class_name, &mut results);
        } else if let Some(id_name) = selector.strip_prefix('#') {
            self.find_by_id(&document, id_name, &mut results);
        } else {
            self.find_by_tag(&document, selector, &mut results);
        }

        debug!("Found {} elements matching selector", results.len());
        Ok(results)
    }

    /// Extracts text content from a selector
    ///
    /// # Example
    /// ```ignore
    /// let text = parser.extract_text(html, ".title")?;
    /// ```
    pub fn extract_text(&self, html: &str, selector: &str) -> Result<String, ParseError> {
        let elements = self.select_css(html, selector)?;

        if elements.is_empty() {
            return Err(ParseError::ElementNotFound(selector.to_string()));
        }

        let text = elements
            .iter()
            .map(|e| self.get_all_text(e))
            .collect::<Vec<_>>()
            .join(" ");

        let normalized = if self.config.normalize_spaces {
            text.split_whitespace().collect::<Vec<_>>().join(" ")
        } else {
            text
        };

        Ok(normalized)
    }

    /// Extracts attribute value from a selector
    ///
    /// # Example
    /// ```ignore
    /// let href = parser.extract_attribute(html, "a.link", "href")?;
    /// ```
    pub fn extract_attribute(
        &self,
        html: &str,
        selector: &str,
        attr: &str,
    ) -> Result<String, ParseError> {
        let elements = self.select_css(html, selector)?;

        if elements.is_empty() {
            return Err(ParseError::ElementNotFound(selector.to_string()));
        }

        elements
            .first()
            .and_then(|e| e.attributes.get(attr).cloned())
            .ok_or_else(|| {
                ParseError::ElementNotFound(format!("{} attribute on {}", attr, selector))
            })
    }

    /// Extracts multiple attribute values from elements matching selector
    ///
    /// # Example
    /// ```ignore
    /// let hrefs = parser.extract_attributes(html, "a", "href")?;
    /// ```
    pub fn extract_attributes(
        &self,
        html: &str,
        selector: &str,
        attr: &str,
    ) -> Result<Vec<String>, ParseError> {
        let elements = self.select_css(html, selector)?;

        Ok(elements
            .iter()
            .filter_map(|e| e.attributes.get(attr).cloned())
            .collect())
    }

    /// Extracts structured data from HTML using CSS selectors
    ///
    /// # Example
    /// ```ignore
    /// let mut rules = HashMap::new();
    /// rules.insert("title".to_string(), ".article-title");
    /// rules.insert("author".to_string(), ".article-author");
    /// let data = parser.extract_structured(html, rules)?;
    /// ```
    pub fn extract_structured(
        &self,
        html: &str,
        rules: HashMap<String, &str>,
    ) -> Result<HashMap<String, String>, ParseError> {
        let span = span!(
            Level::DEBUG,
            "extract_structured",
            rules_count = rules.len()
        );
        let _guard = span.enter();

        debug!("Extracting structured data with {} rules", rules.len());

        let mut result = HashMap::new();

        for (key, selector) in rules {
            match self.extract_text(html, selector) {
                Ok(text) => {
                    result.insert(key.clone(), text);
                    debug!("Successfully extracted {}", key);
                }
                Err(e) => {
                    error!("Failed to extract {}: {}", key, e);
                    // Continue processing other fields on error
                }
            }
        }

        Ok(result)
    }

    /// Builds the entire DOM tree from root node
    fn build_dom_tree(
        &self,
        node: &kuchikiki::NodeRef,
        depth: usize,
    ) -> Result<ParsedElement, ParseError> {
        if let Some(max_depth) = self.config.max_depth {
            if depth > max_depth {
                return Err(ParseError::ParseFailed(
                    "Maximum depth exceeded".to_string(),
                ));
            }
        }

        self.element_to_parsed(node, depth)
    }

    /// Converts a kuchikiki node to ParsedElement
    fn element_to_parsed(
        &self,
        node: &kuchikiki::NodeRef,
        depth: usize,
    ) -> Result<ParsedElement, ParseError> {
        let mut elem = ParsedElement {
            tag: String::new(),
            text: String::new(),
            attributes: HashMap::new(),
            children: Vec::new(),
            depth,
        };

        if let Some(elem_ref) = node.as_element() {
            elem.tag = elem_ref.name.local.to_string();

            for (key, val) in elem_ref.attributes.borrow().map.iter() {
                let key_str = format!("{}", key.local);
                let val_str = format!("{:?}", val);
                elem.attributes.insert(key_str, val_str);
            }
        }

        for child in node.children() {
            if let Some(text_node) = child.as_text() {
                let text = text_node.borrow().to_string();
                if self.config.preserve_whitespace || !text.trim().is_empty() {
                    elem.text.push_str(&text);
                }
            } else if child.as_element().is_some() {
                if let Ok(child_elem) = self.element_to_parsed(&child, depth + 1) {
                    elem.children.push(child_elem);
                }
            }
        }

        if !self.config.preserve_whitespace {
            elem.text = elem.text.trim().to_string();
        }

        Ok(elem)
    }

    /// Find elements by class name
    fn find_by_class(
        &self,
        node: &kuchikiki::NodeRef,
        class_name: &str,
        results: &mut Vec<ParsedElement>,
    ) {
        if let Some(elem_ref) = node.as_element() {
            if let Some(classes) = elem_ref.attributes.borrow().get("class") {
                if classes.split_whitespace().any(|c| c == class_name) {
                    if let Ok(parsed) = self.element_to_parsed(node, 0) {
                        results.push(parsed);
                    }
                }
            }
        }

        for child in node.children() {
            self.find_by_class(&child, class_name, results);
        }
    }

    /// Find elements by ID
    fn find_by_id(
        &self,
        node: &kuchikiki::NodeRef,
        id_name: &str,
        results: &mut Vec<ParsedElement>,
    ) {
        if let Some(elem_ref) = node.as_element() {
            if let Some(id) = elem_ref.attributes.borrow().get("id") {
                if id == id_name {
                    if let Ok(parsed) = self.element_to_parsed(node, 0) {
                        results.push(parsed);
                        return;
                    }
                }
            }
        }

        for child in node.children() {
            self.find_by_id(&child, id_name, results);
        }
    }

    /// Find elements by tag name
    fn find_by_tag(
        &self,
        node: &kuchikiki::NodeRef,
        tag_name: &str,
        results: &mut Vec<ParsedElement>,
    ) {
        if let Some(elem_ref) = node.as_element() {
            if elem_ref.name.local.to_string() == tag_name {
                if let Ok(parsed) = self.element_to_parsed(node, 0) {
                    results.push(parsed);
                }
            }
        }

        for child in node.children() {
            self.find_by_tag(&child, tag_name, results);
        }
    }

    /// Searches for specific terms in HTML content
    ///
    /// # Example
    /// ```ignore
    /// let config = SearchConfig::default();
    /// let results = parser.search_content(html, &["error", "warning"], &config)?;
    /// ```
    pub fn search_content(
        &self,
        html: &str,
        terms: &[&str],
        config: &SearchConfig,
    ) -> Result<Vec<SearchResult>, ParseError> {
        let span = span!(Level::DEBUG, "search_content", terms_count = terms.len());
        let _guard = span.enter();

        debug!("Searching for {} terms in HTML content", terms.len());

        let mut results = Vec::new();

        for &term in terms {
            let matches = self.search_single_term(html, term, config)?;
            let total_matches = matches.len();
            results.push(SearchResult {
                term: term.to_string(),
                matches,
                total_matches,
            });
        }

        Ok(results)
    }

    /// Searches for a single term in HTML content
    fn search_single_term(
        &self,
        html: &str,
        term: &str,
        config: &SearchConfig,
    ) -> Result<Vec<SearchMatch>, ParseError> {
        let mut matches = Vec::new();

        // Build regex pattern
        let pattern = if config.whole_words {
            if config.case_sensitive {
                format!(r"\b{}\b", regex::escape(term))
            } else {
                format!(r"(?i)\b{}\b", regex::escape(term))
            }
        } else if config.case_sensitive {
            regex::escape(term)
        } else {
            format!("(?i){}", regex::escape(term))
        };

        let regex = Regex::new(&pattern)
            .map_err(|e| ParseError::ParseFailed(format!("Invalid regex pattern: {}", e)))?;

        // Split HTML into lines for line numbering
        let lines: Vec<&str> = html.lines().collect();
        let mut current_pos = 0;

        for (line_idx, &line) in lines.iter().enumerate() {
            for cap in regex.find_iter(line) {
                let start_pos = current_pos + cap.start();
                let end_pos = current_pos + cap.end();
                let matched_text = cap.as_str().to_string();

                // Extract context
                let context_start = if cap.start() > config.max_context_length / 2 {
                    cap.start() - config.max_context_length / 2
                } else {
                    0
                };
                let context_end = if cap.end() + config.max_context_length / 2 < line.len() {
                    cap.end() + config.max_context_length / 2
                } else {
                    line.len()
                };

                let context = format!(
                    "...{}[{}]{}...",
                    &line[context_start..cap.start()],
                    matched_text,
                    &line[cap.end()..context_end]
                );

                let search_match = SearchMatch {
                    term: term.to_string(),
                    text: matched_text,
                    start_position: start_pos,
                    end_position: end_pos,
                    context,
                    line_number: if config.include_line_numbers {
                        Some(line_idx + 1)
                    } else {
                        None
                    },
                    element_path: None, // Could be enhanced to include DOM path
                };

                matches.push(search_match);
            }

            current_pos += line.len() + 1; // +1 for newline
        }

        debug!("Found {} matches for term '{}'", matches.len(), term);
        Ok(matches)
    }

    /// Searches for terms within specific HTML elements
    ///
    /// # Example
    /// ```ignore
    /// let results = parser.search_in_elements(html, ".error", &["exception", "failed"])?;
    /// ```
    pub fn search_in_elements(
        &self,
        html: &str,
        selector: &str,
        terms: &[&str],
    ) -> Result<Vec<SearchResult>, ParseError> {
        let span = span!(
            Level::DEBUG,
            "search_in_elements",
            selector,
            terms_count = terms.len()
        );
        let _guard = span.enter();

        debug!("Searching in elements matching selector: {}", selector);

        let elements = self.select_css(html, selector)?;
        let mut all_results = Vec::new();

        for element in elements {
            let element_html = self.element_to_html(&element);
            let config = SearchConfig::default();
            let results = self.search_content(&element_html, terms, &config)?;

            // Add element path to matches
            for mut result in results {
                for match_item in &mut result.matches {
                    match_item.element_path = Some(format!("<{}>", element.tag));
                }
                all_results.push(result);
            }
        }

        Ok(all_results)
    }

    /// Converts ParsedElement back to HTML string (simplified)
    fn element_to_html(&self, element: &ParsedElement) -> String {
        let mut html = format!("<{}", element.tag);

        for (attr, value) in &element.attributes {
            html.push_str(&format!(" {}=\"{}\"", attr, value));
        }

        html.push('>');

        if !element.text.is_empty() {
            html.push_str(&element.text);
        }

        for child in &element.children {
            html.push_str(&self.element_to_html(child));
        }

        html.push_str(&format!("</{}>", element.tag));
        html
    }

    /// Recursively extracts all text content from element and children
    fn get_all_text(&self, elem: &ParsedElement) -> String {
        let mut text = elem.text.clone();
        for child in &elem.children {
            text.push(' ');
            text.push_str(&self.get_all_text(child));
        }
        text
    }
}

impl Default for DomParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_html() {
        let parser = DomParser::new();
        let html = r#"<html><body><div>Hello</div></body></html>"#;
        assert!(parser.parse_html(html).is_ok());
    }

    #[test]
    fn test_empty_html_error() {
        let parser = DomParser::new();
        assert!(parser.parse_html("").is_err());
    }

    #[test]
    fn test_search_content_basic() {
        let parser = DomParser::new();
        let html = r#"<html><body><div class="error">Error: Something went wrong</div><p>Warning: Check this</p></body></html>"#;
        let config = SearchConfig::default();
        let results = parser.search_content(html, &["error", "warning"], &config);

        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(results.len(), 2);

        // Check error matches
        assert_eq!(results[0].term, "error");
        assert!(results[0].total_matches > 0);

        // Check warning matches
        assert_eq!(results[1].term, "warning");
        assert!(results[1].total_matches > 0);
    }

    #[test]
    fn test_search_case_insensitive() {
        let parser = DomParser::new();
        let html = "This has ERROR and error in it.";
        let config = SearchConfig {
            case_sensitive: false,
            ..Default::default()
        };
        let results = parser.search_content(html, &["error"], &config);

        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(results[0].total_matches, 2);
    }

    #[test]
    fn test_search_whole_words() {
        let parser = DomParser::new();
        let html = "This has error and errors in it.";
        let config = SearchConfig {
            whole_words: true,
            ..Default::default()
        };
        let results = parser.search_content(html, &["error"], &config);

        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(results[0].total_matches, 1); // Only "error", not "errors"
    }
}
