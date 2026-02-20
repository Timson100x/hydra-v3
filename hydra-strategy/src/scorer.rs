use hydra_core::{Signal, TokenInfo};
use tracing::debug;

pub struct RiskScorer {
    max_market_cap_usd: f64,
}

impl RiskScorer {
    pub fn new(max_market_cap_usd: f64) -> Self {
        Self { max_market_cap_usd }
    }

    pub fn score(&self, signal: &Signal, token_info: &TokenInfo) -> f64 {
        let confidence_score = signal.confidence;
        let liquidity_score = (token_info.liquidity_usd / 1_000_000.0).min(1.0);
        let market_cap_score = if token_info.market_cap_usd > 0.0 {
            1.0 - (token_info.market_cap_usd / self.max_market_cap_usd).min(1.0)
        } else {
            0.5
        };

        let composite = confidence_score * 0.5 + liquidity_score * 0.3 + market_cap_score * 0.2;
        debug!(
            "Risk score for {}: {:.3} (confidence={:.2}, liquidity={:.2}, mcap={:.2})",
            token_info.address, composite, confidence_score, liquidity_score, market_cap_score
        );
        composite.clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hydra_core::{SignalType, TokenAddress};

    #[test]
    fn test_risk_scorer() {
        let token_address = TokenAddress::new("TestToken11111111111111111111111111111111111");
        let signal = Signal::new(token_address.clone(), SignalType::Buy, 0.8);
        let token_info = TokenInfo {
            address: token_address,
            symbol: "TEST".to_string(),
            name: "Test Token".to_string(),
            decimals: 9,
            liquidity_usd: 500_000.0,
            price_usd: 0.001,
            volume_24h_usd: 100_000.0,
            market_cap_usd: 1_000_000.0,
        };
        let scorer = RiskScorer::new(10_000_000.0);
        let score = scorer.score(&signal, &token_info);
        assert!(score > 0.0 && score <= 1.0);
    }
}
