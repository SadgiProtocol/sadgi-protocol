.PHONY: build test format lint bench dev clean release

# Default Build Profile
PROFILE ?= release

# ==============================================================================
# Development Experience (DX)
# ==============================================================================

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

## build: Build all targets (Contracts, Programs, SDK, CLI)
build: contracts-build programs-build sdk-build cli-build

## contracts-build: Compile Soroban smart contracts to WebAssembly
contracts-build:
	cargo build --manifest-path contracts/marketplace/Cargo.toml --target wasm32-unknown-unknown --profile $(PROFILE) --locked

## programs-build: Compile Canonical Reference Programs to RISC-V
programs-build:
	cargo build --manifest-path programs/hello_world/Cargo.toml --profile $(PROFILE) --locked
	cargo build --manifest-path programs/hash_verification/Cargo.toml --profile $(PROFILE) --locked
	cargo build --manifest-path programs/identity/Cargo.toml --profile $(PROFILE) --locked
	cargo build --manifest-path programs/credit/Cargo.toml --profile $(PROFILE) --locked

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

## format: Format all Rust code
format:
	cargo fmt --all

## lint: Run clippy and check for dead code/bad patterns
lint:
	cargo clippy --workspace --all-targets --all-features --locked -- -D warnings

## bench: Run performance benchmarks
bench:
	cargo bench --workspace

## audit: Scan dependencies for security vulnerabilities
audit:
	cargo audit

## docs: Generate workspace documentation
docs:
	cargo doc --no-deps --workspace

# ==============================================================================
# Release & Deployment
# ==============================================================================

## clean: Remove build artifacts
clean:
	cargo clean
	rm -rf target/
	rm -rf dashboard/.next/

## release: Trigger the CI/CD semantic versioning release pipeline (mocked locally)
release:
	@echo "Triggering Semantic Release Workflow..."
	@# In CI, this would trigger semantic-release or a GitHub Action
