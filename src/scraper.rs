/// Marketing Scraper - Real Data Extraction
/// Extrae: emails, teléfonos, links, metadata, contenido HTML completo

use reqwest::Client;
use scraper::{Html, Selector};
use regex::Regex;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

pub struct MarketingScraper {
    client: Client,
}

impl MarketingScraper {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
                .danger_accept_invalid_certs(true)
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_else(|_| Client::new()),
        }
    }

    /// Scrape URL completa con extracción de datos para marketing
    pub async fn scrape_full(
        &self,
        url: &str,
        custom_selectors: Option<Vec<String>>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        let response = self.client.get(url).send().await?;
        let status_code = response.status().as_u16();
        let html_text = response.text().await?;
        let document = Html::parse_document(&html_text);

        // Metadata básico
        let title = self.extract_title(&document);
        let description = self.extract_meta(&document, "description");
        let keywords = self.extract_meta(&document, "keywords");

        // Emails
        let emails = self.extract_emails(&html_text);

        // Teléfonos
        let phones = self.extract_phones(&html_text);

        // Links (solo internos + externos importantes)
        let links = self.extract_links(&document, url);

        // Imágenes
        let images = self.extract_images(&document);

        // Scripts & Stylesheets
        let scripts = self.extract_scripts(&document);
        let stylesheets = self.extract_stylesheets(&document);

        // Headings hierarchy
        let headings = self.extract_headings(&document);

        // Formularios detectados
        let forms = self.extract_forms(&document);

        // Social media links
        let social = self.extract_social(&html_text);

        // Custom selectors si se proporcionan
        let custom_data = if let Some(selectors) = custom_selectors {
            self.extract_custom(&document, selectors)
        } else {
            HashMap::new()
        };

        Ok(json!({
            "url": url,
            "status_code": status_code,
            "title": title,
            "description": description,
            "keywords": keywords,
            "emails": emails,
            "phones": phones,
            "links_count": links.len(),
            "links": links,
            "images_count": images.len(),
            "images": images,
            "scripts": scripts,
            "stylesheets": stylesheets,
            "headings": headings,
            "forms": forms,
            "social_media": social,
            "custom_data": custom_data,
            "scraped_at": chrono::Utc::now().to_rfc3339()
        }))
    }

    fn extract_title(&self, document: &Html) -> String {
        document
            .select(&Selector::parse("title").unwrap())
            .next()
            .map(|el| el.text().collect::<Vec<_>>().join(""))
            .unwrap_or_default()
    }

    fn extract_meta(&self, document: &Html, attr: &str) -> String {
        document
            .select(&Selector::parse("meta").unwrap())
            .find(|el| {
                el.value().attr("name").map_or(false, |n| n == attr)
                    || el.value().attr("property").map_or(false, |p| p == attr)
            })
            .and_then(|el| el.value().attr("content"))
            .unwrap_or_default()
            .to_string()
    }

    fn extract_emails(&self, html: &str) -> Vec<String> {
        let email_regex =
            Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap();
        email_regex
            .find_iter(html)
            .map(|m| m.as_str().to_string())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }

    fn extract_phones(&self, html: &str) -> Vec<String> {
        let phone_regex = Regex::new(r"\+?[\d\s\-\(\)]{10,}").unwrap();
        phone_regex
            .find_iter(html)
            .map(|m| m.as_str().trim().to_string())
            .filter(|p| p.len() >= 10 && p.chars().filter(|c| c.is_ascii_digit()).count() >= 7)
            .collect::<HashSet<_>>()
            .into_iter()
            .take(50)
            .collect()
    }

    fn extract_links(&self, document: &Html, base_url: &str) -> Vec<Value> {
        document
            .select(&Selector::parse("a[href]").unwrap())
            .take(100)
            .filter_map(|el| {
                el.value().attr("href").map(|href| {
                    let text = el.text().collect::<Vec<_>>().join(" ").trim().to_string();
                    json!({
                        "href": href,
                        "text": text,
                        "is_internal": href.starts_with("/") || href.contains(&base_url.split('/').nth(2).unwrap_or(""))
                    })
                })
            })
            .collect()
    }

    fn extract_images(&self, document: &Html) -> Vec<Value> {
        document
            .select(&Selector::parse("img").unwrap())
            .take(50)
            .filter_map(|el| {
                let src = el.value().attr("src")?;
                let alt = el.value().attr("alt").unwrap_or("");
                Some(json!({
                    "src": src,
                    "alt": alt
                }))
            })
            .collect()
    }

    fn extract_scripts(&self, document: &Html) -> Vec<String> {
        document
            .select(&Selector::parse("script[src]").unwrap())
            .filter_map(|el| el.value().attr("src").map(|s| s.to_string()))
            .collect()
    }

    fn extract_stylesheets(&self, document: &Html) -> Vec<String> {
        document
            .select(&Selector::parse("link[rel='stylesheet']").unwrap())
            .filter_map(|el| el.value().attr("href").map(|h| h.to_string()))
            .collect()
    }

    fn extract_headings(&self, document: &Html) -> Vec<Value> {
        let mut headings = Vec::new();
        for h in ["h1", "h2", "h3", "h4", "h5", "h6"] {
            if let Ok(selector) = Selector::parse(h) {
                for el in document.select(&selector).take(20) {
                    let text = el.text().collect::<Vec<_>>().join(" ").trim().to_string();
                    if !text.is_empty() {
                        headings.push(json!({
                            "level": h,
                            "text": text
                        }));
                    }
                }
            }
        }
        headings
    }

    fn extract_forms(&self, document: &Html) -> Vec<Value> {
        document
            .select(&Selector::parse("form").unwrap())
            .take(10)
            .map(|form| {
                let action = form.value().attr("action").unwrap_or("").to_string();
                let method = form.value().attr("method").unwrap_or("get").to_string();

                let mut inputs = Vec::new();
                if let Ok(input_selector) = Selector::parse("input, select, textarea") {
                    for input in form.select(&input_selector).take(20) {
                        let name = input.value().attr("name").unwrap_or("").to_string();
                        let input_type = input.value().attr("type").unwrap_or("text").to_string();
                        if !name.is_empty() {
                            inputs.push(json!({
                                "name": name,
                                "type": input_type
                            }));
                        }
                    }
                }

                json!({
                    "action": action,
                    "method": method,
                    "fields": inputs
                })
            })
            .collect()
    }

    fn extract_social(&self, html: &str) -> Vec<Value> {
        let mut social = Vec::new();
        let platforms = vec![
            ("facebook", r"facebook\.com/[\w-]+"),
            ("twitter", r"twitter\.com/[\w]+"),
            ("linkedin", r"linkedin\.com/(company|in)/[\w-]+"),
            ("instagram", r"instagram\.com/[\w.]+"),
            ("youtube", r"youtube\.com/(c|channel|user)/[\w-]+"),
        ];

        for (platform, pattern) in platforms {
            if let Ok(regex) = Regex::new(pattern) {
                if let Some(m) = regex.find(html) {
                    social.push(json!({
                        "platform": platform,
                        "url": m.as_str()
                    }));
                }
            }
        }
        social
    }

    fn extract_custom(&self, document: &Html, selectors: Vec<String>) -> HashMap<String, Vec<String>> {
        let mut result = HashMap::new();
        for selector_str in selectors {
            if let Ok(selector) = Selector::parse(&selector_str) {
                let elements: Vec<String> = document
                    .select(&selector)
                    .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                if !elements.is_empty() {
                    result.insert(selector_str.clone(), elements);
                }
            }
        }
        result
    }
}

impl Default for MarketingScraper {
    fn default() -> Self {
        Self::new()
    }
}
