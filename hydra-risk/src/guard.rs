use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tracing::{info, warn};
use hydra_core::{HydraError, RiskConfig, Signal};

pub struct RiskGuard {
    config: RiskConfig,
    daily_loss_lamports: Arc<AtomicU64>,
}

impl RiskGuard {
    pub fn new(config: RiskConfig) -> Self {
        Self {
            config,
            daily_loss_lamports: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn check_signal(&self, signal: &Signal) -> Result<(), HydraError> {
        let daily_loss_sol = self.daily_loss_lamports.load(Ordering::Relaxed) as f64 / 1e9;
        if daily_loss_sol >= self.config.max_daily_loss_sol {
            warn!("Daily loss limit reached: {:.4} SOL", daily_loss_sol);
            return Err(HydraError::RiskThreshold(format!(
                "Daily loss {:.4} SOL exceeds limit {:.4} SOL",
                daily_loss_sol, self.config.max_daily_loss_sol
            )));
        }

        if signal.confidence < self.config.circuit_breaker_threshold {
            warn!(
                "Signal confidence {:.2} below risk threshold {:.2}",
                signal.confidence, self.config.circuit_breaker_threshold
            );
            return Err(HydraError::RiskThreshold(format!(
                "Signal confidence {:.2} below threshold {:.2}",
                signal.confidence, self.config.circuit_breaker_threshold
            )));
        }

        info!("Risk check passed for signal {}", signal.id);
        Ok(())
    }

    pub fn record_loss(&self, loss_sol: f64) {
        let lamports = (loss_sol * 1e9) as u64;
        let total = self.daily_loss_lamports.fetch_add(lamports, Ordering::Relaxed) + lamports;
        warn!("Loss recorded: {:.4} SOL. Daily total: {:.4} SOL", loss_sol, total as f64 / 1e9);
    }

    pub fn daily_loss_sol(&self) -> f64 {
        self.daily_loss_lamports.load(Ordering::Relaxed) as f64 / 1e9
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hydra_core::{SignalType, TokenAddress};

    fn default_config() -> RiskConfig {
        RiskConfig {
            max_position_size_sol: 1.0,
            max_daily_loss_sol: 5.0,
            circuit_breaker_threshold: 0.15,
            min_liquidity_usd: 50_000.0,
        }
    }

    #[test]
    fn test_risk_guard_passes_good_signal() {
        let guard = RiskGuard::new(default_config());
        let token = TokenAddress::new("TestToken11111111111111111111111111111111111");
        let signal = Signal::new(token, SignalType::Buy, 0.9);
        assert!(guard.check_signal(&signal).is_ok());
    }

    #[test]
    fn test_risk_guard_blocks_low_confidence() {
        let guard = RiskGuard::new(default_config());
        let token = TokenAddress::new("TestToken11111111111111111111111111111111111");
        let signal = Signal::new(token, SignalType::Buy, 0.10);
        assert!(guard.check_signal(&signal).is_err());
    }

    #[test]
    fn test_risk_guard_blocks_after_daily_loss() {
        let guard = RiskGuard::new(default_config());
        guard.record_loss(5.0);
        let token = TokenAddress::new("TestToken11111111111111111111111111111111111");
        let signal = Signal::new(token, SignalType::Buy, 0.9);
        assert!(guard.check_signal(&signal).is_err());
    }
}
