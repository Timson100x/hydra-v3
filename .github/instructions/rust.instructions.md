---
applyTo: "**/*.rs"
---

## Error Handling (wichtigste Regel!)
- Crates (Libraries): `thiserror` + HydraError enum
- main.rs / CLI: `anyhow::Result`
- NIEMALS anyhow in pub Library-Funktionen!

## Traits vor Structs
- Neue Funktionalität → erst Trait in hydra-core definieren
- Dann impl für echte Struct
- Dann MockStruct für Tests

## Tests
- `#[cfg(test)] mod tests` am Ende jeder Datei
- Alle externen Calls mocken (Trait verwenden!)
- KEIN echter API Key oder Netzwerk in Tests

## CI-sichere Patterns
```rust
// Config aus env mit Default (CI-safe!)
let api_key = std::env::var("DEEPSEEK_API_KEY")
    .unwrap_or_else(|_| "offline".to_string());
if api_key == "offline" {
    return Ok(None); // graceful degradation
}
```
