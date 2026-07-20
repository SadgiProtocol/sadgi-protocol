# Sadgi Operations Guide

Running a Sadgi Prover Node is a critical piece of the protocol's infrastructure. By running a node, you earn XLM bounties for successfully generating Zero-Knowledge proofs for developers on the Stellar network.

## Hardware Requirements
- **CPU**: Multi-core processor (RISC Zero proving is highly parallelizable).
- **RAM**: Minimum 16GB (32GB+ recommended for complex state transitions).
- **Storage**: 100GB SSD for maintaining the Soroban ledger index.

## Running a Node
The Prover node is bundled as a Docker container.

```bash
docker run -d --name sadgi-prover \
  -e STELLAR_RPC_URL="https://rpc-futurenet.stellar.org:443" \
  -e MARKETPLACE_CONTRACT_ID="C..." \
  -e PROVER_SECRET_KEY="S..." \
  sadgiprotocol/prover-node:latest
```

## Capacity Advertising
Upon initialization, your node will automatically benchmark your hardware and send an on-chain transaction to the Compute Marketplace. The Scheduler will use this `Capacity` metric to intelligently route computationally heavy jobs to your node.
