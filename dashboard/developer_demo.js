const { Keypair, rpc, TransactionBuilder, Networks, BASE_FEE, xdr, Contract, nativeToScVal } = require("@stellar/stellar-sdk");
const fs = require('fs');

async function main() {
    console.log("🚀 Sadgi Protocol Developer Integration Demo\n");

    // 1. Load Deployments
    let MARKETPLACE_ID = "MOCK_MODE";
    const deploymentsPath = "./public/deployments.json";
    if (fs.existsSync(deploymentsPath)) {
        const deployments = JSON.parse(fs.readFileSync(deploymentsPath, 'utf8'));
        MARKETPLACE_ID = deployments.contracts.marketplace;
        console.log(`✅ Loaded Marketplace Contract ID: ${MARKETPLACE_ID}`);
    } else {
        console.warn(`⚠️  deployments.json not found! Running in Mock Mode.`);
    }

    // 2. Generate Developer Wallet
    const developerKeypair = Keypair.random();
    console.log(`\n👨‍💻 Generating new Developer Wallet...`);
    console.log(`Public Key: ${developerKeypair.publicKey()}`);

    // 3. Fund Wallet via Friendbot (Testnet)
    console.log(`\n💸 Requesting funds from Friendbot...`);
    const friendbotUrl = `https://friendbot.stellar.org?addr=${encodeURIComponent(developerKeypair.publicKey())}`;
    const friendbotResponse = await fetch(friendbotUrl);
    if (!friendbotResponse.ok) {
        throw new Error("Failed to fund account. Make sure you are on Testnet.");
    }
    console.log(`✅ Account successfully funded with 10,000 XLM!`);

    // 4. Initialize Soroban RPC
    const server = new rpc.Server("https://soroban-testnet.stellar.org");
    const account = await server.getAccount(developerKeypair.publicKey());

    // 5. Construct Job Request
    console.log(`\n📝 Constructing ZK Job Request...`);
    
    if (MARKETPLACE_ID === "MOCK_MODE") {
        console.log(`\n🔮 Simulating transaction execution to calculate gas limits...`);
        console.log(`\n[MOCK MODE] Simulation successful! Gas fee: 100,000 stroops.`);
        console.log(`[MOCK MODE] 📤 Submitting transaction to the blockchain...`);
        console.log(`[MOCK MODE] Transaction sent! Waiting for confirmation (Hash: 0x8a9b3c4d...)`);
        
        await new Promise(resolve => setTimeout(resolve, 3000));
        console.log(`\n🎉 Success! Transaction was included in the ledger.`);
        console.log(`\n📌 Result: The Marketplace contract successfully registered your ZK Job!`);
        console.log(`Job ID returned: 42`);
        return;
    }

    const contract = new Contract(MARKETPLACE_ID);
    
    // Arguments:
    // 1. developer: Address
    const devAddressVal = nativeToScVal(developerKeypair.publicKey(), { type: 'address' });
    
    // 2. class: JobClass::Standard
    const classVal = xdr.ScVal.scvVec([xdr.ScVal.scvSymbol("Standard")]);
    
    // 3. bounty: i128 (10 XLM minimum)
    const bountyVal = nativeToScVal(10n, { type: 'i128' });
    
    // 4. redundancy: u32
    const redundancyVal = nativeToScVal(1, { type: 'u32' });

    // 6. Build Transaction
    const tx = new TransactionBuilder(account, {
        fee: BASE_FEE,
        networkPassphrase: Networks.TESTNET,
    })
    .addOperation(contract.call("create_job", devAddressVal, classVal, bountyVal, redundancyVal))
    .setTimeout(30)
    .build();

    // 7. Simulate Transaction (to calculate auth and gas)
    console.log(`\n🔮 Simulating transaction execution to calculate gas limits...`);
    
    if (MARKETPLACE_ID.includes("MOCK")) {
        console.log(`\n[MOCK MODE] Simulation successful! Gas fee: 100,000 stroops.`);
        console.log(`[MOCK MODE] 📤 Submitting transaction to the blockchain...`);
        console.log(`[MOCK MODE] Transaction sent! Waiting for confirmation (Hash: 0x8a9b3c4d...)`);
        
        await new Promise(resolve => setTimeout(resolve, 3000));
        console.log(`\n🎉 Success! Transaction was included in the ledger.`);
        console.log(`\n📌 Result: The Marketplace contract successfully registered your ZK Job!`);
        console.log(`Job ID returned: 42`);
        return;
    }

    const simResponse = await server.simulateTransaction(tx);
    
    if (rpc.Api.isSimulationError(simResponse)) {
        console.error("Simulation failed:", simResponse.error);
        return;
    }

    // Assemble the transaction using the simulation result
    const preparedTx = rpc.assembleTransaction(tx, simResponse);
    
    // 8. Sign and Submit
    preparedTx.sign(developerKeypair);
    
    console.log(`\n📤 Submitting transaction to the blockchain...`);
    const sendResponse = await server.sendTransaction(preparedTx);
    
    if (sendResponse.status === "PENDING") {
        console.log(`Transaction sent! Waiting for confirmation (Hash: ${sendResponse.hash})`);
        
        let txStatus = await server.getTransaction(sendResponse.hash);
        let attempts = 0;
        
        // Wait until it's not pending (or timeout after 10 seconds)
        while (txStatus.status === "NOT_FOUND" && attempts < 10) {
            await new Promise(resolve => setTimeout(resolve, 2000));
            txStatus = await server.getTransaction(sendResponse.hash);
            attempts++;
        }

        if (txStatus.status === "SUCCESS") {
            console.log(`\n🎉 Success! Transaction was included in the ledger.`);
            console.log(`Check it out on Stellar Expert:`);
            console.log(`https://stellar.expert/explorer/testnet/tx/${sendResponse.hash}`);
            
            // Extract the returned Job ID
            if (txStatus.resultMetaXdr) {
                const result = txStatus.returnValue;
                if (result) {
                    console.log(`\n📌 Result: The Marketplace contract successfully registered your ZK Job!`);
                    console.log(`Job ID returned: ${result.u64().toString()}`);
                }
            }
        } else {
            console.error(`\n❌ Transaction failed:`, txStatus.resultXdr ? txStatus.resultXdr : txStatus);
        }
    } else {
        console.error(`\n❌ Failed to send transaction:`, sendResponse);
    }
}

main().catch(console.error);
