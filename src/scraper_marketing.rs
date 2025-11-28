/// Web Scraper Marketing Edition
/// Extrae datos REALES con emails, phones, links, imágenes, meta tags, etc.
/// Production-ready para DB y análisis

use reqwest::Client;
use scraper::{Html, Selector};
use regex::Regex;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

/// Scrape URL completa con extracción profunda
pub async fn scrape_url_full(
    url: &str,
    selectors: &[String],
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .danger_accept_invalid_certs(true)
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let response = client.get(url).send().await?;
    let status_code = response.status().as_u16();
    let html = response.text().await?;
    let document = Html::parse_document(&html);

    // Extraer por selectores custom
    let mut scraped_data: HashMap<String, Vec<String>> = HashMap::new();
    for selector_str in selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            let elements: Vec<String> = document
                .select(&selector)
                .map(|el| {
                    let text = el.text().collect::<Vec<_>>().join(" ").trim().to_string();
                    let href = el
                        .value()
                        .attr("href")
                        .map(|h| format!(" [href={}]", h))
                        .unwrap_or_default();
                    let src = el
                        .value()
                        .attr("src")
                        .map(|s| format!(" [src={}]", s))
                        .unwrap_or_default();
                    format!("{}{}{}", text, href, src)
                })
                .filter(|s| !s.is_empty())
                .collect();
            if !elements.is_empty() {
                scraped_data.insert(selector_str.clone(), elements);
            }
        }
    }

    // Meta tags
    let mut meta_tags: HashMap<String, String> = HashMap::new();
    if let Ok(meta_selector) = Selector::parse("meta") {
        for element in document.select(&meta_selector) {
            let name = element
                .value()
                .attr("name")
                .or_else(|| element.value().attr("property"))
                .unwrap_or("unknown");
            if let Some(content) = element.value().attr("content") {
                meta_tags.insert(name.to_string(), content.to_string());
            }
        }
    }

    // Links
    let mut links: Vec<Value> = Vec::new();
    if let Ok(link_selector) = Selector::parse("a[href]") {
        for element in document.select(&link_selector) {
            if let Some(href) = element.value().attr("href") {
                let text = element.text().collect::<Vec<_>>().join(" ").trim().to_string();
                links.push(json!({"href": href, "text": text}));
            }
        }
    }

    // Imágenes
    let mut images: Vec<Value> = Vec::new();
    if let Ok(img_selector) = Selector::parse("img") {
        for element in document.select(&img_selector) {
            let src = element.value().attr("src").unwrap_or("");
            let alt = element.value().attr("alt").unwrap_or("");
            if !src.is_empty() {
                images.push(json!({"src": src, "alt": alt}));
            }
        }
    }

    // Scripts
    let mut scripts: Vec<String> = Vec::new();
    if let Ok(script_selector) = Selector::parse("script[src]") {
        for element in document.select(&script_selector) {
            if let Some(src) = element.value().attr("src") {
                scripts.push(src.to_string());
            }
        }
    }

    // Stylesheets
    let mut stylesheets: Vec<String> = Vec::new();
    if let Ok(css_selector) = Selector::parse("link[rel='stylesheet']") {
        for element in document.select(&css_selector) {
            if let Some(href) = element.value().attr("href") {
                stylesheets.push(href.to_string());
            }
        }
    }

    // Emails con regex
    let email_regex = Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}")?;
    let emails: Vec<String> = email_regex
        .find_iter(&html)
        .map(|m| m.as_str().to_string())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    // Teléfonos con regex
    let phone_regex = Regex::new(r"\+?[\d\s\-\(\)]{10,}")?;
    let phones: Vec<String> = phone_regex
        .find_iter(&html)
        .map(|m| m.as_str().trim().to_string())
        .filter(|p| p.len() >= 10 && p.chars().filter(|c| c.is_ascii_digit()).count() >= 7)
        .collect::<HashSet<_>>()
        .into_iter()
        .take(20)
        .collect();

    // Título
    let title = if let Ok(title_selector) = Selector::parse("title") {
        document
            .select(&title_selector)
            .next()
            .map(|el| el.text().collect::<Vec<_>>().join(""))
            .unwrap_or_default()
    } else {
        String::new()
    };

    // Body text completo
    let body_text = if let Ok(body_selector) = Selector::parse("body") {
        document
            .select(&body_selector)
            .next()
            .map(|el| {
                el.text()
                    .collect::<Vec<_>>()
                    .join(" ")
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .unwrap_or_default()
    } else {
        String::new()
    };

    // Headings
    let mut headings: Vec<Value> = Vec::new();
    for h in &["h1", "h2", "h3", "h4", "h5", "h6"] {
        if let Ok(h_selector) = Selector::parse(h) {
            for element in document.select(&h_selector) {
                let text = element.text().collect::<Vec<_>>().join(" ").trim().to_string();
                if !text.is_empty() {
                    headings.push(json!({"level": h, "text": text}));
                }
            }
        }
    }

    // Forms
    let mut forms: Vec<Value> = Vec::new();
    if let Ok(form_selector) = Selector::parse("form") {
        for form in document.select(&form_selector) {
            let action = form.value().attr("action").unwrap_or("");
            let method = form.value().attr("method").unwrap_or("get");

            let mut inputs: Vec<Value> = Vec::new();
            if let Ok(input_selector) = Selector::parse("input, select, textarea") {
                for input in form.select(&input_selector) {
                    let name = input.value().attr("name").unwrap_or("");
                    let type_attr = input.value().attr("type").unwrap_or("text");
                    inputs.push(json!({"name": name, "type": type_attr}));
                }
            }

            forms.push(json!({
                "action": action,
                "method": method,
                "inputs": inputs
            }));
        }
    }

    Ok(json!({
        "url": url,
        "status_code": status_code,
        "title": title,
        "meta_tags": meta_tags,
        "headings": headings,
        "links": links,
        "images": images,
        "scripts": scripts,
        "stylesheets": stylesheets,
        "forms": forms,
        "emails": emails,
        "phones": phones,
        "custom_selectors": scraped_data,
        "body_preview": if body_text.len() > 500 { format!("{}...", &body_text[..500]) } else { body_text },
        "total_elements": links.len() + images.len() + forms.len()
    }))
}

/// Scrape múltiples URLs en paralelo
pub async fn scrape_batch(
    urls: Vec<String>,
    selectors: Vec<String>,
) -> Vec<Value> {
    let mut results = Vec::new();
    for url in urls {
        match scrape_url_full(&url, &selectors).await {
            Ok(data) => results.push(data),
            Err(e) => results.push(json!({"url": url, "error": e.to_string()})),
        }
    }
    results
}
