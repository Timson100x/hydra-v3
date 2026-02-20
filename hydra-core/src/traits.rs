// Central interfaces for all crates.
// Enables mocking in tests!

use crate::signal::MintSignal;
use std::future::Future;
use thiserror::Error;

/// Signal scored by the AI analyzer
#[derive(Debug, Clone)]
pub struct ScoredSignal {
    pub signal: MintSignal,
    pub score: f64,
    pub should_buy: bool,
}

/// Error returned when a risk check is denied
#[derive(Debug, Clone, Error)]
#[error("Risk check denied: {reason}")]
pub struct RiskDenied {
    pub reason: String,
}

/// Yields new token-launch signals from the stream
pub trait MarketDataStream: Send + Sync {
    fn next_signal(&mut self) -> impl Future<Output = Option<MintSignal>> + Send;
}

/// Scores a signal using AI analysis
pub trait AiAnalyzer: Send + Sync {
    fn analyze(&self, signal: &MintSignal) -> impl Future<Output = Option<ScoredSignal>> + Send;
}

/// Executes a trade for a scored signal
pub trait TradeExecutor: Send + Sync {
    fn execute(&self, signal: &ScoredSignal) -> impl Future<Output = anyhow::Result<()>> + Send;
}

/// Checks whether a trade is permitted
pub trait RiskEngine: Send + Sync {
    fn approve(&self, signal: &MintSignal) -> Result<(), RiskDenied>;
    fn record_loss(&self);
    fn record_win(&self);
    fn is_halted(&self) -> bool;
}

// ── Mocks für Tests ───────────────────────────────────────────────────────────
#[cfg(test)]
pub mod mocks {
    use super::*;

    pub struct AlwaysBuyAnalyzer;
    impl AiAnalyzer for AlwaysBuyAnalyzer {
        async fn analyze(&self, signal: &MintSignal) -> Option<ScoredSignal> {
            Some(ScoredSignal {
                signal: signal.clone(),
                score: 0.9,
                should_buy: true,
            })
        }
    }

    pub struct AlwaysApproveRisk;
    impl RiskEngine for AlwaysApproveRisk {
        fn approve(&self, _: &MintSignal) -> Result<(), RiskDenied> {
            Ok(())
        }
        fn record_loss(&self) {}
        fn record_win(&self) {}
        fn is_halted(&self) -> bool {
            false
        }
    }

    #[tokio::test]
    async fn test_always_buy_analyzer() {
        use crate::signal::MintSignal;
        let analyzer = AlwaysBuyAnalyzer;
        let signal = MintSignal::new(
            "test_mint".to_string(),
            1000.0,
            500.0,
            0.001,
            100,
            800.0,
            10.0,
        );
        let result = analyzer.analyze(&signal).await;
        assert!(result.is_some());
        let scored = result.unwrap();
        assert!(scored.should_buy);
        assert_eq!(scored.score, 0.9);
    }

    #[test]
    fn test_always_approve_risk() {
        use crate::signal::MintSignal;
        let engine = AlwaysApproveRisk;
        let signal = MintSignal::new(
            "test_mint".to_string(),
            1000.0,
            500.0,
            0.001,
            100,
            800.0,
            10.0,
        );
        assert!(engine.approve(&signal).is_ok());
        assert!(!engine.is_halted());
    }
}
