use anyhow::{bail, Result};
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

    pub fn record_trade_pnl(&mut self, pnl_sol: f64) -> Result<()> {
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
        Ok(())
    }

    pub fn check(&mut self) -> Result<()> {
        self.maybe_reset();
        // realized_pnl_sol is negative when we've lost money; negate to get loss magnitude
        let loss = -self.realized_pnl_sol;
        if loss >= self.max_daily_loss_sol {
            bail!(
                "Daily loss limit reached: {:.4} SOL (max {:.4} SOL)",
                loss,
                self.max_daily_loss_sol
            );
        }
        Ok(())
    }

    pub fn daily_pnl_sol(&mut self) -> f64 {
        self.maybe_reset();
        self.realized_pnl_sol
    }
}
