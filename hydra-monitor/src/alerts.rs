use std::time::Duration;
use serde::{Deserialize, Serialize};
use tokio::time::timeout;
use tracing::{info, warn, error};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub level: AlertLevel,
    pub message: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertLevel {
    Info,
    Warning,
    Critical,
}

impl std::fmt::Display for AlertLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlertLevel::Info => write!(f, "INFO"),
            AlertLevel::Warning => write!(f, "WARNING"),
            AlertLevel::Critical => write!(f, "CRITICAL"),
        }
    }
}

pub struct AlertManager {
    webhook_url: Option<String>,
    client: reqwest::Client,
}

impl AlertManager {
    pub fn new(webhook_url: Option<String>) -> Self {
        Self {
            webhook_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn send(&self, alert: Alert) -> Result<()> {
        match alert.level {
            AlertLevel::Info => info!("[ALERT] {}: {}", alert.source, alert.message),
            AlertLevel::Warning => warn!("[ALERT] {}: {}", alert.source, alert.message),
            AlertLevel::Critical => error!("[ALERT] {}: {}", alert.source, alert.message),
        }

        if let Some(url) = &self.webhook_url {
            let payload = serde_json::json!({
                "text": format!("[{}] {}: {}", alert.level, alert.source, alert.message)
            });

            timeout(
                Duration::from_secs(10),
                self.client.post(url).json(&payload).send(),
            )
            .await
            .map_err(|_| anyhow::anyhow!("Webhook timeout"))?
            .map_err(|e| anyhow::anyhow!("Webhook error: {}", e))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_alert_no_webhook() {
        let manager = AlertManager::new(None);
        let alert = Alert {
            level: AlertLevel::Info,
            message: "Test alert".to_string(),
            source: "test".to_string(),
        };
        assert!(manager.send(alert).await.is_ok());
    }
}
