use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{info, warn};

use hydra_core::signal::MintSignal;

const DEEPSEEK_TIMEOUT_MS: u64 = 800;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiScore {
    pub confidence: f64,
    pub reasoning: String,
    pub should_buy: bool,
}

#[derive(Debug, Serialize)]
struct DeepSeekRequest {
    model: String,
    messages: Vec<DeepSeekMessage>,
    temperature: f64,
}

#[derive(Debug, Serialize)]
struct DeepSeekMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct DeepSeekResponse {
    choices: Vec<DeepSeekChoice>,
}

#[derive(Debug, Deserialize)]
struct DeepSeekChoice {
    message: DeepSeekResponseMessage,
}

#[derive(Debug, Deserialize)]
struct DeepSeekResponseMessage {
    content: String,
}

pub struct DeepSeekScorer {
    client: reqwest::Client,
    api_url: String,
    model: String,
    api_key: String,
}

impl DeepSeekScorer {
    pub fn new(api_url: String, model: String, api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_url,
            model,
            api_key,
        }
    }

    pub async fn score(&self, signal: &MintSignal) -> Result<AiScore> {
        let prompt = format!(
            "Analyze this token signal and respond with JSON {{\"confidence\": 0.0-1.0, \"reasoning\": \"...\", \"should_buy\": true/false}}:\n\
             mint={}, mcap_usd={:.2}, volume_24h={:.2}, price={:.6}, holders={}, liquidity={:.2}, top_holder_pct={:.2}",
            signal.mint_address,
            signal.market_cap_usd,
            signal.volume_24h_usd,
            signal.price_usd,
            signal.holder_count,
            signal.liquidity_usd,
            signal.top_holder_pct,
        );

        let request = DeepSeekRequest {
            model: self.model.clone(),
            messages: vec![DeepSeekMessage {
                role: "user".to_string(),
                content: prompt,
            }],
            temperature: 0.1,
        };

        let url = format!("{}/v1/chat/completions", self.api_url.trim_end_matches('/'));

        let response = tokio::time::timeout(
            Duration::from_millis(DEEPSEEK_TIMEOUT_MS),
            self.client
                .post(&url)
                .bearer_auth(&self.api_key)
                .json(&request)
                .send(),
        )
        .await
        .context("DeepSeek API request timed out")?
        .context("DeepSeek API request failed")?;

        if !response.status().is_success() {
            warn!(status = %response.status(), "DeepSeek API returned non-success status");
            return Ok(AiScore {
                confidence: 0.0,
                reasoning: format!("API error: {}", response.status()),
                should_buy: false,
            });
        }

        let ds_response: DeepSeekResponse = response
            .json()
            .await
            .context("Failed to deserialize DeepSeek response")?;

        let content = ds_response
            .choices
            .first()
            .map(|c| c.message.content.as_str())
            .unwrap_or("");

        let score = parse_ai_response(content).unwrap_or_else(|e| {
            warn!(error = %e, "Failed to parse AI response, using default score");
            AiScore {
                confidence: 0.0,
                reasoning: format!("Parse error: {e}"),
                should_buy: false,
            }
        });

        info!(
            mint = %signal.mint_address,
            confidence = score.confidence,
            should_buy = score.should_buy,
            "AI scoring complete"
        );

        Ok(score)
    }
}

fn parse_ai_response(content: &str) -> Result<AiScore> {
    // Extract the JSON object from the response
    let start = content
        .find('{')
        .ok_or_else(|| anyhow::anyhow!("No JSON object found in AI response"))?;
    let end = content
        .rfind('}')
        .map(|i| i + 1)
        .ok_or_else(|| anyhow::anyhow!("Unterminated JSON object in AI response"))?;
    let json_str = &content[start..end];
    let score: AiScore = serde_json::from_str(json_str).context("Failed to parse AI JSON")?;
    Ok(score)
}
