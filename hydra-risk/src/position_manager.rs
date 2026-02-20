use anyhow::{bail, Result};
use dashmap::DashMap;
use hydra_core::position::Position;
use tracing::info;

pub struct PositionManager {
    positions: DashMap<String, Position>,
    max_open: usize,
}

impl PositionManager {
    pub fn new(max_open: usize) -> Self {
        Self {
            positions: DashMap::new(),
            max_open,
        }
    }

    pub fn open(&self, position: Position) -> Result<()> {
        if self.positions.len() >= self.max_open {
            bail!(
                "Cannot open position: max open positions ({}) reached",
                self.max_open
            );
        }
        info!(position_id = %position.id, mint = %position.mint_address, "Opening position");
        self.positions.insert(position.id.clone(), position);
        Ok(())
    }

    pub fn close(&self, position_id: &str) -> Result<Position> {
        self.positions
            .remove(position_id)
            .map(|(_, p)| p)
            .ok_or_else(|| anyhow::anyhow!("Position {} not found", position_id))
    }

    pub fn get(&self, position_id: &str) -> Option<Position> {
        self.positions.get(position_id).map(|p| p.clone())
    }

    pub fn open_count(&self) -> usize {
        self.positions.len()
    }

    pub fn all(&self) -> Vec<Position> {
        self.positions.iter().map(|e| e.value().clone()).collect()
    }
}

impl Default for PositionManager {
    fn default() -> Self {
        Self::new(10)
    }
}
