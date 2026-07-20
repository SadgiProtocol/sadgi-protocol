mod backend;
mod executor;
mod runner;
mod scheduler;

use tokio;

#[tokio::main]
async fn main() {
    println!("Starting Sadgi Prover Node...");

    // 1. Initialize the Scheduler to poll the Soroban Marketplace for Assigned Jobs.
    let mut sched = scheduler::Scheduler::new();

    // 2. Main Event Loop
    loop {
        if let Some(job) = sched.poll_next_job().await {
            println!("Found Job: {:?}", job);

            // 3. Delegate to the Executor
            let receipt = executor::execute_job(job).await;

            // 4. Submit Receipt back to Marketplace
            sched.submit_receipt(receipt).await;
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
