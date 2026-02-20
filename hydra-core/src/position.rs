use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub id: String,
    pub mint_address: String,
    pub entry_price_usd: f64,
    pub size_sol: f64,
    pub take_profit_pct: f64,
    pub stop_loss_pct: f64,
    pub opened_at: DateTime<Utc>,
}

impl Position {
    pub fn new(
        id: String,
        mint_address: String,
        entry_price_usd: f64,
        size_sol: f64,
        take_profit_pct: f64,
        stop_loss_pct: f64,
    ) -> Self {
        Self {
            id,
            mint_address,
            entry_price_usd,
            size_sol,
            take_profit_pct,
            stop_loss_pct,
            opened_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedTrade {
    pub position_id: String,
    pub mint_address: String,
    pub entry_price_usd: f64,
    pub exit_price_usd: f64,
    pub size_sol: f64,
    pub pnl_sol: f64,
    pub opened_at: DateTime<Utc>,
    pub closed_at: DateTime<Utc>,
    pub exit_reason: String,
}
