# Verifiable Computation

Verifiable computation lets one party (the **prover**) convince another party (the **verifier**) that a computation was performed correctly—without the verifier having to re-execute it.

## The Problem

Blockchains are expensive execution environments. Running complex computations (ML inference, credential checks, threshold signatures) on-chain would consume enormous gas/fee resources and expose private inputs to the entire network.

Sadgi solves this by moving computation **off-chain** while keeping verification **on-chain**.

## Zero-Knowledge Proofs

A ZK proof has three properties:

| Property | Meaning |
|----------|---------|
| **Completeness** | If the statement is true, an honest prover can always convince the verifier. |
| **Soundness** | A dishonest prover cannot convince the verifier of a false statement (with overwhelming probability). |
| **Zero-Knowledge** | The verifier learns nothing about the private inputs beyond the truth of the statement. |

## SP1: From Program to STARK

Sadgi uses [SP1 by Succinct Labs](https://github.com/succinctlabs/sp1), a zkVM that turns **any Rust program** into a STARK proof:

```
Rust guest program
      │
      ▼
  SP1 zkVM  ──── executes program
      │
      ▼
  STARK proof  (large, cheap to generate)
      │
      ▼
  Groth16 SNARK  (tiny, cheap to verify on-chain)
```

The STARK-to-Groth16 wrapping step produces a proof that is:
- **~200 bytes** regardless of computation size.
- Verifiable in **O(1)** time with a fixed-cost pairing check.

## Verification Keys

Each SP1 guest program has a **verification key (VK)**—a short cryptographic fingerprint of the program's circuit. The VK is stored in Sadgi's **Registry contract** at deployment time.

When a prover submits a proof, the **Verifier contract** checks:

```
e(proof.A, proof.B) == e(vk.alpha, vk.beta) · e(publicInputsCommitment, vk.gamma) · e(proof.C, vk.delta)
```

This pairing equation is the BN254 Groth16 verification equation. It is computed natively inside a Soroban host function.

## Computation Flow in Sadgi

```
Job Poster                 Prover                    Stellar Network
    │                        │                              │
    │── postJob(programId) ──►│                              │
    │                        │── fetch program VK ─────────►│
    │                        │◄── VK ──────────────────────│
    │                        │                              │
    │                        │  run SP1 guest program       │
    │                        │  generate STARK              │
    │                        │  wrap to Groth16 SNARK       │
    │                        │                              │
    │                        │── submitProof(jobId, proof) ►│
    │                        │                              │── Verifier.verify()
    │                        │                              │── emit Receipt
    │                        │◄── bounty released ─────────│
    │◄── receipt CID ────────│                              │
```

## Why Groth16 and Not STARK?

STARKs are excellent for generation but produce large proofs (hundreds of KB). Soroban contracts have limited entry-point data size. Wrapping to Groth16 gives us:

- **Fixed small proof size**: always ~200 bytes (3 BN254 curve points).
- **Deterministic verification cost**: one pairing check regardless of circuit depth.
- **Battle-tested security**: Groth16 with BN254 is used by Zcash, Tornado Cash, and many production systems.

The tradeoff is a one-time trusted setup per circuit, which Sadgi inherits from the SP1 Groth16 wrapper's universal SRS (no per-program ceremony needed).

## Further Reading

- [SP1 Book](https://docs.succinct.xyz/sp1/getting-started/intro)
- [Groth16 Paper](https://eprint.iacr.org/2016/260)
- [Receipts](./receipts.md) — how verified proofs become on-chain claims
