use crate::error::HydraRiskError;
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

    pub fn record_loss(&mut self) {
        self.consecutive_losses += 1;
        warn!(
            consecutive_losses = self.consecutive_losses,
            "Loss recorded"
        );
        if self.consecutive_losses >= self.max_consecutive_losses {
            self.tripped = true;
            warn!(
                "Circuit breaker tripped after {} consecutive losses",
                self.consecutive_losses
            );
        }
    }

    pub fn record_win(&mut self) {
        self.consecutive_losses = 0;
        info!("Win recorded, consecutive losses reset");
    }

    pub fn check(&self) -> Result<(), HydraRiskError> {
        if self.tripped {
            return Err(HydraRiskError::CircuitBreakerTripped {
                consecutive_losses: self.consecutive_losses,
            });
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_breaker_not_tripped_initially() {
        let cb = CircuitBreaker::new(4);
        assert!(!cb.is_tripped());
        assert!(cb.check().is_ok());
    }

    #[test]
    fn test_circuit_breaker_trips_after_max_losses() {
        let mut cb = CircuitBreaker::new(4);
        for _ in 0..4 {
            cb.record_loss();
        }
        assert!(cb.is_tripped());
        assert!(matches!(
            cb.check(),
            Err(HydraRiskError::CircuitBreakerTripped { .. })
        ));
    }

    #[test]
    fn test_circuit_breaker_win_resets_losses() {
        let mut cb = CircuitBreaker::new(4);
        cb.record_loss();
        cb.record_loss();
        cb.record_win();
        assert!(!cb.is_tripped());
        assert!(cb.check().is_ok());
    }

    #[test]
    fn test_circuit_breaker_reset() {
        let mut cb = CircuitBreaker::new(2);
        cb.record_loss();
        cb.record_loss();
        assert!(cb.is_tripped());
        cb.reset();
        assert!(!cb.is_tripped());
        assert!(cb.check().is_ok());
    }
}
