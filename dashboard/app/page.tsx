"use client";

import { Terminal, Database, Activity, GitCommit, FileCode, Clock, Wallet } from "lucide-react";
import { useState, useEffect } from "react";
import { isAllowed, setAllowed, getUserInfo, signTransaction } from "@stellar/freighter-api";
import { rpc, TransactionBuilder, Networks, SorobanRpc } from "@stellar/stellar-sdk";

// Stellar Testnet configuration
const RPC_URL = "https://soroban-testnet.stellar.org";
const NETWORK_PASSPHRASE = Networks.TESTNET;
const MARKETPLACE_ID = "CC_MOCK_MARKETPLACE_ID_REPLACE_ME";

type JobState = "Request" | "Queue" | "Assigned" | "Executing" | "Submitted" | "Verified" | "Settled" | "Failed";

interface Job {
  id: string;
  program: string;
  bounty: string;
  state: JobState;
  prover?: string;
  receiptHash?: string;
  time: string;
}

interface ProgramRecord {
  name: string;
  version: number;
  verification_key: string;
}

export default function Dashboard() {
  const [mounted, setMounted] = useState(false);
  const [registry, setRegistry] = useState<Record<string, ProgramRecord>>({});
  const [jobs, setJobs] = useState<Job[]>([]);
  const [tps, setTps] = useState(42.1);
  const [block, setBlock] = useState(0);
  
  // Web3 State
  const [walletAddress, setWalletAddress] = useState<string | null>(null);
  const [isConnecting, setIsConnecting] = useState(false);
  const [server, setServer] = useState<rpc.Server | null>(null);

  const [deployments, setDeployments] = useState<any>(null);

  useEffect(() => {
    setMounted(true);
    setServer(new rpc.Server(RPC_URL));
    
    // Fetch Deployments
    fetch('/deployments.json')
      .then(res => res.json())
      .then(data => setDeployments(data))
      .catch(e => console.warn("deployments.json not found, using mock IDs"));

    // Fetch Registry
    fetch('/registry_init.json')
      .then(res => res.json())
      .then(data => setRegistry(data))
      .catch(() => {
        setRegistry({
          "0000000000000000000000000000000000000000000000000000000000000004": { name: "hash", version: 1, verification_key: "766b5f6d6f636b5f68617368" },
          "0000000000000000000000000000000000000000000000000000000000000009": { name: "signature", version: 1, verification_key: "766b5f6d6f636b5f7369676e6174757265" }
        });
      });
      
    checkWalletConnection();
  }, []);

  // Web3: Poll for latest ledger block
  useEffect(() => {
    if (!server) return;
    const interval = setInterval(async () => {
      try {
        const latest = await server.getLatestLedger();
        setBlock(latest.sequence);
        setTps(t => +(t + (Math.random() * 2 - 1)).toFixed(1));
      } catch (e) {
        // Fallback for simulation if RPC is offline
        setBlock(b => b + 1);
        setTps(t => +(t + (Math.random() * 2 - 1)).toFixed(1));
      }
    }, 2000);
    return () => clearInterval(interval);
  }, [server]);

  const checkWalletConnection = async () => {
    try {
      if (await isAllowed()) {
        const user = await getUserInfo();
        if (user.publicKey) {
          setWalletAddress(user.publicKey);
        }
      }
    } catch (e) {
      console.warn("Freighter not installed or locked");
    }
  };

  const connectWallet = async () => {
    setIsConnecting(true);
    try {
      await setAllowed();
      await checkWalletConnection();
    } catch (e) {
      console.error(e);
    } finally {
      setIsConnecting(false);
    }
  };

  // Web3: Create a Job transaction
  const handleDeployJob = async (programId: string) => {
    if (!walletAddress) {
      alert("Please connect your wallet first.");
      return;
    }

    const marketplaceId = deployments?.contracts?.marketplace || MARKETPLACE_ID;

    try {
      // 1. Build a mock generic transaction payload (In real life, we fetch account sequence first)
      console.log(`Building Soroban transaction to create job for ${programId} on contract ${marketplaceId}...`);
      
      // We simulate the transaction lifecycle
      const newJobId = Math.floor(Math.random() * 10000).toString();
      const progName = registry[programId]?.name || "Unknown";
      
      setJobs(prev => [{
        id: newJobId,
        program: progName,
        bounty: "50 XLM",
        state: "Queue",
        time: "Just now"
      }, ...prev]);
      
      alert(`Job Request Sent to ${marketplaceId.substring(0,8)}... (Simulated)`);
      
    } catch (e) {
      console.error(e);
      alert("Transaction failed");
    }
  };

  if (!mounted) return null;

  const renderStatus = (state: JobState) => {
    switch (state) {
      case "Settled":
      case "Verified":
        return <div className="status-indicator"><div className="status-dot green"></div>{state}</div>;
      case "Executing":
      case "Assigned":
      case "Submitted":
        return <div className="status-indicator"><div className="status-dot blue"></div>{state}</div>;
      default:
        return <div className="status-indicator"><div className="status-dot gray"></div>{state}</div>;
    }
  };

  return (
    <div className="layout-container">
      {/* Header */}
      <header className="header-nav">
        <div style={{ display: "flex", alignItems: "center", gap: "12px" }}>
          <Terminal size={20} color="var(--text-primary)" />
          <h1 style={{ fontSize: "16px", fontWeight: "600", letterSpacing: "1px" }}>SADGI // PLATFORM</h1>
        </div>
        <div style={{ display: "flex", alignItems: "center", gap: "20px" }}>
          <div className="status-indicator">
            <div className="status-dot green" style={{ boxShadow: "0 0 8px var(--accent-green)" }} />
            <span style={{ color: "var(--text-secondary)" }}>Localnet Sandbox</span>
          </div>
          
          {walletAddress ? (
            <div className="btn-secondary" style={{ display: "flex", alignItems: "center", gap: "8px", cursor: "default" }}>
              <Wallet size={14} />
              <span className="mono">{walletAddress.slice(0, 4)}...{walletAddress.slice(-4)}</span>
            </div>
          ) : (
            <button className="btn-secondary" onClick={connectWallet} disabled={isConnecting}>
              {isConnecting ? "Connecting..." : "Connect Freighter"}
            </button>
          )}
        </div>
      </header>

      {/* Main Grid */}
      <main className="grid-dashboard">
        
        {/* Left Sidebar: Program Registry */}
        <div style={{ display: "flex", flexDirection: "column", gap: "16px" }}>
          <h2 style={{ fontSize: "14px", fontWeight: 600, color: "var(--text-muted)", textTransform: "uppercase", letterSpacing: "0.5px" }}>
            Program Registry (SP1)
          </h2>
          
          {Object.entries(registry).map(([id, program]) => (
            <div key={id} className="panel-interactive" style={{ display: "flex", flexDirection: "column", gap: "8px" }}>
              <div style={{ display: "flex", alignItems: "center", gap: "8px", marginBottom: "4px" }}>
                <FileCode size={16} color="var(--text-primary)" />
                <h3 style={{ fontSize: "14px", fontWeight: 500, textTransform: "capitalize" }}>{program.name} (v{program.version})</h3>
              </div>
              <p className="mono" style={{ color: "var(--text-secondary)" }}>VK: 0x{program.verification_key.substring(0, 12)}...</p>
              <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginTop: "12px" }}>
                <span style={{ fontSize: "12px", color: "var(--text-muted)" }}>Platform Primitive</span>
                <button className="btn-secondary" style={{ padding: "4px 8px", fontSize: "12px" }} onClick={() => handleDeployJob(id)}>Deploy Job</button>
              </div>
            </div>
          ))}

          <button className="btn-primary" style={{ marginTop: "8px", width: "100%" }}>
            Register Custom Program
          </button>
        </div>

        {/* Right Pane: Marketplace Explorer (Terminal Style) */}
        <div className="panel" style={{ display: "flex", flexDirection: "column", overflow: "hidden" }}>
          <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", marginBottom: "20px" }}>
            <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
              <Activity size={16} color="var(--text-primary)" />
              <h2 style={{ fontSize: "14px", fontWeight: 600 }}>Marketplace State Explorer</h2>
            </div>
            <span style={{ fontSize: "12px", color: "var(--text-muted)" }}>Live Feed</span>
          </div>

          <div style={{ flex: 1, overflowY: "auto" }}>
            {/* Table Header */}
            <div className="log-row log-header">
              <div>Job ID</div>
              <div>State</div>
              <div>Bounty</div>
              <div>Program</div>
              <div style={{ textAlign: "right" }}>Prover</div>
            </div>

            {/* Table Body */}
            <div style={{ display: "flex", flexDirection: "column" }}>
              {jobs.length === 0 && <div style={{ padding: "20px", color: "var(--text-muted)", fontSize: "13px" }}>No active jobs. Connect your wallet and deploy one to start the process.</div>}
              {jobs.map((job) => (
                <div key={job.id} className="log-row" style={{ fontSize: "13px" }}>
                  <div className="mono" style={{ color: "var(--text-primary)" }}>#{job.id}</div>
                  <div>{renderStatus(job.state)}</div>
                  <div className="mono" style={{ color: "var(--text-secondary)" }}>{job.bounty}</div>
                  <div style={{ fontWeight: 500, textTransform: "capitalize" }}>{job.program}</div>
                  <div className="mono" style={{ textAlign: "right", color: "var(--text-muted)" }}>
                    {job.prover || "Unassigned"}
                  </div>
                </div>
              ))}
            </div>
          </div>
          
          {/* Terminal Footer */}
          <div style={{ marginTop: "24px", paddingTop: "16px", borderTop: "1px solid var(--border-subtle)", display: "flex", alignItems: "center", gap: "12px", color: "var(--text-muted)", fontSize: "12px" }}>
            <GitCommit size={14} />
            <span className="mono">Latest Block: {block}</span>
            <span style={{ margin: "0 8px" }}>|</span>
            <Clock size={14} />
            <span className="mono">Network TPS: {tps}</span>
          </div>
        </div>
      </main>
    </div>
  );
}
