use anyhow::Result;
use axum::{routing::get, Router};
use prometheus_client::{
    encoding::text::encode,
    metrics::{counter::Counter, gauge::Gauge},
    registry::Registry,
};
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicU64;
use tracing::info;

#[derive(Clone)]
pub struct HydraMetrics {
    pub trades_total: Counter,
    pub trades_won: Counter,
    pub trades_lost: Counter,
    pub open_positions: Gauge,
    pub daily_pnl_sol: Gauge<f64, AtomicU64>,
    pub ai_score_requests: Counter,
    pub circuit_breaker_trips: Counter,
    pub signals_received: Counter,
}

impl HydraMetrics {
    pub fn new(registry: &mut Registry) -> Self {
        let trades_total: Counter = Counter::default();
        let trades_won: Counter = Counter::default();
        let trades_lost: Counter = Counter::default();
        let open_positions: Gauge = Gauge::default();
        let daily_pnl_sol: Gauge<f64, AtomicU64> = Gauge::default();
        let ai_score_requests: Counter = Counter::default();
        let circuit_breaker_trips: Counter = Counter::default();
        let signals_received: Counter = Counter::default();

        registry.register("hydra_trades_total", "Total number of trades", trades_total.clone());
        registry.register("hydra_trades_won", "Number of winning trades", trades_won.clone());
        registry.register("hydra_trades_lost", "Number of losing trades", trades_lost.clone());
        registry.register("hydra_open_positions", "Current open positions", open_positions.clone());
        registry.register("hydra_daily_pnl_sol", "Daily P&L in SOL", daily_pnl_sol.clone());
        registry.register("hydra_ai_score_requests", "Total AI score requests", ai_score_requests.clone());
        registry.register("hydra_circuit_breaker_trips", "Circuit breaker trip count", circuit_breaker_trips.clone());
        registry.register("hydra_signals_received", "Total signals received", signals_received.clone());

        Self {
            trades_total,
            trades_won,
            trades_lost,
            open_positions,
            daily_pnl_sol,
            ai_score_requests,
            circuit_breaker_trips,
            signals_received,
        }
    }
}

pub struct MetricsServer {
    port: u16,
    registry: Arc<Mutex<Registry>>,
}

impl MetricsServer {
    pub fn new(port: u16) -> (Self, HydraMetrics) {
        let mut registry = Registry::default();
        let metrics = HydraMetrics::new(&mut registry);
        (
            Self {
                port,
                registry: Arc::new(Mutex::new(registry)),
            },
            metrics,
        )
    }

    pub async fn run(self) -> Result<()> {
        let registry = self.registry.clone();
        let app = Router::new().route(
            "/metrics",
            get(move || {
                let registry = registry.clone();
                async move {
                    let mut buf = String::new();
                    let reg = registry.lock().map_err(|e| {
                        (
                            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Lock error: {e}"),
                        )
                    })?;
                    encode(&mut buf, &reg).map_err(|e| {
                        (
                            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Encode error: {e}"),
                        )
                    })?;
                    Ok::<_, (axum::http::StatusCode, String)>(buf)
                }
            }),
        );

        let addr = format!("0.0.0.0:{}", self.port);
        info!("Metrics server listening on {}", addr);
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}
