#!/bin/bash
set -e

# This script generates the Groth16 Verifier contract using soroban-verifier-gen
# It expects the `generate_verifier` Rust script to have run first to populate the `data` directory.

DATA_DIR="../contracts/verifier/data"
OUTPUT_DIR="../contracts/verifier/src"

echo "Building Soroban Verifier Contract..."

if [ ! -f "$DATA_DIR/proof.bin" ]; then
    echo "Error: proof.bin not found. Run generate_verifier first."
    exit 1
fi

# We assume soroban-verifier-gen is installed globally
soroban-verifier-gen \
    --proof $DATA_DIR/proof.bin \
    --vk $DATA_DIR/vk.bin \
    --public-values $DATA_DIR/public_values.bin \
    --out-dir $OUTPUT_DIR

echo "Verifier successfully generated in $OUTPUT_DIR!"
