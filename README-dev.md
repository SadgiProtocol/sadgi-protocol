# Sadgi Local Development Guide

This guide is for developers who want to run the Sadgi Protocol locally to test their Soroban Smart Contracts and zkVM Guest Programs.

## Prerequisites
- Docker & Docker Compose
- Rust (>= 1.70)
- Stellar CLI

## The One-Click Sandbox
The easiest way to develop locally is by using the Sadgi Sandbox.

```bash
make dev
```

This command simultaneously spins up:
1. **Stellar Localnet**: A local Soroban environment.
2. **Mock Prover Node**: A local daemon that automatically fulfills `JobRequests` without requiring actual heavy ZK proofs (bypassing slow hardware requirements).
3. **Next.js Explorer**: A local web interface available at `http://localhost:3000` to monitor the Compute Marketplace state machine.

## Running Tests
To run the full suite of unit and integration tests across the monorepo:
```bash
make test
```
