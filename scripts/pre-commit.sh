#!/usr/bin/env bash
#
# A pre-commit hook that runs code formatting before allowing a commit.
#
# To install: ln -s ../../scripts/pre-commit.sh .git/hooks/pre-commit

set -e

echo "Running pre-commit checks..."

# Check formatting in Root Workspace
cargo fmt --all -- --check || { echo "Root Workspace formatting failed! Run 'cargo fmt --all'"; exit 1; }

# Check formatting in Prover Workspace
cd prover
cargo fmt --all -- --check || { echo "Prover Workspace formatting failed! Run 'cd prover && cargo fmt --all'"; exit 1; }
cd ..

# Optional: Add JS/TS formatting here
# cd dashboard && npm run lint || { exit 1; }

echo "All checks passed. Proceeding with commit."
