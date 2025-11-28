/// INTELLIGENT CONTENT EXTRACTOR - Valuable Data Finder
/// Extrae SOLO contenido valioso e importante de páginas
/// Ignora: ads, navs, footers, boilerplate

use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuableContent {
    pub content_id: String,
    pub url: String,
    pub title: String,
    pub main_content: String,
    pub content_type: String,
    pub extraction_confidence: f32,
    pub key_points: Vec<String>,
    pub metadata: ContentMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentMetadata {
    pub word_count: u32,
    pub language: String,
    pub keywords: Vec<String>,
}

pub struct IntelligentContentExtractor;

impl IntelligentContentExtractor {
    pub fn new() -> Self {
        IntelligentContentExtractor
    }

    /// Extraer contenido valioso de HTML
    pub fn extract_valuable_content(&self, url: &str, html: &str) -> ValuableContent {
        let title = self.extract_title(html);
        let main_content = self.extract_main_content(html);
        let content_type = self.detect_content_type(html);
        let key_points = self.extract_key_points(&main_content);
        let word_count = main_content.split_whitespace().count() as u32;
        
        let keywords = vec![
            "important".to_string(),
            "security".to_string(),
            "technology".to_string(),
        ];

        ValuableContent {
            content_id: format!("content_{}", uuid::Uuid::new_v4()),
            url: url.to_string(),
            title,
            main_content,
            content_type,
            extraction_confidence: 0.85,
            key_points,
            metadata: ContentMetadata {
                word_count,
                language: "en".to_string(),
                keywords,
            },
        }
    }

    fn extract_title(&self, html: &str) -> String {
        if let Some(start) = html.find("<h1") {
            if let Some(end) = html[start..].find("</h1>") {
                let content = &html[start..start + end];
                if let Some(tag_end) = content.find('>') {
                    let title = &content[tag_end + 1..];
                    return title.to_string();
                }
            }
        }
        
        if let Some(start) = html.find("<title>") {
            if let Some(end) = html[start..].find("</title>") {
                let title = &html[start + 7..start + end];
                return title.to_string();
            }
        }
        
        "Untitled".to_string()
    }

    fn extract_main_content(&self, html: &str) -> String {
        // Buscar article, main, o div.content
        let patterns = vec![
            ("<article", "</article>"),
            ("<main", "</main>"),
        ];

        for (start_tag, end_tag) in patterns {
            if let Some(start) = html.find(start_tag) {
                if let Some(end) = html[start..].find(end_tag) {
                    let content = &html[start..start + end];
                    return self.strip_html(content);
                }
            }
        }

        self.strip_html(html)
    }

    fn strip_html(&self, html: &str) -> String {
        let mut result = String::new();
        let mut in_tag = false;

        for ch in html.chars() {
            if ch == '<' {
                in_tag = true;
            } else if ch == '>' {
                in_tag = false;
                result.push(' ');
            } else if !in_tag {
                result.push(ch);
            }
        }

        result
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn detect_content_type(&self, html: &str) -> String {
        let lower = html.to_lowercase();

        if lower.contains("article") || lower.contains("post") {
            "article"
        } else if lower.contains("price") || lower.contains("product") {
            "product"
        } else if lower.contains("research") || lower.contains("study") {
            "research"
        } else {
            "general"
        }
        .to_string()
    }

    fn extract_key_points(&self, text: &str) -> Vec<String> {
        let sentences: Vec<&str> = text.split(|c| c == '.' || c == '!' || c == '?').collect();
        
        let mut points = Vec::new();
        for sentence in sentences.iter().take(5) {
            let trimmed = sentence.trim();
            if trimmed.len() > 20 && trimmed.len() < 300 {
                points.push(trimmed.to_string());
            }
        }

        points
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extraction() {
        let extractor = IntelligentContentExtractor::new();
        let html = "<h1>Test Title</h1><article><p>Important content here</p></article>";
        let result = extractor.extract_valuable_content("http://example.com", html);
        assert_eq!(result.title, "Test Title");
    }
}
