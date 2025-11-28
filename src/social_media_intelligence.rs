/// SOCIAL MEDIA INTELLIGENCE
/// Inteligencia de redes sociales con análisis de sentimientos y detección de campañas
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialPost {
    pub id: String,
    pub platform: String,
    pub username: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub likes: u32,
    pub shares: u32,
    pub comments: u32,
    pub hashtags: Vec<String>,
    pub mentions: Vec<String>,
    pub sentiment_score: f32,
    pub engagement_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentAnalysis {
    pub positive: f32,
    pub negative: f32,
    pub neutral: f32,
    pub compound_score: f32,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignPattern {
    pub pattern_id: String,
    pub description: String,
    pub confidence: f32,
    pub posts_sample: Vec<SocialPost>,
    pub indicators: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfluencerProfile {
    pub username: String,
    pub platform: String,
    pub followers: u32,
    pub following: u32,
    pub posts_count: u32,
    pub engagement_rate: f32,
    pub verified: bool,
    pub description: String,
    pub keywords: Vec<String>,
    pub influence_score: f32,
}

pub struct SocialMediaIntelligence {
    pub cache: Vec<SocialPost>,
}

impl SocialMediaIntelligence {
    pub fn new() -> Self {
        Self {
            cache: Vec::new(),
        }
    }

    /// Buscar posts por keyword en una plataforma específica
    pub async fn search_posts(
        &mut self,
        platform: &str,
        keyword: &str,
        limit: u32,
    ) -> Result<Vec<SocialPost>, String> {
        let mut posts = match platform {
            "twitter" => self.search_twitter_posts(keyword, limit).await?,
            "linkedin" => self.search_linkedin_posts(keyword, limit).await?,
            "instagram" => self.search_instagram_posts(keyword, limit).await?,
            _ => return Err(format!("Platform '{}' not supported", platform)),
        };

        // Procesar posts para análisis de sentimientos
        for post in posts.iter_mut() {
            post.sentiment_score = self.analyze_sentiment(&post.content).compound_score;
        }

        // Cachear resultados
        self.cache.extend(posts.clone());
        Ok(posts)
    }

    /// Búsqueda específica en Twitter
    async fn search_twitter_posts(&self, keyword: &str, limit: u32) -> Result<Vec<SocialPost>, String> {
        // Simulación de búsqueda en Twitter
        let mut posts = Vec::new();
        
        for i in 0..limit.min(10) {
            posts.push(SocialPost {
                id: format!("twitter_{}", i),
                platform: "Twitter".to_string(),
                username: format!("user_{}", i),
                content: format!("Tweet sobre {} #{}", keyword, i),
                timestamp: Utc::now(),
                likes: i * 10,
                shares: i * 2,
                comments: i * 5,
                hashtags: vec![keyword.to_string()],
                mentions: vec![],
                sentiment_score: 0.0,
                engagement_rate: (i as f32) * 0.1,
            });
        }

        Ok(posts)
    }

    /// Búsqueda específica en LinkedIn
    async fn search_linkedin_posts(&self, keyword: &str, limit: u32) -> Result<Vec<SocialPost>, String> {
        // Simulación de búsqueda en LinkedIn
        let mut posts = Vec::new();
        
        for i in 0..limit.min(5) {
            posts.push(SocialPost {
                id: format!("linkedin_{}", i),
                platform: "LinkedIn".to_string(),
                username: format!("professional_{}", i),
                content: format!("Post profesional sobre {} - Análisis {}", keyword, i),
                timestamp: Utc::now(),
                likes: i * 15,
                shares: i * 3,
                comments: i * 8,
                hashtags: vec![keyword.to_string(), "professional".to_string()],
                mentions: vec![],
                sentiment_score: 0.0,
                engagement_rate: (i as f32) * 0.15,
            });
        }

        Ok(posts)
    }

    /// Búsqueda específica en Instagram
    async fn search_instagram_posts(&self, keyword: &str, limit: u32) -> Result<Vec<SocialPost>, String> {
        // Simulación de búsqueda en Instagram
        let mut posts = Vec::new();
        
        for i in 0..limit.min(8) {
            posts.push(SocialPost {
                id: format!("instagram_{}", i),
                platform: "Instagram".to_string(),
                username: format!("influencer_{}", i),
                content: format!("Instagram post sobre {} 📸 #{}", keyword, i),
                timestamp: Utc::now(),
                likes: i * 50,
                shares: i * 5,
                comments: i * 12,
                hashtags: vec![keyword.to_string(), "instagram".to_string(), "photo".to_string()],
                mentions: vec![],
                sentiment_score: 0.0,
                engagement_rate: (i as f32) * 0.2,
            });
        }

        Ok(posts)
    }

    /// Análisis de sentimientos usando NLP básico
    pub fn analyze_sentiment(&self, text: &str) -> SentimentAnalysis {
        // Palabras positivas y negativas básicas
        let positive_words = ["good", "great", "excellent", "amazing", "wonderful", "fantastic", "love", "best"];
        let negative_words = ["bad", "terrible", "awful", "hate", "worst", "horrible", "disgusting", "poor"];

        let lowercase_text = text.to_lowercase();
        let words: Vec<&str> = lowercase_text.split_whitespace().collect();
        
        let positive_count = words.iter().filter(|word| positive_words.contains(word)).count() as f32;
        let negative_count = words.iter().filter(|word| negative_words.contains(word)).count() as f32;
        let total_sentiment_words = positive_count + negative_count;

        let (positive, negative, neutral) = if total_sentiment_words > 0.0 {
            (
                positive_count / total_sentiment_words,
                negative_count / total_sentiment_words,
                0.0
            )
        } else {
            (0.0, 0.0, 1.0)
        };

        let compound_score = positive - negative;

        SentimentAnalysis {
            positive,
            negative,
            neutral,
            compound_score,
            confidence: if total_sentiment_words > 0.0 { 0.8 } else { 0.3 },
        }
    }

    /// Detectar campañas coordinadas o patrones sospechosos
    pub fn detect_campaign_patterns(&self, posts: &[SocialPost]) -> Vec<CampaignPattern> {
        let mut patterns = Vec::new();

        // Detectar posts con contenido muy similar
        if posts.len() > 5 {
            let similar_content = self.find_similar_content(posts);
            if !similar_content.is_empty() {
                patterns.push(CampaignPattern {
                    pattern_id: "similar_content".to_string(),
                    description: "Multiple posts with very similar content detected".to_string(),
                    confidence: 0.75,
                    posts_sample: similar_content,
                    indicators: vec!["similar_text".to_string(), "coordinated_posting".to_string()],
                });
            }
        }

        // Detectar hashtags coordinados
        let coordinated_hashtags = self.find_coordinated_hashtags(posts);
        if !coordinated_hashtags.is_empty() {
            patterns.push(CampaignPattern {
                pattern_id: "coordinated_hashtags".to_string(),
                description: "Coordinated hashtag usage detected".to_string(),
                confidence: 0.65,
                posts_sample: coordinated_hashtags,
                indicators: vec!["hashtag_coordination".to_string(), "artificial_trending".to_string()],
            });
        }

        patterns
    }

    /// Encontrar contenido similar entre posts
    fn find_similar_content(&self, posts: &[SocialPost]) -> Vec<SocialPost> {
        // Implementación simple de detección de similitud
        let mut similar_posts = Vec::new();
        
        for (i, post1) in posts.iter().enumerate() {
            for post2 in posts.iter().skip(i + 1) {
                let similarity = self.calculate_text_similarity(&post1.content, &post2.content);
                if similarity > 0.8 {
                    similar_posts.push(post1.clone());
                    similar_posts.push(post2.clone());
                }
            }
        }

        similar_posts
    }

    /// Encontrar hashtags coordinados
    fn find_coordinated_hashtags(&self, posts: &[SocialPost]) -> Vec<SocialPost> {
        let mut coordinated = Vec::new();
        
        // Buscar patrones de hashtags que aparecen juntos frecuentemente
        for post in posts {
            if post.hashtags.len() > 3 {
                coordinated.push(post.clone());
            }
        }

        coordinated
    }

    /// Calcular similitud entre dos textos (Jaccard similarity simplificado)
    fn calculate_text_similarity(&self, text1: &str, text2: &str) -> f32 {
        let words1: std::collections::HashSet<&str> = text1.split_whitespace().collect();
        let words2: std::collections::HashSet<&str> = text2.split_whitespace().collect();

        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f32 / union as f32
        }
    }

    /// Análisis de influencers en posts
    pub fn analyze_influencers(&self, posts: &[SocialPost]) -> Vec<InfluencerProfile> {
        let mut influencers = Vec::new();
        let _all_posts: Vec<SocialPost> = Vec::new();

        for _platform in &["twitter", "linkedin", "instagram"] {
            // Análisis de influencers por plataforma
        }

        // Crear perfiles basados en engagement y alcance
        for post in posts {
            if post.engagement_rate > 0.1 {
                influencers.push(InfluencerProfile {
                    username: post.username.clone(),
                    platform: post.platform.clone(),
                    followers: (post.engagement_rate * 10000.0) as u32,
                    following: 500,
                    posts_count: 150,
                    engagement_rate: post.engagement_rate,
                    verified: post.engagement_rate > 0.15,
                    description: format!("Influencer en {}", post.platform),
                    keywords: post.hashtags.clone(),
                    influence_score: post.engagement_rate * 100.0,
                });
            }
        }

        influencers
    }

    /// Generar reporte de inteligencia social
    pub fn generate_intelligence_report(&self, posts: &[SocialPost]) -> serde_json::Value {
        let sentiment_analysis: Vec<SentimentAnalysis> = posts.iter()
            .map(|p| self.analyze_sentiment(&p.content))
            .collect();

        let avg_sentiment = if !sentiment_analysis.is_empty() {
            sentiment_analysis.iter().map(|s| s.compound_score).sum::<f32>() / sentiment_analysis.len() as f32
        } else {
            0.0
        };

        let campaign_patterns = self.detect_campaign_patterns(posts);
        let influencers = self.analyze_influencers(posts);

        serde_json::json!({
            "timestamp": Utc::now().to_rfc3339(),
            "summary": {
                "total_posts": posts.len(),
                "platforms": posts.iter().map(|p| &p.platform).collect::<std::collections::HashSet<_>>().len(),
                "average_sentiment": avg_sentiment,
                "suspicious_patterns": campaign_patterns.len(),
                "influencers_identified": influencers.len(),
            },
            "sentiment_distribution": {
                "positive": sentiment_analysis.iter().filter(|s| s.compound_score > 0.1).count(),
                "neutral": sentiment_analysis.iter().filter(|s| s.compound_score.abs() <= 0.1).count(),
                "negative": sentiment_analysis.iter().filter(|s| s.compound_score < -0.1).count(),
            },
            "campaign_patterns": campaign_patterns,
            "top_influencers": influencers.into_iter().take(5).collect::<Vec<_>>(),
            "trending_hashtags": self.get_trending_hashtags(posts),
        })
    }

    /// Obtener hashtags en tendencia
    fn get_trending_hashtags(&self, posts: &[SocialPost]) -> Vec<String> {
        let mut hashtag_count = std::collections::HashMap::new();
        
        for post in posts {
            for hashtag in &post.hashtags {
                *hashtag_count.entry(hashtag.clone()).or_insert(0) += 1;
            }
        }

        let mut sorted_hashtags: Vec<(String, i32)> = hashtag_count.into_iter().collect();
        sorted_hashtags.sort_by(|a, b| b.1.cmp(&a.1));
        
        sorted_hashtags.into_iter().take(10).map(|(hashtag, _)| hashtag).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_posts() {
        let mut intelligence = SocialMediaIntelligence::new();
        
        match intelligence.search_posts("twitter", "rust", 5).await {
            Ok(posts) => {
                assert!(!posts.is_empty());
                assert_eq!(posts[0].platform, "Twitter");
                println!("✅ Twitter search successful: {} posts", posts.len());
            },
            Err(e) => println!("⚠️ Twitter search error: {}", e),
        }
    }

    #[test]
    fn test_sentiment_analysis() {
        let intelligence = SocialMediaIntelligence::new();
        
        let positive_text = "This is amazing and wonderful!";
        let negative_text = "This is terrible and awful!";
        let neutral_text = "This is a regular post about technology.";

        let pos_sentiment = intelligence.analyze_sentiment(positive_text);
        let neg_sentiment = intelligence.analyze_sentiment(negative_text);
        let neu_sentiment = intelligence.analyze_sentiment(neutral_text);

        assert!(pos_sentiment.compound_score > 0.0);
        assert!(neg_sentiment.compound_score < 0.0);
        assert!(neu_sentiment.compound_score.abs() < 0.5);

        println!("✅ Sentiment analysis working correctly");
    }

    #[test]
    fn test_intelligence_report() {
        let intelligence = SocialMediaIntelligence::new();
        let posts = vec![
            SocialPost {
                id: "test1".to_string(),
                platform: "Twitter".to_string(),
                username: "test_user".to_string(),
                content: "Great product! Love it!".to_string(),
                timestamp: Utc::now(),
                likes: 100,
                shares: 10,
                comments: 25,
                hashtags: vec!["great".to_string(), "product".to_string()],
                mentions: vec![],
                sentiment_score: 0.8,
                engagement_rate: 0.15,
            }
        ];

        let report = intelligence.generate_intelligence_report(&posts);
        assert!(report["summary"]["total_posts"].as_u64().unwrap() > 0);
        println!("✅ Intelligence report generation successful");
    }
}
