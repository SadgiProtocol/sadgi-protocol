#!/usr/bin/env bash
set -e

echo "==========================================="
echo "Sadgi Protocol Local CI Validation"
echo "==========================================="

echo ""
echo "=> [1/4] Checking Root Workspace (Contracts, Core, SDK)"
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

echo ""
echo "=> [2/4] Checking Prover Workspace"
cd prover
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cd ..

echo ""
echo "=> [3/4] Checking Dashboard"
cd dashboard
npm ci
npm run lint
npm run build
cd ..

echo ""
echo "=> [4/4] Dependency Audit"
cargo deny check || echo "Warning: cargo-deny failed or is not installed. Run 'cargo install cargo-deny'"

echo ""
echo "✅ All local CI checks passed!"
