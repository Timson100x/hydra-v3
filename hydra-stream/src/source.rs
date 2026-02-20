use async_trait::async_trait;
use hydra_core::{HydraError, TokenInfo};

#[async_trait]
pub trait TokenSource: Send + Sync {
    async fn fetch_new_tokens(&self) -> Result<Vec<TokenInfo>, HydraError>;
}

pub struct MockTokenSource {
    tokens: Vec<TokenInfo>,
}

impl MockTokenSource {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        Self { tokens }
    }
}

#[async_trait]
impl TokenSource for MockTokenSource {
    async fn fetch_new_tokens(&self) -> Result<Vec<TokenInfo>, HydraError> {
        Ok(self.tokens.clone())
    }
}
