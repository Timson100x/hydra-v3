use std::sync::Arc;
use async_trait::async_trait;
use tracing::{info, warn};

use hydra_core::{HydraError, OrderType, Signal, SignalType, TradeOrder};
use crate::transaction::TransactionManager;

#[async_trait]
pub trait ExecutionBackend: Send + Sync {
    async fn execute_order(&self, order: &TradeOrder) -> Result<String, HydraError>;
}

pub struct MockBackend;

#[async_trait]
impl ExecutionBackend for MockBackend {
    async fn execute_order(&self, order: &TradeOrder) -> Result<String, HydraError> {
        info!("Mock executing order {} for {}", order.id, order.token);
        Ok(format!("mock_tx_{}", order.id))
    }
}

pub struct TradeExecutor {
    manager: Arc<TransactionManager>,
    backend: Arc<dyn ExecutionBackend>,
    position_size_sol: f64,
}

impl TradeExecutor {
    pub fn new(
        manager: Arc<TransactionManager>,
        backend: Arc<dyn ExecutionBackend>,
        position_size_sol: f64,
    ) -> Self {
        Self {
            manager,
            backend,
            position_size_sol,
        }
    }

    pub async fn execute_signal(&self, signal: &Signal) -> Result<(), HydraError> {
        let order_type = match signal.signal_type {
            SignalType::Buy => OrderType::Buy,
            SignalType::Sell => OrderType::Sell,
            SignalType::Hold => {
                info!("Hold signal for {} - skipping", signal.token);
                return Ok(());
            }
        };

        let order = TradeOrder::new(signal, order_type, self.position_size_sol, 100);
        let id = self.manager.submit(order.clone()).await;

        match self.backend.execute_order(&order).await {
            Ok(sig) => {
                self.manager.complete(id, Some(sig), None).await;
                Ok(())
            }
            Err(e) => {
                warn!("Execution failed for order {}: {}", id, e);
                self.manager.complete(id, None, Some(e.to_string())).await;
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hydra_core::TokenAddress;

    #[tokio::test]
    async fn test_execute_buy_signal() {
        let manager = Arc::new(TransactionManager::new());
        let backend = Arc::new(MockBackend);
        let executor = TradeExecutor::new(manager.clone(), backend, 0.5);

        let token = TokenAddress::new("TestToken11111111111111111111111111111111111");
        let signal = Signal::new(token, SignalType::Buy, 0.9);

        executor.execute_signal(&signal).await.unwrap();
        let results = manager.completed_results().await;
        assert_eq!(results.len(), 1);
    }

    #[tokio::test]
    async fn test_skip_hold_signal() {
        let manager = Arc::new(TransactionManager::new());
        let backend = Arc::new(MockBackend);
        let executor = TradeExecutor::new(manager.clone(), backend, 0.5);

        let token = TokenAddress::new("TestToken11111111111111111111111111111111111");
        let signal = Signal::new(token, SignalType::Hold, 0.5);

        executor.execute_signal(&signal).await.unwrap();
        assert_eq!(manager.pending_count().await, 0);
        assert!(manager.completed_results().await.is_empty());
    }
}
