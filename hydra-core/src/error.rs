use thiserror::Error;

#[derive(Debug, Error)]
pub enum HydraError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("AI service error: {0}")]
    AiService(String),

    #[error("Execution error: {0}")]
    Execution(String),

    #[error("Risk threshold exceeded: {0}")]
    RiskThreshold(String),

    #[error("Circuit breaker open: {0}")]
    CircuitBreaker(String),

    #[error("Phase error: {0}")]
    Phase(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
