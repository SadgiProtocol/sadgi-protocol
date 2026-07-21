# API Reference

Complete reference for all Sadgi contract interfaces and SDK methods.

---

## Registry Contract

### `register_program`

Register a new ZK program. Admin only.

| Parameter | Type | Description |
|-----------|------|-------------|
| `admin` | `Address` | Must match the Registry admin |
| `program_id` | `String` | Unique identifier (e.g., `"identity_v1"`) |
| `vk` | `Bytes` | Serialized Groth16 verification key |
| `name` | `String` | Human-readable name |

**Returns**: `BytesN<32>` — the VK hash.

### `get_vk`

```
fn get_vk(program_id: String) → Option<Bytes>
```

Returns the raw VK bytes for a program, or `None` if not registered.

### `get_vk_hash`

```
fn get_vk_hash(program_id: String) → Option<BytesN<32>>
```

Returns the SHA-256 hash of the VK (cheaper than fetching full VK).

### `list_programs`

```
fn list_programs() → Vec<String>
```

Returns all registered program IDs.

---

## Verifier Contract

### `verify`

```
fn verify(vk: Bytes, proof: Bytes, public_inputs: Vec<Bytes>) → bool
```

Verifies a Groth16 SNARK. Panics with `VerificationFailed` if invalid.

| Parameter | Type | Description |
|-----------|------|-------------|
| `vk` | `Bytes` | Verification key bytes |
| `proof` | `Bytes` | 256-byte Groth16 proof (A, B, C points) |
| `public_inputs` | `Vec<Bytes>` | 32-byte big-endian BN254 field elements |

---

## Marketplace Contract

### `post_job`

```
fn post_job(poster, program_id, class, bounty, input_commitment, deadline_ledger) → BytesN<32>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `poster` | `Address` | Job poster (must authorize) |
| `program_id` | `String` | Registered program ID |
| `class` | `JobClass` | `Standard \| Priority \| Bulk \| AI` |
| `bounty` | `i128` | Bounty in stroops (1 XLM = 10,000,000) |
| `input_commitment` | `BytesN<32>` | SHA-256 of private inputs |
| `deadline_ledger` | `u32` | Ledger sequence number of deadline |

**Returns**: `BytesN<32>` — the job ID.

**Errors**:
- `InsufficientBounty` — Bounty below class minimum.
- `ProgramNotFound` — `program_id` not in Registry.

### `claim_job`

```
fn claim_job(prover: Address, job_id: BytesN<32>)
```

Assigns a job to a prover. Job must be in `Posted` state.

### `submit_proof`

```
fn submit_proof(prover: Address, job_id: BytesN<32>, proof: Bytes, public_inputs: Vec<Bytes>)
```

Submits a proof for a claimed job. On success:
- Verifies the proof via the Verifier contract.
- Persists the receipt.
- Transfers bounty to `prover`.
- Emits `ProofVerified` event.

**Errors**:
- `NotClaimer` — Caller is not the prover who claimed this job.
- `JobExpired` — Deadline has passed.
- `VerificationFailed` — Groth16 proof is invalid.

### `cancel_job`

```
fn cancel_job(poster: Address, job_id: BytesN<32>)
```

Cancels a job and refunds the bounty. Only allowed if:
- Job is `Posted` (not yet claimed), or
- Job is `Claimed` but past the deadline.

### `get_job`

```
fn get_job(job_id: BytesN<32>) → Job
```

### `get_receipt`

```
fn get_receipt(job_id: BytesN<32>) → Receipt
```

Panics if no receipt exists for this job.

### `list_jobs`

```
fn list_jobs(status: Option<JobStatus>, limit: u32) → Vec<Job>
```

---

## SDK Methods

### `SadgiClient`

```typescript
new SadgiClient(config: SadgiClientConfig)
```

| Config Key | Type | Default |
|-----------|------|---------|
| `network` | `"testnet" \| "mainnet"` | Required |
| `rpcUrl` | `string` | Network default |
| `publicKey` | `string` | Required |
| `signer` | `WalletAdapter` | Required |
| `contractAddresses` | `Partial<ContractAddresses>` | Network defaults |

### `client.marketplace.postJob(params)` → `Promise<string>`
### `client.marketplace.getJob(jobId)` → `Promise<Job>`
### `client.marketplace.awaitReceipt(jobId, options)` → `Promise<Receipt>`
### `client.marketplace.cancelJob(jobId)` → `Promise<void>`
### `client.marketplace.listJobs(filters)` → `Promise<Job[]>`
### `client.marketplace.onJobEvent(jobId, handler)` → `() => void`
### `client.registry.listPrograms()` → `Promise<Program[]>`
### `client.registry.getVK(programId)` → `Promise<Uint8Array>`
### `client.verifier.checkReceipt(cid)` → `Promise<boolean>`

---

## Events

### `ProofVerified`

Emitted by Marketplace on successful proof submission.

```
topic:  ("pv", job_id)
data:   (program_id, cid, prover)
```

### `JobPosted`

```
topic:  ("jp", job_id)
data:   (program_id, class, bounty, poster, deadline_ledger)
```

### `JobClaimed`

```
topic:  ("jc", job_id)
data:   (prover)
```

### `JobCancelled`

```
topic:  ("jx", job_id)
data:   (poster, refund_amount)
```

---

## Error Codes

| Code | Name | Description |
|------|------|-------------|
| `1` | `ProgramNotFound` | Program ID not in Registry |
| `2` | `InsufficientBounty` | Bounty below class minimum |
| `3` | `JobNotFound` | Job ID does not exist |
| `4` | `JobNotOpen` | Job cannot transition from current state |
| `5` | `NotClaimer` | Caller did not claim this job |
| `6` | `JobExpired` | Deadline has passed |
| `7` | `VerificationFailed` | Groth16 proof check failed |
| `8` | `AlreadyInitialized` | Marketplace already initialized |
| `9` | `Unauthorized` | Caller is not the admin |
