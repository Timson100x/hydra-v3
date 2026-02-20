use anyhow::Result;
use reqwest::Client;
use std::time::Duration;
use tracing::{info, warn};

pub struct TelegramAlerter {
    client: Client,
    bot_token: String,
    chat_id: String,
}

impl TelegramAlerter {
    pub fn new(bot_token: String, chat_id: String) -> Self {
        Self {
            client: Client::new(),
            bot_token,
            chat_id,
        }
    }

    pub async fn send_alert(&self, message: &str) -> Result<()> {
        let url = format!(
            "https://api.telegram.org/bot{}/sendMessage",
            self.bot_token
        );

        let body = serde_json::json!({
            "chat_id": self.chat_id,
            "text": message,
            "parse_mode": "Markdown"
        });

        let response = tokio::time::timeout(
            Duration::from_secs(10),
            self.client.post(&url).json(&body).send(),
        )
        .await
        .map_err(|_| anyhow::anyhow!("Telegram API request timed out"))?
        .map_err(|e| anyhow::anyhow!("Telegram API request failed: {e}"))?;

        if response.status().is_success() {
            info!("Telegram alert sent successfully");
        } else {
            warn!(status = %response.status(), "Telegram API returned non-success status");
        }

        Ok(())
    }
}
