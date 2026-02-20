.PHONY: build test clippy fmt check clean docker-up docker-down help

CARGO := cargo
DOCKER_COMPOSE := docker compose

## Build all crates in release mode
build:
	$(CARGO) build --workspace --release

## Build all crates in debug mode
build-dev:
	$(CARGO) build --workspace

## Run all unit tests
test:
	$(CARGO) test --workspace

## Run Clippy linter (deny warnings)
clippy:
	$(CARGO) clippy --workspace --all-targets -- -D warnings

## Check formatting
fmt-check:
	$(CARGO) fmt --all -- --check

## Apply formatting
fmt:
	$(CARGO) fmt --all

## Run cargo check
check:
	$(CARGO) check --workspace

## Clean build artifacts
clean:
	$(CARGO) clean

## Start monitoring stack (Prometheus + Grafana)
docker-up:
	$(DOCKER_COMPOSE) up -d

## Stop monitoring stack
docker-down:
	$(DOCKER_COMPOSE) down

## Run all CI checks locally (check + clippy + fmt + test)
ci:
	$(MAKE) check
	$(MAKE) clippy
	$(MAKE) fmt-check
	$(MAKE) test

## Show this help
help:
	@grep -E '^##' $(MAKEFILE_LIST) | sed 's/^## //'
