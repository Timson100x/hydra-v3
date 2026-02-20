use hydra_core::{Signal, SignalType};
use tracing::debug;

pub struct SignalFilter {
    min_confidence: f64,
    allowed_types: Vec<SignalType>,
}

impl SignalFilter {
    pub fn new(min_confidence: f64, allowed_types: Vec<SignalType>) -> Self {
        Self {
            min_confidence,
            allowed_types,
        }
    }

    pub fn apply(&self, signals: Vec<Signal>) -> Vec<Signal> {
        let before = signals.len();
        let filtered: Vec<Signal> = signals
            .into_iter()
            .filter(|s| {
                s.confidence >= self.min_confidence
                    && self.allowed_types.contains(&s.signal_type)
            })
            .collect();
        debug!(
            "Filter: {} -> {} signals (min_confidence={:.2})",
            before,
            filtered.len(),
            self.min_confidence
        );
        filtered
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hydra_core::TokenAddress;

    #[test]
    fn test_filter_by_confidence() {
        let token = TokenAddress::new("TestToken11111111111111111111111111111111111");
        let signals = vec![
            Signal::new(token.clone(), SignalType::Buy, 0.9),
            Signal::new(token.clone(), SignalType::Buy, 0.4),
            Signal::new(token.clone(), SignalType::Buy, 0.75),
        ];
        let filter = SignalFilter::new(0.7, vec![SignalType::Buy]);
        let result = filter.apply(signals);
        assert_eq!(result.len(), 2);
        for s in &result {
            assert!(s.confidence >= 0.7);
        }
    }

    #[test]
    fn test_filter_by_type() {
        let token = TokenAddress::new("TestToken11111111111111111111111111111111111");
        let signals = vec![
            Signal::new(token.clone(), SignalType::Buy, 0.9),
            Signal::new(token.clone(), SignalType::Sell, 0.9),
            Signal::new(token.clone(), SignalType::Hold, 0.9),
        ];
        let filter = SignalFilter::new(0.0, vec![SignalType::Buy]);
        let result = filter.apply(signals);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].signal_type, SignalType::Buy);
    }
}
