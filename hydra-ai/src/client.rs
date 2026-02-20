use std::time::Duration;
use serde::{Deserialize, Serialize};
use tokio::time::timeout;
use tracing::{debug, error, info};

use hydra_core::{AiConfig, HydraError};
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

    pub async fn analyze(&self, prompt: &AnalysisPrompt) -> Result<AiAnalysis, HydraError> {
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

        debug!("Sending AI analysis request");

        let duration = Duration::from_secs(self.config.timeout_secs);
        let response = timeout(duration, self.send_request(&request))
            .await
            .map_err(|_| HydraError::AiService("Request timed out".to_string()))?
            .map_err(|e| HydraError::AiService(e.to_string()))?;

        info!("AI analysis completed");
        Ok(response)
    }

    async fn send_request(&self, request: &DeepSeekRequest) -> anyhow::Result<AiAnalysis> {
        let resp = self
            .client
            .post("https://api.deepseek.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(request)
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

    #[test]
    fn test_client_creation() {
        let config = AiConfig {
            model: "deepseek-chat".to_string(),
            max_tokens: 1024,
            temperature: 0.1,
            timeout_secs: 30,
        };
        let client = DeepSeekClient::new("test-key", config);
        assert_eq!(client.api_key, "test-key");
    }
}
