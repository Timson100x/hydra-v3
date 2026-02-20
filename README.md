# 🐉 Hydra Bot v3

A high-performance crypto trading bot built with Rust, targeting the Solana blockchain. Uses DeepSeek AI for signal analysis, structured as a Cargo workspace.

## Architecture

```
hydra-v3/
├── hydra-core/        # Shared types, config, errors
├── hydra-stream/      # Signal pipeline & token streaming
├── hydra-ai/          # DeepSeek AI API client
├── hydra-strategy/    # Signal filtering & risk scoring
├── hydra-executor/    # Trade execution & transaction management
├── hydra-risk/        # Circuit breaker & daily loss guard
├── hydra-monitor/     # Prometheus metrics & alerts
└── hydra-phases/      # Phase-based trading lifecycle
```

## Prerequisites

- Rust 1.75+ (`rustup update`)
- Docker & Docker Compose (for monitoring)
- DeepSeek API key
- Solana wallet

## Installation

```bash
# 1. Clone the repository
git clone https://github.com/Timson100x/hydra-v3.git
cd hydra-v3

# 2. Copy and configure environment
cp .env.example .env
# Edit .env and fill in your API keys and wallet

# 3. Build all crates
cargo build --workspace --release
```

## Configuration

Edit `hydra.toml` for general configuration (no secrets here):

```toml
[network]
rpc_url = "https://api.mainnet-beta.solana.com"
commitment = "confirmed"

[risk]
max_position_size_sol = 1.0
max_daily_loss_sol = 5.0
```

Edit `phase.toml` to define trading phases.

Set secrets in `.env` (see `.env.example`).

## Makefile Commands

| Command | Description |
|---------|-------------|
| `make build` | Build all crates (release) |
| `make build-dev` | Build all crates (debug) |
| `make test` | Run all unit tests |
| `make clippy` | Run Clippy linter |
| `make fmt` | Format all code |
| `make fmt-check` | Check formatting |
| `make check` | Run cargo check |
| `make clean` | Clean build artifacts |
| `make docker-up` | Start Prometheus + Grafana |
| `make docker-down` | Stop monitoring stack |
| `make ci` | Run all CI checks locally |

## Monitoring

Start the monitoring stack:

```bash
make docker-up
```

- **Prometheus**: http://localhost:9090
- **Grafana**: http://localhost:3000 (default login: `admin` / `hydra_admin`)

## Security

- **Never commit `.env` or any file containing real API keys or private keys.**
- All secrets are loaded from environment variables at runtime.
- All external requests use `tokio::time::timeout` to prevent hangs.
- The circuit breaker (`hydra-risk`) will open if too many consecutive failures occur.

## CI/CD

GitHub Actions runs on every push and PR:
- `cargo check` - compilation check
- `cargo clippy` - linting
- `cargo test` - unit tests  
- `cargo fmt --check` - formatting

## License

MIT
