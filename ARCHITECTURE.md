# Sadgi Protocol Architecture & Delivery Guidelines

This document outlines the engineering standards, deployment topologies, and versioning strategies for the Sadgi Protocol.

## 1. Semantic Versioning Strategy (SemVer)
The protocol enforces strict SemVer (`MAJOR.MINOR.PATCH`) across its diverse components to prevent ecosystem breakage:
- **Smart Contracts (Soroban WASM)**: 
  - `MAJOR`: Breaking changes to the `SadgiMarketplaceClient` interface (e.g., modifying `create_job` arguments).
  - `MINOR`: Internal state machine additions or gas optimizations.
- **Reference Programs (SP1)**:
  - `MAJOR`: Changes to the `CanonicalJournal` payload structure or `PrivateClaim` schemas.
- **Developer SDK (`sadgi-sdk`)**:
  - Versioned independently on `crates.io`. `MAJOR` updates align with Contract API breaking changes.

## 2. Deployment Topology & Environments

The pipeline promotes artifacts sequentially through the following environments:

1. **Localnet (`make dev`)**: Uses `docker-compose` to spin up a single-node Stellar sandbox and Mock Prover. Used by contributors for rapid iteration.
2. **Docker Build CI**: Ensures reproducible compilation of all crates via `Dockerfile`.
3. **Integration (CI)**: A temporary environment spun up during GitHub Actions to run E2E scenarios.
4. **Futurenet (Testnet)**: Nightly deployments of the Soroban contracts. Highly unstable.
5. **Testnet (Stable)**: The canonical public sandbox. The Developer SDK and CLI point here by default.
6. **Mainnet**: Production deployment.

## 3. Deployment Targets
When a Release is cut, the following artifacts are generated and deployed:
- `sadgi_marketplace.wasm`: Deployed to Stellar network.
- `sadgi-prover-node`: Published to GitHub Container Registry (`ghcr.io`).
- `sadgi-sdk`: Published to `crates.io`.
- `sadgi-cli`: Compiled for MacOS, Linux, and Windows and attached to GitHub Releases.
- `dashboard`: Deployed to Vercel or a decentralized hosting provider.

## 4. Observability & Security
- **Security**: The CI pipeline enforces `cargo audit` to catch upstream vulnerabilities in SP1 or Soroban SDKs before they reach Mainnet.
- **Observability**: The Prover Node implements structured logging and exposes healthcheck endpoints for container orchestrators (like Kubernetes).

## 5. Protocol Administration & Governance
Sadgi is a piece of core infrastructure. Like Linux or PostgreSQL, it does not rely on a speculative governance token. The protocol is administered via the `SadgiAdministration` smart contract, which implements an M-of-N Multisig controlled by a Technical Council.

### 5.1 Progressive Decentralization Roadmap
Control of the Protocol Administration will evolve organically through predefined phases:
1. **Core Team (Current)**: A 3-of-5 multisig controlled by the founding developers to enable rapid iteration.
2. **Foundation Stewardship**: Control is transferred to a non-profit Foundation, integrating external security researchers and early adopters.
3. **Technical Council**: Expansion to a 9-of-15 multisig composed of verified node operators, application developers, and ecosystem delegates.
4. **Community Governance**: Fully decentralized reputation-based voting (weighting based on XLM stake or prover history, completely tokenless).

### 5.2 Sadgi Improvement Proposals (SIPs)
To upgrade the protocol or adjust Marketplace Parameters (e.g. minimum prover XLM stake, escrow rules, pausing the protocol), a formal SIP must be authored.
1. **Draft**: RFC published to the developer portal.
2. **Audit**: Code changes are audited by independent security firms.
3. **Proposal**: A Technical Council member triggers `propose()` on-chain, embedding the target contract and action.
4. **Timelock**: Once the threshold is met, the change enters a mandatory waiting period before it can be `execute()`'d.

### 5.3 Emergency Controls
In the event of a catastrophic vulnerability (e.g. a zero-day in SP1's verifier), the Technical Council can bypass the Timelock to trigger a `PauseMarketplace` cross-contract call. This prevents new jobs from being queued while a patch is developed.
