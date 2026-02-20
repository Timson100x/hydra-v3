use hydra_core::Signal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisPrompt {
    pub system: String,
    pub user: String,
}

impl AnalysisPrompt {
    pub fn for_signal(signal: &Signal) -> Self {
        Self {
            system: "You are a crypto trading analyst. Analyze the given signal and respond with a JSON object containing: score (0.0-1.0), action (BUY/SELL/HOLD), and reasoning (string). Be concise.".to_string(),
            user: format!(
                "Signal: token={}, type={:?}, confidence={:.2}, timestamp={}",
                signal.token,
                signal.signal_type,
                signal.confidence,
                signal.timestamp
            ),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiAnalysis {
    pub score: f64,
    pub action: String,
    pub reasoning: String,
}
