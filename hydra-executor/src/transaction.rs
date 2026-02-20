use std::collections::HashMap;
use tokio::sync::RwLock;
use hydra_core::{TradeOrder, TradeResult, TradeStatus};
use uuid::Uuid;
use chrono::Utc;
use tracing::{info, warn};

pub struct TransactionManager {
    pending: RwLock<HashMap<Uuid, TradeOrder>>,
    completed: RwLock<Vec<TradeResult>>,
}

impl TransactionManager {
    pub fn new() -> Self {
        Self {
            pending: RwLock::new(HashMap::new()),
            completed: RwLock::new(Vec::new()),
        }
    }

    pub async fn submit(&self, order: TradeOrder) -> Uuid {
        let id = order.id;
        info!("Submitting order {} for token {}", id, order.token);
        self.pending.write().await.insert(id, order);
        id
    }

    pub async fn complete(&self, order_id: Uuid, tx_signature: Option<String>, error: Option<String>) {
        let mut pending = self.pending.write().await;
        if let Some(order) = pending.remove(&order_id) {
            let status = if error.is_some() {
                warn!("Order {} failed: {:?}", order_id, error);
                TradeStatus::Failed
            } else {
                info!("Order {} executed: {:?}", order_id, tx_signature);
                TradeStatus::Executed
            };
            let result = TradeResult {
                order_id,
                status,
                tx_signature,
                actual_amount_sol: Some(order.amount_sol),
                executed_at: Utc::now(),
                error,
            };
            self.completed.write().await.push(result);
        }
    }

    pub async fn pending_count(&self) -> usize {
        self.pending.read().await.len()
    }

    pub async fn completed_results(&self) -> Vec<TradeResult> {
        self.completed.read().await.clone()
    }
}

impl Default for TransactionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hydra_core::{OrderType, Signal, SignalType, TokenAddress};

    #[tokio::test]
    async fn test_transaction_lifecycle() {
        let manager = TransactionManager::new();
        let token = TokenAddress::new("TestToken11111111111111111111111111111111111");
        let signal = Signal::new(token, SignalType::Buy, 0.9);
        let order = TradeOrder::new(&signal, OrderType::Buy, 0.5, 100);
        let id = order.id;

        manager.submit(order).await;
        assert_eq!(manager.pending_count().await, 1);

        manager.complete(id, Some("sig123".to_string()), None).await;
        assert_eq!(manager.pending_count().await, 0);
        let results = manager.completed_results().await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].status, TradeStatus::Executed);
    }
}
