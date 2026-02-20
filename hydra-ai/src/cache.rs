use chrono::{DateTime, Utc};
use dashmap::DashMap;
use std::time::Duration;

use crate::scorer::AiScore;

struct CacheEntry {
    score: AiScore,
    inserted_at: DateTime<Utc>,
}

pub struct ScoreCache {
    inner: DashMap<String, CacheEntry>,
    ttl: Duration,
}

impl ScoreCache {
    pub fn new(ttl: Duration) -> Self {
        Self {
            inner: DashMap::new(),
            ttl,
        }
    }

    pub fn get(&self, mint: &str) -> Option<AiScore> {
        let entry = self.inner.get(mint)?;
        let age = Utc::now()
            .signed_duration_since(entry.inserted_at)
            .to_std()
            .unwrap_or(Duration::MAX);
        if age > self.ttl {
            drop(entry);
            self.inner.remove(mint);
            return None;
        }
        Some(entry.score.clone())
    }

    pub fn insert(&self, mint: String, score: AiScore) {
        self.inner.insert(
            mint,
            CacheEntry {
                score,
                inserted_at: Utc::now(),
            },
        );
    }

    pub fn remove(&self, mint: &str) {
        self.inner.remove(mint);
    }
}

impl Default for ScoreCache {
    fn default() -> Self {
        Self::new(Duration::from_secs(300))
    }
}
