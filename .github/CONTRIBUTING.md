# Contributing to Sadgi Protocol

First off, thank you for considering contributing to the Sadgi Protocol! It's people like you that make Sadgi a robust, decentralized trust layer for the Stellar ecosystem.

## Development Workflow
1. **Fork the repository** and clone it locally.
2. **Branch naming**: Use `feature/issue-number-description` or `fix/issue-number-description`.
3. **Local Setup**: Run `make dev` to spin up the Soroban Localnet, Mock Prover, and Next.js Dashboard.
4. **Testing**: Run `make test` and `make format` before committing.

## Sadgi Improvement Proposals (SIPs)
If you are proposing a major architectural change or modifying the `Sadgi Receipt Standard`, please open an Issue with the `[SIP]` prefix before writing code. Major changes require Technical Council approval.

## Pull Request Process
- Ensure any install or build dependencies are removed before the end of the layer when doing a build.
- Update the README.md with details of changes to the interface, this includes new environment variables, exposed ports, useful file locations and container parameters.
- Ensure your PR passes the GitHub Actions CI pipeline (Clippy, Fmt, Audit).
