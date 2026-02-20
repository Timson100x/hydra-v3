use anyhow::{bail, Result};
use tracing::info;

/// Stub signature verification shield.
pub struct Shield;

impl Shield {
    pub fn new() -> Self {
        Self
    }

    /// Verify a transaction signature (stub implementation).
    pub fn verify_signature(&self, signature: &str) -> Result<()> {
        if signature.is_empty() {
            bail!("Empty signature is invalid");
        }
        info!(signature, "Stub: signature verification passed");
        Ok(())
    }
}

impl Default for Shield {
    fn default() -> Self {
        Self::new()
    }
}
