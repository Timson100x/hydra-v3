use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::phase::Phase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseEvent {
    pub phase: Phase,
    pub entered_at: DateTime<Utc>,
}

pub struct PhaseTracker {
    trade_id: String,
    current_phase: Phase,
    history: Vec<PhaseEvent>,
}

impl PhaseTracker {
    pub fn new(trade_id: String) -> Self {
        let initial_phase = Phase::SignalReceived;
        let history = vec![PhaseEvent {
            phase: initial_phase.clone(),
            entered_at: Utc::now(),
        }];
        Self {
            trade_id,
            current_phase: initial_phase,
            history,
        }
    }

    pub fn current_phase(&self) -> &Phase {
        &self.current_phase
    }

    pub fn advance(&mut self) -> Result<&Phase> {
        match self.current_phase.next() {
            Some(next) => {
                info!(
                    trade_id = %self.trade_id,
                    from = self.current_phase.number(),
                    to = next.number(),
                    "Phase transition"
                );
                self.current_phase = next.clone();
                self.history.push(PhaseEvent {
                    phase: next,
                    entered_at: Utc::now(),
                });
                Ok(&self.current_phase)
            }
            None => {
                bail!(
                    "Trade {} is already in final phase (TradeClosed)",
                    self.trade_id
                );
            }
        }
    }

    pub fn history(&self) -> &[PhaseEvent] {
        &self.history
    }

    pub fn is_complete(&self) -> bool {
        self.current_phase == Phase::TradeClosed
    }
}
