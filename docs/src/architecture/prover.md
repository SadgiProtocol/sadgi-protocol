# The Prover Node Architecture

The Prover Node is the off-chain daemon that performs the heavy lifting for the Sadgi Protocol. It is responsible for monitoring the Stellar ledger, executing zkVM binaries, and submitting the resulting proofs.

## 1. Ledger Indexer
The Prover continuously polls the Stellar RPC (via Horizon or a Soroban RPC) listening for `JobAssigned` events emitted by the Compute Marketplace contract. 
If an event's `prover_id` matches the Prover's own Stellar public key, it accepts the job.

## 2. IPFS / Artifact Resolution
The Prover reads the `target_image_id` from the `JobRequest`. It queries a decentralized storage network (like IPFS) or a trusted registry to download the compiled RISC-V ELF binary that corresponds to that Image ID.

## 3. zkVM Execution (SP1)
The Prover instantiates the SP1 zkVM locally. It feeds the `caller_contract_id` and any public inputs into the VM, and executes the binary.
This is the most computationally intensive phase. It can take anywhere from a few seconds to several minutes depending on the complexity of the developer's program.

## 4. Submission & Settlement
Once the execution completes, the zkVM outputs a cryptographic Seal and a public Journal.
The Prover serializes these into a `ProofReceipt` according to SIP-1, constructs a Soroban transaction, signs it with its Stellar private key, and submits it to the Compute Marketplace to claim the XLM bounty.
