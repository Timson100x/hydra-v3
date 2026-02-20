use anyhow::Result;
use csv::WriterBuilder;
use hydra_core::position::CompletedTrade;
use std::path::Path;
use tracing::info;

pub struct TradeJournal {
    path: String,
}

impl TradeJournal {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    /// Append a completed trade to the CSV journal.
    pub fn record(&self, trade: &CompletedTrade) -> Result<()> {
        let path = Path::new(&self.path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file_exists = path.exists();
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;

        let mut writer = WriterBuilder::new()
            .has_headers(!file_exists)
            .from_writer(file);

        writer.serialize(trade)?;
        writer.flush()?;

        info!(
            position_id = %trade.position_id,
            pnl_sol = trade.pnl_sol,
            "Trade recorded to journal"
        );
        Ok(())
    }
}
