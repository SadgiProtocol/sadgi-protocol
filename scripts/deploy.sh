#!/bin/bash
set -e

echo "====================================================="
echo " Sadgi Protocol: Soroban Testnet Deployment Script "
echo "====================================================="

NETWORK="testnet"
SOURCE="default"

# Ensure we have the stellar CLI installed
if ! command -v stellar &> /dev/null; then
    echo "❌ stellar-cli could not be found. Please install it first."
    exit 1
fi

echo "📦 1. Building Smart Contracts..."
cargo build --manifest-path ../contracts/registry/Cargo.toml --target wasm32-unknown-unknown --profile release
cargo build --manifest-path ../contracts/verifier/Cargo.toml --target wasm32-unknown-unknown --profile release
cargo build --manifest-path ../contracts/marketplace/Cargo.toml --target wasm32-unknown-unknown --profile release

REGISTRY_WASM="../target/wasm32-unknown-unknown/release/sadgi_registry.wasm"
VERIFIER_WASM="../target/wasm32-unknown-unknown/release/sadgi_verifier.wasm"
MARKETPLACE_WASM="../target/wasm32-unknown-unknown/release/sadgi_marketplace.wasm"

echo "🚀 2. Deploying to $NETWORK..."

# Deploy Registry
echo "Deploying Program Registry..."
REGISTRY_ID=$(stellar contract deploy --wasm $REGISTRY_WASM --source $SOURCE --network $NETWORK)
echo "✅ Registry deployed: $REGISTRY_ID"

# Deploy Verifier
echo "Deploying Groth16 Verifier..."
VERIFIER_ID=$(stellar contract deploy --wasm $VERIFIER_WASM --source $SOURCE --network $NETWORK)
echo "✅ Verifier deployed: $VERIFIER_ID"

# Deploy Marketplace
echo "Deploying Marketplace..."
MARKETPLACE_ID=$(stellar contract deploy --wasm $MARKETPLACE_WASM --source $SOURCE --network $NETWORK)
echo "✅ Marketplace deployed: $MARKETPLACE_ID"

echo "🔗 3. Writing deployments.json for Frontend Dashboard..."

cat <<EOF > ../dashboard/public/deployments.json
{
  "network": "$NETWORK",
  "contracts": {
    "registry": "$REGISTRY_ID",
    "verifier": "$VERIFIER_ID",
    "marketplace": "$MARKETPLACE_ID"
  },
  "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
}
EOF

echo "✅ Deployments successfully saved to dashboard/public/deployments.json!"
echo "🎉 Protocol is ready on testnet!"
