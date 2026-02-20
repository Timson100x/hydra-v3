# Hydra Bot v3 - GitHub Copilot Instructions

## Project Overview
This is a high-frequency crypto trading bot built in Rust, targeting the Solana blockchain. It uses DeepSeek AI for signal analysis and is structured as a Cargo workspace.

## Architecture
- **hydra-core**: Shared types, config, and error types. All other crates depend on this.
- **hydra-stream**: WebSocket stream ingestion and signal pipeline.
- **hydra-ai**: DeepSeek API client for AI-powered signal analysis.
- **hydra-strategy**: Signal filtering and risk scoring logic.
- **hydra-executor**: Trade order management and execution backends.
- **hydra-risk**: Circuit breaker pattern and daily loss guards.
- **hydra-monitor**: Prometheus metrics and alert webhooks.
- **hydra-phases**: Phase-based trading lifecycle management.

## Code Style Guidelines
1. **Always use `tokio::time::timeout`** for any external network requests (RPC, AI API, webhooks).
2. **Never log sensitive data** (private keys, API keys, wallet addresses).
3. Use `tracing` for all logging (not `println!` or `eprintln!`).
4. Use `thiserror` for error types; use `anyhow` for application-level error propagation.
5. All async functions should be `async fn`, not `fn` returning `impl Future`.
6. Use `Arc<T>` for shared state across async tasks.
7. Prefer `RwLock` over `Mutex` when reads are more frequent than writes.

## Security Rules
- **NEVER** commit `.env` files or files containing API keys, private keys, or secrets.
- All config must be loaded from environment variables or `hydra.toml` (which must not contain real secrets).
- External API calls must always have a timeout via `tokio::time::timeout`.
- Validate all external input before processing.

## Testing Guidelines
- Unit tests go in `#[cfg(test)]` modules within each source file.
- Use mock implementations (e.g., `MockBackend`, `MockTokenSource`) for unit tests.
- Integration tests go in `tests/` directories within each crate.
- Never use real API keys or RPC endpoints in tests.

## Dependency Guidelines
- Prefer workspace dependencies (`workspace = true`) over crate-local versions.
- Avoid adding new dependencies without justification.
- Use `rustls-tls` instead of `native-tls` for TLS.
