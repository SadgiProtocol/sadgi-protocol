// --------------------------------------------------------------------------------
// Sadgi Starter Template: Off-Chain zkVM Guest Program
// 
// This template demonstrates how to write a zero-knowledge program that is 
// compliant with the Sadgi Program Standard (SIP-3).
// --------------------------------------------------------------------------------

use risc0_zkvm::guest::env;

fn main() {
    // 1. READ THE CALLER CONTRACT ID
    // The Host (Prover) will always inject the 32-byte Soroban Contract ID 
    // that requested this computation as the first input.
    let caller_contract_id: [u8; 32] = env::read();

    // 2. PREVENT REPLAY ATTACKS (SIP-3 COMPLIANCE)
    // We must immediately commit this ID to the public journal. The Soroban 
    // Verifier will mathematically assert that the contract consuming the 
    // receipt exactly matches this ID.
    env::commit_slice(&caller_contract_id);

    // 3. READ USER INPUTS
    // Now you can read whatever arbitrary inputs the developer provided.
    // E.g., a hash, a digital signature, or an entire JSON document.
    let _user_input: u64 = env::read();

    // 4. PERFORM HEAVY COMPUTATION
    // This code runs strictly off-chain inside the zkVM. You can perform loops,
    // cryptographic hashing, or machine learning inference without worrying 
    // about Soroban gas limits.
    let result = true;

    // 5. COMMIT RESULTS
    // Finally, commit the result of your computation to the journal.
    // This will be publicly readable by the Soroban smart contract.
    env::commit(&result);
}
