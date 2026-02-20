use crate::error::HydraRiskError;
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

    pub fn open(&self, position: Position) -> Result<(), HydraRiskError> {
        if self.positions.len() >= self.max_open {
            return Err(HydraRiskError::MaxPositionsReached { max: self.max_open });
        }
        info!(position_id = %position.id, mint = %position.mint_address, "Opening position");
        self.positions.insert(position.id.clone(), position);
        Ok(())
    }

    pub fn close(&self, position_id: &str) -> Result<Position, HydraRiskError> {
        self.positions
            .remove(position_id)
            .map(|(_, p)| p)
            .ok_or_else(|| HydraRiskError::PositionNotFound {
                id: position_id.to_string(),
            })
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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_position(id: &str, mint: &str) -> Position {
        Position::new(id.to_string(), mint.to_string(), 0.001, 1.0, 0.30, 0.20)
    }

    #[test]
    fn test_open_and_close_position() {
        let pm = PositionManager::new(5);
        let pos = make_position("pos1", "mint_abc");
        assert!(pm.open(pos).is_ok());
        assert_eq!(pm.open_count(), 1);
        let closed = pm.close("pos1");
        assert!(closed.is_ok());
        assert_eq!(pm.open_count(), 0);
    }

    #[test]
    fn test_max_positions_reached() {
        let pm = PositionManager::new(2);
        pm.open(make_position("p1", "m1")).unwrap();
        pm.open(make_position("p2", "m2")).unwrap();
        let result = pm.open(make_position("p3", "m3"));
        assert!(matches!(
            result,
            Err(HydraRiskError::MaxPositionsReached { max: 2 })
        ));
    }

    #[test]
    fn test_close_nonexistent_position() {
        let pm = PositionManager::new(5);
        let result = pm.close("nonexistent");
        assert!(matches!(
            result,
            Err(HydraRiskError::PositionNotFound { .. })
        ));
    }

    #[test]
    fn test_get_position() {
        let pm = PositionManager::new(5);
        pm.open(make_position("pos_x", "mint_x")).unwrap();
        assert!(pm.get("pos_x").is_some());
        assert!(pm.get("missing").is_none());
    }
}
