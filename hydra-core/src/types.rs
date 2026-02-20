use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TokenAddress(pub String);

impl TokenAddress {
    pub fn new(addr: impl Into<String>) -> Self {
        Self(addr.into())
    }
}

impl std::fmt::Display for TokenAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    pub id: Uuid,
    pub token: TokenAddress,
    pub signal_type: SignalType,
    pub confidence: f64,
    pub timestamp: DateTime<Utc>,
    pub metadata: serde_json::Value,
}

impl Signal {
    pub fn new(token: TokenAddress, signal_type: SignalType, confidence: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            token,
            signal_type,
            confidence,
            timestamp: Utc::now(),
            metadata: serde_json::Value::Null,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SignalType {
    Buy,
    Sell,
    Hold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeOrder {
    pub id: Uuid,
    pub signal_id: Uuid,
    pub token: TokenAddress,
    pub order_type: OrderType,
    pub amount_sol: f64,
    pub slippage_bps: u32,
    pub created_at: DateTime<Utc>,
}

impl TradeOrder {
    pub fn new(
        signal: &Signal,
        order_type: OrderType,
        amount_sol: f64,
        slippage_bps: u32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            signal_id: signal.id,
            token: signal.token.clone(),
            order_type,
            amount_sol,
            slippage_bps,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeResult {
    pub order_id: Uuid,
    pub status: TradeStatus,
    pub tx_signature: Option<String>,
    pub actual_amount_sol: Option<f64>,
    pub executed_at: DateTime<Utc>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TradeStatus {
    Pending,
    Executed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub address: TokenAddress,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    pub liquidity_usd: f64,
    pub price_usd: f64,
    pub volume_24h_usd: f64,
    pub market_cap_usd: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_creation() {
        let token = TokenAddress::new("So11111111111111111111111111111111111111112");
        let signal = Signal::new(token.clone(), SignalType::Buy, 0.85);
        assert_eq!(signal.token, token);
        assert_eq!(signal.signal_type, SignalType::Buy);
        assert!((signal.confidence - 0.85).abs() < f64::EPSILON);
    }

    #[test]
    fn test_token_address_display() {
        let addr = "So11111111111111111111111111111111111111112";
        let token = TokenAddress::new(addr);
        assert_eq!(token.to_string(), addr);
    }
}
