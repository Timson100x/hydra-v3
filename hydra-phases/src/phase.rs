use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PhaseState {
    Inactive,
    Active,
    Paused,
    Completed,
    Failed,
}

impl std::fmt::Display for PhaseState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PhaseState::Inactive => write!(f, "Inactive"),
            PhaseState::Active => write!(f, "Active"),
            PhaseState::Paused => write!(f, "Paused"),
            PhaseState::Completed => write!(f, "Completed"),
            PhaseState::Failed => write!(f, "Failed"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseConfig {
    pub name: String,
    pub description: String,
    pub max_trades: u32,
    pub max_position_sol: f64,
    pub min_confidence: f64,
    pub duration_hours: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phase {
    pub config: PhaseConfig,
    pub state: PhaseState,
    pub trades_executed: u32,
    pub total_pnl_sol: f64,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
}

impl Phase {
    pub fn new(config: PhaseConfig) -> Self {
        Self {
            config,
            state: PhaseState::Inactive,
            trades_executed: 0,
            total_pnl_sol: 0.0,
            started_at: None,
            ended_at: None,
        }
    }

    pub fn activate(&mut self) {
        self.state = PhaseState::Active;
        self.started_at = Some(Utc::now());
    }

    pub fn pause(&mut self) {
        if self.state == PhaseState::Active {
            self.state = PhaseState::Paused;
        }
    }

    pub fn complete(&mut self) {
        self.state = PhaseState::Completed;
        self.ended_at = Some(Utc::now());
    }

    pub fn fail(&mut self, _reason: &str) {
        self.state = PhaseState::Failed;
        self.ended_at = Some(Utc::now());
    }

    pub fn is_active(&self) -> bool {
        self.state == PhaseState::Active
    }

    pub fn can_trade(&self) -> bool {
        self.is_active() && self.trades_executed < self.config.max_trades
    }

    pub fn record_trade(&mut self, pnl_sol: f64) {
        self.trades_executed += 1;
        self.total_pnl_sol += pnl_sol;
        if self.trades_executed >= self.config.max_trades {
            self.complete();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_phase_config() -> PhaseConfig {
        PhaseConfig {
            name: "Test Phase".to_string(),
            description: "A test phase".to_string(),
            max_trades: 10,
            max_position_sol: 1.0,
            min_confidence: 0.7,
            duration_hours: None,
        }
    }

    #[test]
    fn test_phase_lifecycle() {
        let mut phase = Phase::new(default_phase_config());
        assert_eq!(phase.state, PhaseState::Inactive);
        assert!(!phase.is_active());

        phase.activate();
        assert_eq!(phase.state, PhaseState::Active);
        assert!(phase.is_active());
        assert!(phase.can_trade());

        phase.pause();
        assert_eq!(phase.state, PhaseState::Paused);
        assert!(!phase.is_active());
    }

    #[test]
    fn test_phase_completes_after_max_trades() {
        let mut phase = Phase::new(default_phase_config());
        phase.activate();
        for i in 0..10 {
            assert!(phase.can_trade(), "Should be able to trade on iteration {}", i);
            phase.record_trade(0.01);
        }
        assert_eq!(phase.state, PhaseState::Completed);
        assert!(!phase.can_trade());
    }
}
