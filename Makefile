.PHONY: build test format lint bench dev clean release setup

# Default Build Profile
PROFILE ?= release

# ==============================================================================
# Development Experience (DX)
# ==============================================================================

## setup: Install required toolchains and dependencies
setup:
	rustup toolchain install 1.81.0
	curl -L https://sp1.succinct.xyz | bash
	sp1up
	cd dashboard && npm install
	cargo install cargo-deny || true

## dev: Spin up the entire local environment (Soroban Localnet, Mock Prover, Dashboard)
dev:
	docker-compose up -d --build
	@echo "Sadgi Protocol is running locally!"
	@echo "Dashboard: http://localhost:3000"

## down: Tear down the local environment
down:
	docker-compose down -v

# ==============================================================================
# Build Targets
# ==============================================================================

## build: Build all targets (Contracts, Prover, SDK, CLI)
build: contracts-build prover-build sdk-build cli-build

## contracts-build: Compile Soroban smart contracts to WebAssembly
contracts-build:
	cargo build --manifest-path contracts/marketplace/Cargo.toml --target wasm32v1-none --profile $(PROFILE) --locked

## prover-build: Compile Canonical Reference Programs to RISC-V via SP1
prover-build:
	cd prover && cargo build --workspace --profile $(PROFILE) --locked

## sdk-build: Compile the Rust Developer SDK
sdk-build:
	cargo build --manifest-path sdk/sadgi-sdk/Cargo.toml --profile $(PROFILE) --locked

## cli-build: Compile the Developer CLI
cli-build:
	cargo build --manifest-path cli/sadgi-cli/Cargo.toml --profile $(PROFILE) --locked

# ==============================================================================
# Quality Assurance
# ==============================================================================

## test: Run all unit and integration tests across the workspace
test:
	cargo test --workspace --locked
	cd prover && cargo test --workspace --locked

## format: Format all Rust code
format:
	cargo fmt --all
	cd prover && cargo fmt --all

## lint: Run clippy and check for dead code/bad patterns
lint:
	cargo clippy --workspace --all-targets --locked -- -D warnings
	cd prover && cargo clippy --workspace --all-targets --locked -- -D warnings

## bench: Run performance benchmarks
bench:
	cargo bench --workspace
	cd prover && cargo bench --workspace

## audit: Scan dependencies for security vulnerabilities
audit:
	cargo deny check

## docs: Generate workspace documentation
docs:
	cargo doc --no-deps --workspace
	cd prover && cargo doc --no-deps --workspace

# ==============================================================================
# Release & Deployment
# ==============================================================================

## clean: Remove build artifacts
clean:
	cargo clean
	cd prover && cargo clean
	rm -rf target/
	rm -rf dashboard/.next/

## release: Trigger the CI/CD semantic versioning release pipeline (mocked locally)
release:
	@echo "Triggering Semantic Release Workflow..."
	@# In CI, this would trigger semantic-release or a GitHub Action

