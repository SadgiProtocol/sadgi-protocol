use sp1_sdk::ProverClient;
use std::collections::HashMap;
use std::fs;

fn main() {
    let client = ProverClient::new();
    let mut registry_map = HashMap::new();

    let proofs_dir = "../prover/proofs";
    let output_file = "../dashboard/public/registry_init.json";

    println!("Generating Verification Keys for Sadgi Proof Claims...");

    // We hardcode the claims we expect to mock
    let claims = vec!["hash", "signature", "threshold", "credential", "reputation"];

    for claim in claims {
        // In a real environment, we'd compile the ELF using sp1_sdk::build::build_program
        // Here we simulate the process since SP1 building fails on windows MSVC locally.
        println!("Processing Claim: {}", claim);

        let dummy_program_id = format!(
            "00000000000000000000000000000000000000000000000000000000000000{}",
            claim.len()
        );
        let dummy_vk = format!("vk_mock_{}", claim);

        registry_map.insert(
            dummy_program_id,
            serde_json::json!({
                "name": claim,
                "version": 1,
                "verification_key": hex::encode(dummy_vk)
            }),
        );
    }

    let json = serde_json::to_string_pretty(&registry_map).unwrap();
    fs::create_dir_all("../dashboard/public").unwrap();
    fs::write(output_file, json).unwrap();
    println!("Saved Verification Keys to {}", output_file);
}
