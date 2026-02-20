use tokio::sync::RwLock;
use tracing::{info, warn};
use anyhow::Result;
use crate::phase::{Phase, PhaseConfig, PhaseState};

pub struct PhaseManager {
    phases: RwLock<Vec<Phase>>,
    current_index: RwLock<Option<usize>>,
}

impl PhaseManager {
    pub fn new() -> Self {
        Self {
            phases: RwLock::new(Vec::new()),
            current_index: RwLock::new(None),
        }
    }

    pub async fn add_phase(&self, config: PhaseConfig) {
        let phase = Phase::new(config);
        info!("Adding phase: {}", phase.config.name);
        self.phases.write().await.push(phase);
    }

    pub async fn start_next_phase(&self) -> Result<String> {
        let mut phases = self.phases.write().await;
        let mut current = self.current_index.write().await;

        if let Some(idx) = *current {
            if idx < phases.len() && phases[idx].is_active() {
                warn!("Phase {} is still active", phases[idx].config.name);
                return Ok(phases[idx].config.name.clone());
            }
        }

        let next_idx = current.map(|i| i + 1).unwrap_or(0);
        if next_idx >= phases.len() {
            return Err(anyhow::anyhow!("No more phases available"));
        }

        phases[next_idx].activate();
        let name = phases[next_idx].config.name.clone();
        *current = Some(next_idx);
        info!("Started phase: {}", name);
        Ok(name)
    }

    pub async fn current_phase_name(&self) -> Option<String> {
        let phases = self.phases.read().await;
        let current = self.current_index.read().await;
        current.and_then(|i| phases.get(i).map(|p| p.config.name.clone()))
    }

    pub async fn phase_count(&self) -> usize {
        self.phases.read().await.len()
    }

    pub async fn current_phase_state(&self) -> Option<PhaseState> {
        let phases = self.phases.read().await;
        let current = self.current_index.read().await;
        current.and_then(|i| phases.get(i).map(|p| p.state.clone()))
    }
}

impl Default for PhaseManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_config(name: &str) -> PhaseConfig {
        PhaseConfig {
            name: name.to_string(),
            description: "Test".to_string(),
            max_trades: 5,
            max_position_sol: 1.0,
            min_confidence: 0.7,
            duration_hours: None,
        }
    }

    #[tokio::test]
    async fn test_phase_manager() {
        let manager = PhaseManager::new();
        manager.add_phase(make_config("Phase 1")).await;
        manager.add_phase(make_config("Phase 2")).await;
        assert_eq!(manager.phase_count().await, 2);

        let name = manager.start_next_phase().await.unwrap();
        assert_eq!(name, "Phase 1");
        assert_eq!(manager.current_phase_state().await, Some(PhaseState::Active));

        let name2 = manager.start_next_phase().await.unwrap();
        // Phase 1 still active, so same phase returned
        assert_eq!(name2, "Phase 1");
    }
}
