use thiserror::Error;

#[derive(Debug, Error)]
pub enum HydraRiskError {
    #[error("Circuit breaker tripped after {consecutive_losses} consecutive losses")]
    CircuitBreakerTripped { consecutive_losses: u32 },

    #[error("Daily loss limit reached: {loss_sol:.4} SOL (max {max_sol:.4} SOL)")]
    DailyLossLimitReached { loss_sol: f64, max_sol: f64 },

    #[error("Cannot open position: max open positions ({max}) reached")]
    MaxPositionsReached { max: usize },

    #[error("Position not found: {id}")]
    PositionNotFound { id: String },
}
