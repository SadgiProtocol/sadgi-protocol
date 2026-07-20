#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    // 1. Read caller contract ID
    let caller_contract_id = sp1_zkvm::io::read::<[u8; 32]>();
    sp1_zkvm::io::commit(&caller_contract_id);

    // 2. Read private financial transactions and threshold
    let transactions = sp1_zkvm::io::read::<Vec<i32>>();
    let target_threshold = sp1_zkvm::io::read::<i32>();

    // 3. Compute score securely inside the enclave (without revealing data)
    let sum: i32 = transactions.iter().sum();
    let score = sum / (transactions.len() as i32).max(1);

    // 4. Assert privacy-preserving condition
    assert!(score >= target_threshold, "Score below threshold");

    // 5. Commit ONLY that the threshold was met, hiding the actual score and data
    sp1_zkvm::io::commit(&target_threshold);
}
