use std::time::Duration;
use tracing::{info, warn};

pub struct StreamReconnect {
    base_delay: Duration,
    max_delay: Duration,
    attempt: u32,
}

impl StreamReconnect {
    pub fn new(base_delay: Duration, max_delay: Duration) -> Self {
        Self {
            base_delay,
            max_delay,
            attempt: 0,
        }
    }

    /// Returns the next backoff duration and increments the attempt counter.
    pub fn next_backoff(&mut self) -> Duration {
        let delay = self
            .base_delay
            .saturating_mul(2u32.saturating_pow(self.attempt))
            .min(self.max_delay);
        self.attempt = self.attempt.saturating_add(1);
        delay
    }

    pub fn reset(&mut self) {
        self.attempt = 0;
    }

    /// Waits for the next backoff duration asynchronously.
    pub async fn wait(&mut self) {
        let current_attempt = self.attempt;
        let delay = self.next_backoff();
        warn!(attempt = current_attempt, delay_ms = delay.as_millis(), "Stream reconnect backoff");
        tokio::time::sleep(delay).await;
        info!("Stream reconnect attempting connection");
    }
}

impl Default for StreamReconnect {
    fn default() -> Self {
        Self::new(Duration::from_secs(1), Duration::from_secs(60))
    }
}
