use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HydraConfig {
    pub network: NetworkConfig,
    pub risk: RiskConfig,
    pub ai: AiConfig,
    pub monitoring: MonitoringConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NetworkConfig {
    pub rpc_url: String,
    pub wss_url: String,
    pub commitment: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RiskConfig {
    pub max_position_size_sol: f64,
    pub max_daily_loss_sol: f64,
    pub circuit_breaker_threshold: f64,
    pub min_liquidity_usd: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AiConfig {
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f64,
    pub timeout_secs: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MonitoringConfig {
    pub prometheus_port: u16,
    pub alert_webhook_url: Option<String>,
}

impl Default for HydraConfig {
    fn default() -> Self {
        Self {
            network: NetworkConfig {
                rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
                wss_url: "wss://api.mainnet-beta.solana.com".to_string(),
                commitment: "confirmed".to_string(),
            },
            risk: RiskConfig {
                max_position_size_sol: 1.0,
                max_daily_loss_sol: 5.0,
                circuit_breaker_threshold: 0.15,
                min_liquidity_usd: 50_000.0,
            },
            ai: AiConfig {
                model: "deepseek-chat".to_string(),
                max_tokens: 1024,
                temperature: 0.1,
                timeout_secs: 30,
            },
            monitoring: MonitoringConfig {
                prometheus_port: 9090,
                alert_webhook_url: None,
            },
        }
    }
}

impl HydraConfig {
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: HydraConfig = toml::from_str(&content)?;
        Ok(config)
    }
}
