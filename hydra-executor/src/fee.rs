use anyhow::Result;
use hydra_core::constants::MAX_PRIORITY_FEE_MICRO_LAMPORTS;
use tracing::info;

pub struct FeeCalculator {
    base_fee: u64,
    p75_multiplier: f64,
}

impl FeeCalculator {
    pub fn new(base_fee: u64, p75_multiplier: f64) -> Self {
        Self {
            base_fee,
            p75_multiplier,
        }
    }

    /// Compute the priority fee capped at MAX_PRIORITY_FEE_MICRO_LAMPORTS.
    pub fn compute_fee(&self) -> Result<u64> {
        let fee = (self.base_fee as f64 * self.p75_multiplier) as u64;
        let capped = fee.min(MAX_PRIORITY_FEE_MICRO_LAMPORTS);
        info!(
            computed_fee = fee,
            capped_fee = capped,
            "Priority fee computed"
        );
        Ok(capped)
    }
}

impl Default for FeeCalculator {
    fn default() -> Self {
        Self::new(1_000, 1.75)
    }
}
