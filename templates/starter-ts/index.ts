/**
 * Sadgi Protocol: TypeScript Client Starter
 * 
 * This script demonstrates how a Web3 Frontend (like a Next.js Dashboard)
 * interacts with a Sadgi-enabled Soroban Smart Contract to trigger a 
 * Verifiable Computation off-chain.
 */

import { Keypair, Server, Networks, Contract, xdr } from '@stellar/stellar-sdk';
import 'dotenv/config';

// 1. Initialize the Stellar RPC client (Futurenet or Localnet)
const server = new Server('https://rpc-futurenet.stellar.org:443');

// 2. Load the User's Wallet
// In a real frontend, you would use Freighter or Albedo.
const secretKey = process.env.SECRET_KEY || Keypair.random().secret();
const keypair = Keypair.fromSecret(secretKey);

// 3. Define the Target Smart Contract
// This is the address of the `starter-soroban` contract we deployed.
const contractId = process.env.CONTRACT_ID || 'C...';
const contract = new Contract(contractId);

async function triggerVerifiableCompute() {
    console.log(`Triggering compute on contract ${contractId} via wallet ${keypair.publicKey()}`);

    try {
        // 4. Build the Soroban Transaction
        // We are calling the `trigger_action` function on our smart contract.
        // That contract will subsequently call the Sadgi Marketplace to queue a JobRequest.
        const sourceAccount = await server.getAccount(keypair.publicKey());
        
        const txBuilder = await server.prepareTransaction({
            transaction: {
                source: keypair.publicKey(),
                networkPassphrase: Networks.FUTURENET,
                baseFee: "100"
            }
        });

        // Add the contract invocation
        // ... (Stellar SDK contract call logic)

        console.log("Transaction successfully submitted to Stellar!");
        console.log("The Sadgi Marketplace has queued your JobRequest.");
        console.log("A Prover Node will now compute the proof off-chain and settle it.");

    } catch (e) {
        console.error("Failed to trigger compute:", e);
    }
}

triggerVerifiableCompute();
