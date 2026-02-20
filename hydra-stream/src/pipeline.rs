use std::sync::Arc;
use tracing::{info, warn};
use hydra_core::{Signal, SignalType, TokenInfo};
use crate::source::TokenSource;

pub struct SignalPipeline {
    source: Arc<dyn TokenSource>,
    min_liquidity_usd: f64,
}

impl SignalPipeline {
    pub fn new(source: Arc<dyn TokenSource>, min_liquidity_usd: f64) -> Self {
        Self {
            source,
            min_liquidity_usd,
        }
    }

    pub async fn process(&self) -> anyhow::Result<Vec<Signal>> {
        let tokens = self.source.fetch_new_tokens().await?;
        info!("Processing {} tokens from source", tokens.len());

        let signals: Vec<Signal> = tokens
            .into_iter()
            .filter_map(|token| self.evaluate_token(token))
            .collect();

        info!("Generated {} signals", signals.len());
        Ok(signals)
    }

    fn evaluate_token(&self, token: TokenInfo) -> Option<Signal> {
        if token.liquidity_usd < self.min_liquidity_usd {
            warn!(
                "Token {} filtered: liquidity ${:.0} below threshold ${:.0}",
                token.address, token.liquidity_usd, self.min_liquidity_usd
            );
            return None;
        }

        let confidence = self.calculate_confidence(&token);
        let signal_type = if confidence > 0.7 {
            SignalType::Buy
        } else if confidence < 0.3 {
            SignalType::Sell
        } else {
            SignalType::Hold
        };

        Some(Signal::new(token.address, signal_type, confidence))
    }

    fn calculate_confidence(&self, token: &TokenInfo) -> f64 {
        let liquidity_score = (token.liquidity_usd / 1_000_000.0).min(1.0);
        let volume_score = if token.liquidity_usd > 0.0 {
            (token.volume_24h_usd / token.liquidity_usd).min(1.0)
        } else {
            0.0
        };
        (liquidity_score * 0.5 + volume_score * 0.5).clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hydra_core::TokenAddress;
    use crate::source::MockTokenSource;

    fn make_token(liquidity: f64, volume: f64) -> TokenInfo {
        TokenInfo {
            address: TokenAddress::new("TestToken11111111111111111111111111111111111"),
            symbol: "TEST".to_string(),
            name: "Test Token".to_string(),
            decimals: 9,
            liquidity_usd: liquidity,
            price_usd: 0.001,
            volume_24h_usd: volume,
            market_cap_usd: liquidity * 2.0,
        }
    }

    #[tokio::test]
    async fn test_pipeline_filters_low_liquidity() {
        let token = make_token(10_000.0, 5_000.0);
        let source = Arc::new(MockTokenSource::new(vec![token]));
        let pipeline = SignalPipeline::new(source, 50_000.0);
        let signals = pipeline.process().await.unwrap();
        assert!(signals.is_empty());
    }

    #[tokio::test]
    async fn test_pipeline_generates_signal() {
        let token = make_token(1_000_000.0, 900_000.0);
        let source = Arc::new(MockTokenSource::new(vec![token]));
        let pipeline = SignalPipeline::new(source, 50_000.0);
        let signals = pipeline.process().await.unwrap();
        assert_eq!(signals.len(), 1);
        assert_eq!(signals[0].signal_type, SignalType::Buy);
    }
}
