# 🐉 Hydra Bot v3 – Copilot Instructions

## ZUERST LESEN
- `AI-MEMORY/CONTEXT.md`
- `AI-MEMORY/NEXT.md`
- `AI-MEMORY/ERRORS.md`

## Stack
Rust 2021 · tokio · 8-Crate Workspace · Triton One
Fumarole → Vixen → PumpfunParser → DeepSeek → JetTpuClient

## Core Traits (hydra-core definiert, alle Crates nutzen!)
```rust
// hydra-core/src/traits.rs
pub trait MarketDataStream: Send + Sync {
    fn next_signal(&mut self) -> impl Future<Output = Option<MintSignal>> + Send;
}
pub trait AiAnalyzer: Send + Sync {
    fn analyze(&self, s: &MintSignal) -> impl Future<Output = Option<ScoredSignal>> + Send;
}
pub trait TradeExecutor: Send + Sync {
    fn execute(&self, s: &ScoredSignal) -> impl Future<Output = anyhow::Result<()>> + Send;
}
pub trait RiskEngine: Send + Sync {
    fn approve(&self, s: &MintSignal) -> Result<(), RiskDenied>;
}
```

## Error Strategie
- Libraries (`hydra-*`): `thiserror` + eigene `HydraError` Enums
- Binaries/main.rs: `anyhow::Result` an Boundaries
- NIEMALS `anyhow` in Library-Signaturen!

## NIEMALS
- `unwrap()` außerhalb Tests
- Hardcodierte API Keys (auch nicht "test-key-for-ci"!)
- `dtolnay/rust-toolchain@stable` → immer `@v1` mit `toolchain: stable`
- Externe API Calls in Unit Tests (Mocks verwenden!)
- Blocking I/O in `async fn`

## IMMER
- `anyhow::Result<T>` nur in main.rs / CLI
- `thiserror` in allen Crates für eigene Errors
- Traits aus `hydra-core::traits` implementieren
- Tests offline-fähig (kein Netzwerk, kein echter Key!)
- `cargo fmt` + `cargo clippy -- -D warnings` vor Push

## CI Regel
```yaml
# KORREKT:
- uses: dtolnay/rust-toolchain@v1
  with:
    toolchain: stable

# FALSCH (bricht CI!):
- uses: dtolnay/rust-toolchain@stable
```

## Tests offline machen
```rust
// Mock für AI (kein echter API Call in Tests!)
struct MockAiAnalyzer { score: f64 }
impl AiAnalyzer for MockAiAnalyzer {
    async fn analyze(&self, signal: &MintSignal) -> Option<ScoredSignal> {
        Some(ScoredSignal { signal: signal.clone(), score: self.score, should_buy: self.score > 0.6 })
    }
}
```

## Defaults
| Parameter | Wert |
|---|---|
| Take Profit | +30% |
| Stop Loss | −20% |
| Max Hold | 60s |
| Circuit Breaker | 4 Verluste → 5 Min |
| DeepSeek Gate | rule_score > 0.6 |
| TX Retry | 3x · 200/400/800ms |
| Priority Fee | P75 · 1_000..=100_000 |
