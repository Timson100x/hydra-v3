use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    /// Phase 1: Signal received
    SignalReceived,
    /// Phase 2: AI scoring in progress
    AiScoring,
    /// Phase 3: Strategy filters applied
    StrategyFiltering,
    /// Phase 4: Risk check
    RiskCheck,
    /// Phase 5: Order submitted
    OrderSubmitted,
    /// Phase 6: Position open
    PositionOpen,
    /// Phase 7: TP/SL monitoring
    Monitoring,
    /// Phase 8: Trade closed
    TradeClosed,
}

impl Phase {
    pub fn next(&self) -> Option<Phase> {
        match self {
            Phase::SignalReceived => Some(Phase::AiScoring),
            Phase::AiScoring => Some(Phase::StrategyFiltering),
            Phase::StrategyFiltering => Some(Phase::RiskCheck),
            Phase::RiskCheck => Some(Phase::OrderSubmitted),
            Phase::OrderSubmitted => Some(Phase::PositionOpen),
            Phase::PositionOpen => Some(Phase::Monitoring),
            Phase::Monitoring => Some(Phase::TradeClosed),
            Phase::TradeClosed => None,
        }
    }

    pub fn number(&self) -> u8 {
        match self {
            Phase::SignalReceived => 1,
            Phase::AiScoring => 2,
            Phase::StrategyFiltering => 3,
            Phase::RiskCheck => 4,
            Phase::OrderSubmitted => 5,
            Phase::PositionOpen => 6,
            Phase::Monitoring => 7,
            Phase::TradeClosed => 8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_progression_is_sequential() {
        let mut phase = Phase::SignalReceived;
        let expected_numbers = [1u8, 2, 3, 4, 5, 6, 7, 8];
        for &n in &expected_numbers {
            assert_eq!(phase.number(), n);
            if let Some(next) = phase.next() {
                phase = next;
            }
        }
    }

    #[test]
    fn test_trade_closed_has_no_next() {
        assert!(Phase::TradeClosed.next().is_none());
    }

    #[test]
    fn test_all_phases_have_numbers_1_to_8() {
        let phases = [
            Phase::SignalReceived,
            Phase::AiScoring,
            Phase::StrategyFiltering,
            Phase::RiskCheck,
            Phase::OrderSubmitted,
            Phase::PositionOpen,
            Phase::Monitoring,
            Phase::TradeClosed,
        ];
        let numbers: Vec<u8> = phases.iter().map(|p| p.number()).collect();
        assert_eq!(numbers, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_phase_equality() {
        assert_eq!(Phase::SignalReceived, Phase::SignalReceived);
        assert_ne!(Phase::SignalReceived, Phase::AiScoring);
    }
}
