use hydra_ai::AiAnalysis;
use hydra_core::{Signal, TokenInfo};
use tracing::{debug, info};

/// Threshold above which the rule-based score is treated as a buy signal.
pub const RULE_BUY_THRESHOLD: f64 = 0.6;

pub struct RiskScorer {
    max_market_cap_usd: f64,
}

impl RiskScorer {
    pub fn new(max_market_cap_usd: f64) -> Self {
        Self { max_market_cap_usd }
    }

    /// Computes a composite risk score for the given signal and token.
    ///
    /// When `ai_analysis` is `Some`, the AI score is blended in (40 %).
    /// When `ai_analysis` is `None` (e.g. after a DeepSeek timeout), the
    /// rule-based `rule_score` is used exclusively and a buy is recommended
    /// when `rule_score > RULE_BUY_THRESHOLD` (0.6).
    pub fn score(
        &self,
        signal: &Signal,
        token_info: &TokenInfo,
        ai_analysis: Option<&AiAnalysis>,
    ) -> f64 {
        let rule_score = self.rule_score(signal, token_info);

        let composite = match ai_analysis {
            Some(ai) => {
                debug!(
                    ai_score = ai.score,
                    rule_score,
                    "Blending AI score with rule score"
                );
                rule_score * 0.6 + ai.score * 0.4
            }
            None => {
                info!(
                    rule_score,
                    "AI unavailable — using rule score only (buy threshold: {})",
                    RULE_BUY_THRESHOLD
                );
                rule_score
            }
        };

        composite.clamp(0.0, 1.0)
    }

    /// Returns `true` when the composite score indicates a buy.
    ///
    /// When `ai_analysis` is `None` the decision is made purely on
    /// `rule_score > RULE_BUY_THRESHOLD`.
    pub fn should_buy(
        &self,
        signal: &Signal,
        token_info: &TokenInfo,
        ai_analysis: Option<&AiAnalysis>,
    ) -> bool {
        let composite = self.score(signal, token_info, ai_analysis);
        composite > RULE_BUY_THRESHOLD
    }

    fn rule_score(&self, signal: &Signal, token_info: &TokenInfo) -> f64 {
        let confidence_score = signal.confidence;
        let liquidity_score = (token_info.liquidity_usd / 1_000_000.0).min(1.0);
        let market_cap_score = if token_info.market_cap_usd > 0.0 {
            1.0 - (token_info.market_cap_usd / self.max_market_cap_usd).min(1.0)
        } else {
            0.5
        };

        let score =
            confidence_score * 0.5 + liquidity_score * 0.3 + market_cap_score * 0.2;
        debug!(
            "Rule score for {}: {:.3} (confidence={:.2}, liquidity={:.2}, mcap={:.2})",
            token_info.address, score, confidence_score, liquidity_score, market_cap_score
        );
        score.clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hydra_core::{SignalType, TokenAddress};

    fn make_token(addr: &str) -> TokenInfo {
        TokenInfo {
            address: TokenAddress::new(addr),
            symbol: "TEST".to_string(),
            name: "Test Token".to_string(),
            decimals: 9,
            liquidity_usd: 500_000.0,
            price_usd: 0.001,
            volume_24h_usd: 100_000.0,
            market_cap_usd: 1_000_000.0,
        }
    }

    #[test]
    fn test_risk_scorer_with_ai() {
        let addr = TokenAddress::new("TestToken11111111111111111111111111111111111");
        let signal = Signal::new(addr.clone(), SignalType::Buy, 0.8);
        let token_info = make_token("TestToken11111111111111111111111111111111111");
        let ai = AiAnalysis {
            score: 0.9,
            action: "BUY".to_string(),
            reasoning: "Strong signal".to_string(),
        };
        let scorer = RiskScorer::new(10_000_000.0);
        let score = scorer.score(&signal, &token_info, Some(&ai));
        assert!(score > 0.0 && score <= 1.0);
    }

    #[test]
    fn test_risk_scorer_no_ai_uses_rule_score() {
        let addr = TokenAddress::new("TestToken11111111111111111111111111111111111");
        let signal = Signal::new(addr.clone(), SignalType::Buy, 0.8);
        let token_info = make_token("TestToken11111111111111111111111111111111111");
        let scorer = RiskScorer::new(10_000_000.0);

        let score_no_ai = scorer.score(&signal, &token_info, None);
        let rule = scorer.rule_score(&signal, &token_info);
        // Without AI the composite equals the rule score exactly.
        assert!((score_no_ai - rule).abs() < f64::EPSILON);
    }

    #[test]
    fn test_should_buy_above_threshold_no_ai() {
        let addr = TokenAddress::new("TestToken11111111111111111111111111111111111");
        // High confidence + good liquidity → rule_score > 0.6
        let signal = Signal::new(addr.clone(), SignalType::Buy, 0.9);
        let token_info = make_token("TestToken11111111111111111111111111111111111");
        let scorer = RiskScorer::new(10_000_000.0);
        assert!(scorer.should_buy(&signal, &token_info, None));
    }

    #[test]
    fn test_should_not_buy_below_threshold_no_ai() {
        let addr = TokenAddress::new("TestToken11111111111111111111111111111111111");
        // Very low confidence → rule_score ≤ 0.6
        let signal = Signal::new(addr.clone(), SignalType::Buy, 0.1);
        let token_info = TokenInfo {
            address: addr,
            symbol: "WEAK".to_string(),
            name: "Weak Token".to_string(),
            decimals: 9,
            liquidity_usd: 1_000.0,  // low liquidity
            price_usd: 0.0001,
            volume_24h_usd: 100.0,
            market_cap_usd: 9_000_000.0, // near max cap
        };
        let scorer = RiskScorer::new(10_000_000.0);
        assert!(!scorer.should_buy(&signal, &token_info, None));
    }
}
