use anyhow::{bail, Result};
use tracing::{info, warn};

pub struct CircuitBreaker {
    max_consecutive_losses: u32,
    consecutive_losses: u32,
    tripped: bool,
}

impl CircuitBreaker {
    pub fn new(max_consecutive_losses: u32) -> Self {
        Self {
            max_consecutive_losses,
            consecutive_losses: 0,
            tripped: false,
        }
    }

    pub fn is_tripped(&self) -> bool {
        self.tripped
    }

    pub fn record_loss(&mut self) -> Result<()> {
        self.consecutive_losses += 1;
        warn!(consecutive_losses = self.consecutive_losses, "Loss recorded");
        if self.consecutive_losses >= self.max_consecutive_losses {
            self.tripped = true;
            warn!("Circuit breaker tripped after {} consecutive losses", self.consecutive_losses);
        }
        Ok(())
    }

    pub fn record_win(&mut self) {
        self.consecutive_losses = 0;
        info!("Win recorded, consecutive losses reset");
    }

    pub fn check(&self) -> Result<()> {
        if self.tripped {
            bail!("Circuit breaker is tripped, trading halted");
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        self.tripped = false;
        self.consecutive_losses = 0;
        info!("Circuit breaker reset");
    }
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self::new(5)
    }
}
