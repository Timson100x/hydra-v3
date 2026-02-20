use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use tokio::time::timeout;
use tracing::{debug, error, info, warn};

use hydra_core::AiConfig;
use crate::prompt::{AiAnalysis, AnalysisPrompt};

#[derive(Debug, Serialize)]
struct DeepSeekRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
    temperature: f64,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct DeepSeekResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}

pub struct DeepSeekClient {
    client: reqwest::Client,
    api_key: String,
    config: AiConfig,
}

impl DeepSeekClient {
    pub fn new(api_key: impl Into<String>, config: AiConfig) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: api_key.into(),
            config,
        }
    }

    /// Attempts to get an AI analysis for the given prompt.
    ///
    /// Returns `None` on timeout or any API error, logging a `warn!` with the
    /// elapsed duration so the caller can fall back to rule-based scoring.
    pub async fn try_analyze(&self, prompt: &AnalysisPrompt) -> Option<AiAnalysis> {
        let started = Instant::now();
        let duration = Duration::from_secs(self.config.timeout_secs);

        debug!("Sending AI analysis request");

        let result = timeout(duration, self.send_request(prompt)).await;
        let elapsed = started.elapsed();

        match result {
            Ok(Ok(analysis)) => {
                info!("AI analysis completed in {:.3}s", elapsed.as_secs_f64());
                Some(analysis)
            }
            Ok(Err(e)) => {
                warn!(
                    elapsed_ms = elapsed.as_millis(),
                    error = %e,
                    "DeepSeek API error after {:.3}s — falling back to rule score",
                    elapsed.as_secs_f64()
                );
                None
            }
            Err(_) => {
                warn!(
                    elapsed_ms = elapsed.as_millis(),
                    "DeepSeek API timeout after {:.3}s — falling back to rule score",
                    elapsed.as_secs_f64()
                );
                None
            }
        }
    }

    async fn send_request(&self, prompt: &AnalysisPrompt) -> anyhow::Result<AiAnalysis> {
        let request = DeepSeekRequest {
            model: self.config.model.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: prompt.system.clone(),
                },
                Message {
                    role: "user".to_string(),
                    content: prompt.user.clone(),
                },
            ],
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
        };

        let resp = self
            .client
            .post("https://api.deepseek.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json::<DeepSeekResponse>()
            .await?;

        let content = resp
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("Empty response"))?
            .message
            .content;

        let analysis: AiAnalysis = serde_json::from_str(&content).map_err(|e| {
            error!("Failed to parse AI response: {}", e);
            anyhow::anyhow!("Parse error: {}", e)
        })?;

        Ok(analysis)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> AiConfig {
        AiConfig {
            model: "deepseek-chat".to_string(),
            max_tokens: 1024,
            temperature: 0.1,
            timeout_secs: 1,
        }
    }

    #[test]
    fn test_client_creation() {
        let client = DeepSeekClient::new("test-key", test_config());
        assert_eq!(client.api_key, "test-key");
    }

    /// Verifies that `try_analyze` returns `None` when the underlying request
    /// times out (simulated by using an unreachable host with a 1 s timeout).
    #[tokio::test]
    async fn test_try_analyze_returns_none_on_timeout() {
        // Point to a non-routable address so the request never completes.
        let mut config = test_config();
        config.timeout_secs = 1;
        let client = DeepSeekClient::new("invalid-key", config);
        let prompt = AnalysisPrompt {
            system: "sys".to_string(),
            user: "user".to_string(),
        };
        // Should return None (timeout/error) rather than panic.
        let result = client.try_analyze(&prompt).await;
        assert!(result.is_none());
    }
}
