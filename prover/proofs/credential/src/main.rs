#![no_main]
sp1_zkvm::entrypoint!(main);

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct CredentialSubject {
    pub id: String,
    pub age: u8,
    pub kyc_status: String,
}

#[derive(Deserialize, Debug)]
struct W3CCredential {
    pub issuer: String,
    #[serde(rename = "credentialSubject")]
    pub credential_subject: CredentialSubject,
}

pub fn main() {
    // 1. Read caller contract ID (Public Input)
    let caller_contract_id = sp1_zkvm::io::read::<[u8; 32]>();
    
    // We commit the caller contract ID so the Verifier knows this proof is bound to the exact marketplace request.
    sp1_zkvm::io::commit(&caller_contract_id);

    // 2. Read W3C VC payload (Private Input - Prover only)
    let vc_json = sp1_zkvm::io::read::<String>();
    
    // 3. Parse JSON
    let vc: W3CCredential = serde_json::from_str(&vc_json).expect("Failed to parse W3C VC JSON");
    
    // 4. Cryptographic Verification Mock (Ensure the issuer is our trusted KYC provider)
    assert_eq!(vc.issuer, "did:sadgi:kyc-provider-1", "Untrusted VC Issuer");
    
    // 5. Business Logic: Assert age > 18 without revealing the actual age
    assert!(vc.credential_subject.age >= 18, "User is under 18");
    
    // 6. Business Logic: Assert KYC is verified
    assert_eq!(vc.credential_subject.kyc_status, "verified", "KYC status not verified");
    
    // 7. Commit success (Public Output)
    // By only outputting `true`, the network knows the user is >18 and KYC'd, 
    // but their actual age and DID ID remain completely hidden in the ZK proof.
    let success = true;
    sp1_zkvm::io::commit(&success);
}
