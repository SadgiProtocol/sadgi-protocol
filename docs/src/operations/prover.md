# Running a Prover Node

Prover nodes are the backbone of the Sadgi Protocol. They watch the Marketplace for open jobs, generate Groth16 SNARKs using SP1, and earn bounties for successful submissions.

## Hardware Requirements

| Tier | CPU | RAM | Storage | Earnings Potential |
|------|-----|-----|---------|-------------------|
| Minimal | 8-core x86 | 16 GB | 50 GB SSD | Bulk + Standard jobs |
| Recommended | 16-core x86 | 32 GB | 100 GB NVMe | All classes |
| High-Performance | 32-core + NVIDIA A10 | 64 GB | 200 GB NVMe | AI jobs |

> **GPU Note**: SP1's CUDA backend accelerates proof generation ~10x for large circuits. AI-class jobs effectively require a GPU to complete within their deadline.

## Software Dependencies

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install stable

# SP1 prover toolchain
curl -L https://sp1.succinct.xyz | bash
sp1up

# Docker (for containerized proving)
# Follow https://docs.docker.com/engine/install/

# Stellar CLI (for tx submission)
cargo install --locked stellar-cli
```

## Configuration

Create a prover configuration file at `~/.sadgi/prover.toml`:

```toml
[network]
name = "testnet"
rpc_url = "https://soroban-testnet.stellar.org"
horizon_url = "https://horizon-testnet.stellar.org"

[identity]
secret_key = "SCECRET..."   # Keep secure! Use env var in production.

[contracts]
marketplace = "CMARKETPLACE..."
registry = "CREGISTRY..."

[prover]
programs = ["hash_proof_v1", "identity_v1", "credit_v1"]  # Programs to accept
max_concurrent_jobs = 2
job_classes = ["Standard", "Priority"]   # Skip Bulk and AI if under-resourced
poll_interval_ms = 3000

[proving]
backend = "cpu"   # "cpu" | "cuda" | "network"
sp1_prover_url = ""  # Only for "network" backend (Succinct's remote prover)
mock_proofs = false  # Set true for development/testing only

[storage]
ipfs_gateway = "https://ipfs.io"
ipfs_pin_service = ""   # Optional: Pinata API key for pinning inputs
```

Use environment variables for secrets:

```bash
export SADGI_SECRET_KEY="SCECRET..."
export SADGI_IPFS_KEY="your-pinata-key"
```

## Starting the Prover

```bash
# From binary
sadgi-prover --config ~/.sadgi/prover.toml

# Via Docker
docker run -d \
  --name sadgi-prover \
  -e SADGI_SECRET_KEY=$SADGI_SECRET_KEY \
  -v ~/.sadgi:/config \
  ghcr.io/sadgi-protocol/prover:latest \
  --config /config/prover.toml
```

## Prover Lifecycle

For each discovered job, the prover node executes this workflow:

```
1. Poll / subscribe to JobPosted events
           │
           ▼
2. Filter: programId ∈ configured programs
           program class ∈ configured classes
           │
           ▼
3. Claim job via marketplace.claim_job()
           │
           ▼
4. Fetch encrypted inputs from inputCID (IPFS)
   Decrypt with prover's private key
           │
           ▼
5. Execute SP1 guest program (generates STARK)
           │
           ▼
6. Wrap STARK → Groth16 SNARK
           │
           ▼
7. Submit proof via marketplace.submit_proof()
           │
           ▼
8. Receive bounty payment
```

## Monitoring

The prover exposes a Prometheus-compatible metrics endpoint on port `9090`:

```
sadgi_jobs_claimed_total        # Counter
sadgi_jobs_verified_total       # Counter
sadgi_jobs_failed_total         # Counter
sadgi_proof_generation_seconds  # Histogram
sadgi_bounty_earned_xlm         # Gauge
```

Access the metrics:
```bash
curl http://localhost:9090/metrics
```

Or point Grafana at `http://localhost:9090` for a dashboard.

## Profitability Estimates

Proof generation costs (CPU, approximate):

| Program | Generation Time | Recommended Class |
|---------|----------------|------------------|
| `hash_proof_v1` | ~30s | Standard (10 XLM) |
| `identity_v1` | ~90s | Standard (12 XLM) |
| `credit_v1` | ~120s | Priority (50 XLM) |
| `ai_inference_v1` | ~15 min | AI (200 XLM) |

## Security Hardening

1. **Never expose the prover's secret key**. Use a dedicated keypair, not your personal account.
2. **Run in a VM or container**. Guest program inputs may contain user data—isolate the process.
3. **Validate `inputCID` sizes** before fetching. Reject inputs larger than expected to prevent DoS.
4. **Set `max_concurrent_jobs`** conservatively. Racing to claim too many jobs and failing to prove them will damage your prover reputation score.

## Prover Reputation (Upcoming)

Sadgi tracks prover performance on-chain. Future protocol upgrades will weight job assignment toward provers with higher reputation, calculated from:
- Proof success rate
- Latency vs. deadline
- Slashing history (failed proofs after claiming)

## Troubleshooting

**`OutOfMemory` during proof generation**: Reduce `max_concurrent_jobs` to 1, or switch to the network backend (`sp1_prover_url` pointing to Succinct's remote service).

**`JobExpired` on submit**: Increase hardware or reduce the job classes you accept. Priority jobs have the tightest deadlines.

**`InsufficientFunds` on claim**: Ensure your prover account has ≥ 2 XLM for transaction fees. Top up via Friendbot on testnet.

## See Also

- [Local Environment](../getting_started/local_environment.md)
- [Contracts Architecture](../architecture/contracts.md)
- [Verifiable Computation](../concepts/verifiable_computation.md)
