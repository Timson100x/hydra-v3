use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintSignal {
    pub mint_address: String,
    pub market_cap_usd: f64,
    pub volume_24h_usd: f64,
    pub price_usd: f64,
    pub holder_count: u64,
    pub liquidity_usd: f64,
    pub top_holder_pct: f64,
    pub timestamp: DateTime<Utc>,
}

impl MintSignal {
    pub fn new(
        mint_address: String,
        market_cap_usd: f64,
        volume_24h_usd: f64,
        price_usd: f64,
        holder_count: u64,
        liquidity_usd: f64,
        top_holder_pct: f64,
    ) -> Self {
        Self {
            mint_address,
            market_cap_usd,
            volume_24h_usd,
            price_usd,
            holder_count,
            liquidity_usd,
            top_holder_pct,
            timestamp: Utc::now(),
        }
    }
}
