use anyhow::{Context, Result};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct HydraConfig {
    pub deepseek_api_url: String,
    pub deepseek_model: String,
    pub telegram_bot_token: Option<String>,
    pub telegram_chat_id: Option<String>,
    pub prometheus_port: u16,
    pub max_daily_loss_sol: f64,
    pub max_open_positions: usize,
    pub trade_journal_path: String,
}

impl HydraConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            deepseek_api_url: std::env::var("DEEPSEEK_API_URL")
                .unwrap_or_else(|_| "https://api.deepseek.com".to_string()),
            deepseek_model: std::env::var("DEEPSEEK_MODEL")
                .unwrap_or_else(|_| "deepseek-chat".to_string()),
            telegram_bot_token: std::env::var("TELEGRAM_BOT_TOKEN").ok(),
            telegram_chat_id: std::env::var("TELEGRAM_CHAT_ID").ok(),
            prometheus_port: std::env::var("PROMETHEUS_PORT")
                .unwrap_or_else(|_| "9001".to_string())
                .parse::<u16>()
                .context("PROMETHEUS_PORT must be a valid port number")?,
            max_daily_loss_sol: std::env::var("MAX_DAILY_LOSS_SOL")
                .unwrap_or_else(|_| "1.0".to_string())
                .parse::<f64>()
                .context("MAX_DAILY_LOSS_SOL must be a valid float")?,
            max_open_positions: std::env::var("MAX_OPEN_POSITIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse::<usize>()
                .context("MAX_OPEN_POSITIONS must be a valid integer")?,
            trade_journal_path: std::env::var("TRADE_JOURNAL_PATH")
                .unwrap_or_else(|_| "logs/trades.csv".to_string()),
        })
    }
}
