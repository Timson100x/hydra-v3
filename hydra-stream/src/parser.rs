use anyhow::Result;
use hydra_core::signal::MintSignal;
use tracing::warn;

pub struct PumpfunParser;

impl PumpfunParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse raw pump.fun event data into a MintSignal.
    /// This is a stub implementation that deserializes expected JSON fields.
    pub fn parse(&self, raw: &[u8]) -> Result<MintSignal> {
        let value: serde_json::Value = serde_json::from_slice(raw)?;

        let mint_address = value["mint"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("missing mint field"))?
            .to_string();

        let market_cap_usd = value["marketCapUsd"].as_f64().unwrap_or(0.0);
        let volume_24h_usd = value["volume24hUsd"].as_f64().unwrap_or(0.0);
        let price_usd = value["priceUsd"].as_f64().unwrap_or(0.0);
        let holder_count = value["holderCount"].as_u64().unwrap_or(0);
        let liquidity_usd = value["liquidityUsd"].as_f64().unwrap_or(0.0);
        let top_holder_pct = value["topHolderPct"].as_f64().unwrap_or(0.0);

        if mint_address.is_empty() {
            warn!("Parsed empty mint address from pump.fun event");
        }

        Ok(MintSignal::new(
            mint_address,
            market_cap_usd,
            volume_24h_usd,
            price_usd,
            holder_count,
            liquidity_usd,
            top_holder_pct,
        ))
    }
}

impl Default for PumpfunParser {
    fn default() -> Self {
        Self::new()
    }
}
