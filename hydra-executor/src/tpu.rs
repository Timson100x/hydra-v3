use anyhow::{bail, Result};
use tracing::{info, warn};

use crate::fee::FeeCalculator;
use crate::retry::RetryPolicy;
use hydra_core::constants::MAX_PRIORITY_FEE_MICRO_LAMPORTS;

/// Stub QUIC-based transaction client (replaces yellowstone-jet dependency).
pub struct JetTpuClient {
    endpoint: String,
    fee_calculator: FeeCalculator,
    retry_policy: RetryPolicy,
}

impl JetTpuClient {
    pub fn new(endpoint: String, fee_calculator: FeeCalculator, retry_policy: RetryPolicy) -> Self {
        Self {
            endpoint,
            fee_calculator,
            retry_policy,
        }
    }

    /// Send a transaction with priority fee enforcement.
    /// Returns the transaction signature (stub).
    pub async fn send_transaction(&self, tx_data: &[u8]) -> Result<String> {
        let priority_fee = self.fee_calculator.compute_fee()?;
        if priority_fee > MAX_PRIORITY_FEE_MICRO_LAMPORTS {
            bail!(
                "Priority fee {} exceeds maximum allowed {} micro-lamports",
                priority_fee,
                MAX_PRIORITY_FEE_MICRO_LAMPORTS
            );
        }

        let mut last_err = anyhow::anyhow!("No attempts made");
        for attempt in 0..self.retry_policy.max_attempts {
            match self.try_send(tx_data, priority_fee).await {
                Ok(sig) => {
                    info!(attempt, signature = %sig, "Transaction sent successfully");
                    return Ok(sig);
                }
                Err(e) => {
                    warn!(attempt, error = %e, "Transaction send failed, will retry");
                    last_err = e;
                    if attempt + 1 < self.retry_policy.max_attempts {
                        tokio::time::sleep(self.retry_policy.next_delay(attempt)).await;
                    }
                }
            }
        }
        Err(last_err)
    }

    async fn try_send(&self, _tx_data: &[u8], priority_fee: u64) -> Result<String> {
        // Stub: in production this would send via QUIC to the TPU endpoint
        info!(endpoint = %self.endpoint, priority_fee, "Stub: sending transaction via TPU");
        Ok("stub_signature_placeholder".to_string())
    }
}
