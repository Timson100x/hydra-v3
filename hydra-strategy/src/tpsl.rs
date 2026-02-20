#[derive(Debug, Clone)]
pub struct TpSlLevels {
    pub take_profit_pct: f64,
    pub stop_loss_pct: f64,
}

pub struct TpSlCalculator {
    default_tp_pct: f64,
    default_sl_pct: f64,
}

impl TpSlCalculator {
    pub fn new(default_tp_pct: f64, default_sl_pct: f64) -> Self {
        Self {
            default_tp_pct,
            default_sl_pct,
        }
    }

    /// Calculate TP/SL levels based on AI confidence score.
    /// Higher confidence -> tighter stop loss, higher take profit.
    pub fn calculate(&self, ai_confidence: f64) -> TpSlLevels {
        let confidence_factor = ai_confidence.clamp(0.0, 1.0);
        let tp = self.default_tp_pct * (1.0 + confidence_factor);
        let sl = self.default_sl_pct * (1.0 - confidence_factor * 0.5);
        TpSlLevels {
            take_profit_pct: tp,
            stop_loss_pct: sl,
        }
    }
}

impl Default for TpSlCalculator {
    fn default() -> Self {
        Self::new(0.5, 0.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tpsl_zero_confidence() {
        let calc = TpSlCalculator::new(0.30, 0.20);
        let levels = calc.calculate(0.0);
        assert!((levels.take_profit_pct - 0.30).abs() < 1e-10);
        assert!((levels.stop_loss_pct - 0.20).abs() < 1e-10);
    }

    #[test]
    fn test_tpsl_full_confidence() {
        let calc = TpSlCalculator::new(0.30, 0.20);
        let levels = calc.calculate(1.0);
        // tp = 0.30 * (1.0 + 1.0) = 0.60
        assert!((levels.take_profit_pct - 0.60).abs() < 1e-10);
        // sl = 0.20 * (1.0 - 0.5) = 0.10
        assert!((levels.stop_loss_pct - 0.10).abs() < 1e-10);
    }

    #[test]
    fn test_tpsl_clamped_above_one() {
        let calc = TpSlCalculator::new(0.30, 0.20);
        let levels_clamped = calc.calculate(2.0);
        let levels_one = calc.calculate(1.0);
        assert!((levels_clamped.take_profit_pct - levels_one.take_profit_pct).abs() < 1e-10);
    }

    #[test]
    fn test_tpsl_default_params() {
        let calc = TpSlCalculator::default();
        let levels = calc.calculate(0.5);
        // tp = 0.5 * 1.5 = 0.75
        assert!((levels.take_profit_pct - 0.75).abs() < 1e-10);
        // sl = 0.1 * 0.75 = 0.075
        assert!((levels.stop_loss_pct - 0.075).abs() < 1e-10);
    }
}
