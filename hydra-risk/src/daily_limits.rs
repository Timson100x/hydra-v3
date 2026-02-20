use crate::error::HydraRiskError;
use chrono::{DateTime, Utc};
use tracing::{info, warn};

pub struct DailyLimits {
    max_daily_loss_sol: f64,
    realized_pnl_sol: f64,
    day: DateTime<Utc>,
}

impl DailyLimits {
    pub fn new(max_daily_loss_sol: f64) -> Self {
        Self {
            max_daily_loss_sol,
            realized_pnl_sol: 0.0,
            day: Utc::now(),
        }
    }

    fn maybe_reset(&mut self) {
        let now = Utc::now();
        if now.date_naive() != self.day.date_naive() {
            self.realized_pnl_sol = 0.0;
            self.day = now;
            info!("Daily P&L reset for new day");
        }
    }

    pub fn record_trade_pnl(&mut self, pnl_sol: f64) {
        self.maybe_reset();
        self.realized_pnl_sol += pnl_sol;
        if pnl_sol < 0.0 {
            warn!(
                pnl_sol,
                daily_pnl = self.realized_pnl_sol,
                "Trade loss recorded"
            );
        } else {
            info!(
                pnl_sol,
                daily_pnl = self.realized_pnl_sol,
                "Trade profit recorded"
            );
        }
    }

    pub fn check(&mut self) -> Result<(), HydraRiskError> {
        self.maybe_reset();
        // realized_pnl_sol is negative when we've lost money; negate to get loss magnitude
        let loss = -self.realized_pnl_sol;
        if loss >= self.max_daily_loss_sol {
            return Err(HydraRiskError::DailyLossLimitReached {
                loss_sol: loss,
                max_sol: self.max_daily_loss_sol,
            });
        }
        Ok(())
    }

    pub fn daily_pnl_sol(&mut self) -> f64 {
        self.maybe_reset();
        self.realized_pnl_sol
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daily_limits_within_limit() {
        let mut dl = DailyLimits::new(5.0);
        dl.record_trade_pnl(-2.0);
        assert!(dl.check().is_ok());
        assert!((dl.daily_pnl_sol() - (-2.0)).abs() < f64::EPSILON);
    }

    #[test]
    fn test_daily_limits_exceeded() {
        let mut dl = DailyLimits::new(3.0);
        dl.record_trade_pnl(-4.0);
        assert!(matches!(
            dl.check(),
            Err(HydraRiskError::DailyLossLimitReached { .. })
        ));
    }

    #[test]
    fn test_daily_limits_profit_does_not_trip() {
        let mut dl = DailyLimits::new(1.0);
        dl.record_trade_pnl(10.0);
        assert!(dl.check().is_ok());
    }
}
