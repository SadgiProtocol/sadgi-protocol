# Security Policy

## Supported Versions

The following versions of the Sadgi Protocol are currently being supported with security updates.

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

Security is a top priority for the Sadgi Protocol. If you discover a security vulnerability in the Soroban smart contracts, SP1 prover circuits, or the frontend dashboard, we kindly ask that you report it to us immediately rather than opening a public issue.

Please send an email to **security@sadgiprotocol.com** with the following details:
- A description of the vulnerability.
- Steps to reproduce the issue.
- Potential impact and any suggested mitigations.

We will acknowledge receipt of your vulnerability report within 48 hours and strive to send you regular updates about our progress. If the vulnerability is accepted, we will coordinate a security advisory and patch release.

## Automated Scans

We run `cargo-deny`, `cargo-audit`, and CodeQL on every pull request to ensure that dependencies are free from known vulnerabilities and that our source code adheres to secure coding standards.
