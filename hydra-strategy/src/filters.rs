use anyhow::Result;
use hydra_core::signal::MintSignal;
use tracing::info;

pub struct McapFilter {
    pub min_usd: f64,
    pub max_usd: f64,
}

impl McapFilter {
    pub fn new(min_usd: f64, max_usd: f64) -> Self {
        Self { min_usd, max_usd }
    }

    pub fn passes(&self, signal: &MintSignal) -> bool {
        let passes = signal.market_cap_usd >= self.min_usd && signal.market_cap_usd <= self.max_usd;
        if !passes {
            info!(
                mint = %signal.mint_address,
                mcap = signal.market_cap_usd,
                "McapFilter rejected"
            );
        }
        passes
    }
}

pub struct ZScoreFilter {
    mean: f64,
    std_dev: f64,
    threshold: f64,
}

impl ZScoreFilter {
    pub fn new(mean: f64, std_dev: f64, threshold: f64) -> Self {
        Self {
            mean,
            std_dev,
            threshold,
        }
    }

    pub fn z_score(&self, value: f64) -> f64 {
        if self.std_dev == 0.0 {
            return 0.0;
        }
        (value - self.mean) / self.std_dev
    }

    pub fn passes(&self, signal: &MintSignal) -> bool {
        let z = self.z_score(signal.volume_24h_usd);
        let passes = z >= self.threshold;
        if !passes {
            info!(
                mint = %signal.mint_address,
                z_score = z,
                threshold = self.threshold,
                "ZScoreFilter rejected"
            );
        }
        passes
    }
}

pub struct RugCheckFilter {
    pub min_liquidity_usd: f64,
    pub max_top_holder_pct: f64,
    pub min_holder_count: u64,
}

impl RugCheckFilter {
    pub fn new(min_liquidity_usd: f64, max_top_holder_pct: f64, min_holder_count: u64) -> Self {
        Self {
            min_liquidity_usd,
            max_top_holder_pct,
            min_holder_count,
        }
    }

    pub fn passes(&self, signal: &MintSignal) -> Result<bool> {
        if signal.liquidity_usd < self.min_liquidity_usd {
            info!(
                mint = %signal.mint_address,
                liquidity = signal.liquidity_usd,
                "RugCheckFilter: insufficient liquidity"
            );
            return Ok(false);
        }
        if signal.top_holder_pct > self.max_top_holder_pct {
            info!(
                mint = %signal.mint_address,
                top_holder_pct = signal.top_holder_pct,
                "RugCheckFilter: top holder concentration too high"
            );
            return Ok(false);
        }
        if signal.holder_count < self.min_holder_count {
            info!(
                mint = %signal.mint_address,
                holder_count = signal.holder_count,
                "RugCheckFilter: too few holders"
            );
            return Ok(false);
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_signal(
        mcap: f64,
        volume: f64,
        liquidity: f64,
        top_holder: f64,
        holders: u64,
    ) -> MintSignal {
        MintSignal::new(
            "test_mint".to_string(),
            mcap,
            volume,
            0.001,
            holders,
            liquidity,
            top_holder,
        )
    }

    #[test]
    fn test_mcap_filter_passes() {
        let f = McapFilter::new(1_000.0, 100_000.0);
        assert!(f.passes(&make_signal(50_000.0, 0.0, 0.0, 0.0, 0)));
    }

    #[test]
    fn test_mcap_filter_rejects_below() {
        let f = McapFilter::new(1_000.0, 100_000.0);
        assert!(!f.passes(&make_signal(500.0, 0.0, 0.0, 0.0, 0)));
    }

    #[test]
    fn test_mcap_filter_rejects_above() {
        let f = McapFilter::new(1_000.0, 100_000.0);
        assert!(!f.passes(&make_signal(200_000.0, 0.0, 0.0, 0.0, 0)));
    }

    #[test]
    fn test_zscore_filter_passes() {
        // mean=1000, std=500, threshold=1.0 → z=(1600-1000)/500=1.2 ≥ 1.0
        let f = ZScoreFilter::new(1_000.0, 500.0, 1.0);
        assert!(f.passes(&make_signal(0.0, 1_600.0, 0.0, 0.0, 0)));
    }

    #[test]
    fn test_zscore_filter_rejects() {
        // mean=1000, std=500, threshold=2.0 → z=(1200-1000)/500=0.4 < 2.0
        let f = ZScoreFilter::new(1_000.0, 500.0, 2.0);
        assert!(!f.passes(&make_signal(0.0, 1_200.0, 0.0, 0.0, 0)));
    }

    #[test]
    fn test_zscore_zero_std_dev() {
        let f = ZScoreFilter::new(1_000.0, 0.0, 0.0);
        assert_eq!(f.z_score(9_999.0), 0.0);
    }

    #[test]
    fn test_rug_check_passes() {
        let f = RugCheckFilter::new(5_000.0, 30.0, 100);
        let s = make_signal(0.0, 0.0, 10_000.0, 20.0, 200);
        assert!(f.passes(&s).unwrap());
    }

    #[test]
    fn test_rug_check_low_liquidity() {
        let f = RugCheckFilter::new(5_000.0, 30.0, 100);
        let s = make_signal(0.0, 0.0, 100.0, 10.0, 200);
        assert!(!f.passes(&s).unwrap());
    }

    #[test]
    fn test_rug_check_high_top_holder() {
        let f = RugCheckFilter::new(1_000.0, 30.0, 100);
        let s = make_signal(0.0, 0.0, 5_000.0, 90.0, 200);
        assert!(!f.passes(&s).unwrap());
    }

    #[test]
    fn test_rug_check_too_few_holders() {
        let f = RugCheckFilter::new(1_000.0, 50.0, 100);
        let s = make_signal(0.0, 0.0, 5_000.0, 20.0, 10);
        assert!(!f.passes(&s).unwrap());
    }
}
