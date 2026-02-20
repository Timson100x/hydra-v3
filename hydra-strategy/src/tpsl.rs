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
