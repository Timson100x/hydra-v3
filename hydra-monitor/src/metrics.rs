use prometheus::{
    Gauge, Histogram, HistogramOpts, IntCounter, Opts, Registry,
};
use anyhow::Result;

pub struct HydraMetrics {
    pub registry: Registry,
    pub signals_processed: IntCounter,
    pub trades_executed: IntCounter,
    pub trades_failed: IntCounter,
    pub daily_pnl_sol: Gauge,
    pub circuit_breaker_trips: IntCounter,
    pub ai_request_duration: Histogram,
    pub active_positions: Gauge,
}

impl HydraMetrics {
    pub fn new() -> Result<Self> {
        let registry = Registry::new();

        let signals_processed = IntCounter::with_opts(Opts::new(
            "hydra_signals_processed_total",
            "Total number of signals processed",
        ))?;

        let trades_executed = IntCounter::with_opts(Opts::new(
            "hydra_trades_executed_total",
            "Total number of trades executed",
        ))?;

        let trades_failed = IntCounter::with_opts(Opts::new(
            "hydra_trades_failed_total",
            "Total number of trades failed",
        ))?;

        let daily_pnl_sol = Gauge::with_opts(Opts::new(
            "hydra_daily_pnl_sol",
            "Daily profit and loss in SOL",
        ))?;

        let circuit_breaker_trips = IntCounter::with_opts(Opts::new(
            "hydra_circuit_breaker_trips_total",
            "Total number of circuit breaker activations",
        ))?;

        let ai_request_duration = Histogram::with_opts(HistogramOpts::new(
            "hydra_ai_request_duration_seconds",
            "Duration of AI API requests",
        ))?;

        let active_positions = Gauge::with_opts(Opts::new(
            "hydra_active_positions",
            "Number of currently active positions",
        ))?;

        registry.register(Box::new(signals_processed.clone()))?;
        registry.register(Box::new(trades_executed.clone()))?;
        registry.register(Box::new(trades_failed.clone()))?;
        registry.register(Box::new(daily_pnl_sol.clone()))?;
        registry.register(Box::new(circuit_breaker_trips.clone()))?;
        registry.register(Box::new(ai_request_duration.clone()))?;
        registry.register(Box::new(active_positions.clone()))?;

        Ok(Self {
            registry,
            signals_processed,
            trades_executed,
            trades_failed,
            daily_pnl_sol,
            circuit_breaker_trips,
            ai_request_duration,
            active_positions,
        })
    }

    pub fn gather_text(&self) -> String {
        use prometheus::Encoder;
        let encoder = prometheus::TextEncoder::new();
        let mut buffer = Vec::new();
        encoder
            .encode(&self.registry.gather(), &mut buffer)
            .unwrap_or_default();
        String::from_utf8(buffer).unwrap_or_default()
    }
}

impl Default for HydraMetrics {
    fn default() -> Self {
        Self::new().expect("Failed to create metrics")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_creation() {
        let metrics = HydraMetrics::new().unwrap();
        metrics.signals_processed.inc();
        metrics.trades_executed.inc();
        assert_eq!(metrics.signals_processed.get(), 1);
        assert_eq!(metrics.trades_executed.get(), 1);
    }

    #[test]
    fn test_metrics_gather() {
        let metrics = HydraMetrics::new().unwrap();
        metrics.signals_processed.inc_by(5);
        let text = metrics.gather_text();
        assert!(text.contains("hydra_signals_processed_total"));
    }
}
