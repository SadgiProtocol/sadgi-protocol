# Changelog

All notable changes to the Sadgi Protocol will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- **Compute Marketplace**: Initial implementation of the Soroban Escrow and Scheduler.
- **Reference Programs**: Scaffolded `Identity` (KYC) and `Credit` zkVM guest programs.
- **Developer SDK**: Added the Rust SDK for generating and submitting proof requests.
- **Protocol Standards**: Defined the `SadgiReceipt` standard serialization format.
- **Ecosystem DX**: Established `make dev` localnet sandbox with Next.js Explorer integration.

### Changed
- Shifted governance model to an XLM-native M-of-N Multisig (Technical Council).

### Removed
- Removed the speculative `$SADGI` token architecture in favor of pure XLM utility.
