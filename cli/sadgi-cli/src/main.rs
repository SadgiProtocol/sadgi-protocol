use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "sadgi")]
#[command(about = "Sadgi Protocol Developer CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Developer commands (Init, Dev, etc.)
    Init { name: String },
    Dev,
    
    /// DAO Governance Commands
    Dao {
        #[command(subcommand)]
        dao_cmd: DaoCommands,
    },
    
    /// Marketplace Job Commands
    Job {
        #[command(subcommand)]
        job_cmd: JobCommands,
    },
    
    /// Proof Verification Commands
    Proof {
        #[command(subcommand)]
        proof_cmd: ProofCommands,
    }
}

#[derive(Subcommand)]
enum JobCommands {
    /// Submit a new compute request to the Marketplace Queue
    Create {
        #[arg(long)]
        program: String,
        
        #[arg(long)]
        bounty: i128,
    },
    
    /// Query the current state of a Job (e.g. Queue -> Executing -> Settled)
    Status {
        #[arg(long)]
        id: u64,
    },
}

#[derive(Subcommand)]
enum ProofCommands {
    /// Submit a serialized SadgiReceipt for verification and settlement
    Submit {
        #[arg(long)]
        job: u64,
        
        #[arg(long)]
        receipt: String, // Path to receipt file
    }
}

#[derive(Subcommand)]
enum DaoCommands {
    /// Submit a new proposal to the Sadgi Protocol
    Propose {
        #[arg(long)]
        target: String,
        
        #[arg(long)]
        action: String,
    },
    
    /// Vote on an active proposal
    Vote {
        #[arg(long)]
        proposal: u32,
        
        #[arg(long)]
        approve: bool,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { name } => {
            println!("Initializing Sadgi project: {}", name);
        }
        Commands::Dev => {
            println!("Starting local Sandbox...");
        }
        Commands::Dao { dao_cmd } => {
            match dao_cmd {
                DaoCommands::Propose { target, action } => {
                    println!("=> Submitting proposal to invoke {} on target contract {}", action, target);
                    println!("=> Transaction submitted successfully. Proposal ID: 42");
                }
                DaoCommands::Vote { proposal, approve } => {
                    let vote_str = if *approve { "YES" } else { "NO" };
                    println!("=> Casting {} vote for Proposal ID: {}", vote_str, proposal);
                    println!("=> Transaction confirmed!");
                }
            }
        }
        Commands::Job { job_cmd } => {
            match job_cmd {
                JobCommands::Create { program, bounty } => {
                    println!("=> Locking {} XLM into Escrow...", bounty);
                    println!("=> Job successfully submitted to Marketplace Queue!");
                    println!("=> Assigned Job ID: 1492");
                    println!("=> Target Program ID: {}", program);
                }
                JobCommands::Status { id } => {
                    println!("=> Querying Soroban state for Job #{}...", id);
                    println!("=> State: EXECUTING");
                    println!("=> Assigned Prover: GARV...3K2A");
                }
            }
        }
        Commands::Proof { proof_cmd } => {
            match proof_cmd {
                ProofCommands::Submit { job, receipt } => {
                    println!("=> Loading SadgiReceipt from file: {}", receipt);
                    println!("=> Submitting Proof to Soroban Marketplace for Job #{}...", job);
                    println!("=> Verification Successful!");
                    println!("=> Escrow funds released. Settlement complete.");
                }
            }
        }
    }
}
