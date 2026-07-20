"use client";

import { Terminal, Database, Activity, GitCommit, FileCode, CheckCircle2, Clock } from "lucide-react";
import { useState, useEffect } from "react";

// Mocking the new JobState from `state.rs`
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

export default function Dashboard() {
  const [mounted, setMounted] = useState(false);
  const [jobs, setJobs] = useState<Job[]>([
    { id: "1049", program: "Hash Verification", bounty: "50 XLM", state: "Settled", prover: "GARV...3K2A", receiptHash: "0x8f2b...1c3a", time: "2m ago" },
    { id: "1050", program: "Hello World", bounty: "10 XLM", state: "Verified", prover: "SADG...9L0P", receiptHash: "0x1a4c...9d2e", time: "1m ago" },
    { id: "1051", program: "Hash Verification", bounty: "50 XLM", state: "Executing", prover: "GARV...3K2A", time: "Just now" },
    { id: "1052", program: "Custom Program", bounty: "100 XLM", state: "Queue", time: "Just now" },
  ]);

  useEffect(() => {
    setMounted(true);
  }, []);

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
          <button className="btn-secondary">Connect Node</button>
        </div>
      </header>

      {/* Main Grid */}
      <main className="grid-dashboard">
        
        {/* Left Sidebar: Program Registry */}
        <div style={{ display: "flex", flexDirection: "column", gap: "16px" }}>
          <h2 style={{ fontSize: "14px", fontWeight: 600, color: "var(--text-muted)", textTransform: "uppercase", letterSpacing: "0.5px" }}>
            Program Registry
          </h2>
          
          <div className="panel-interactive" style={{ display: "flex", flexDirection: "column", gap: "8px" }}>
            <div style={{ display: "flex", alignItems: "center", gap: "8px", marginBottom: "4px" }}>
              <FileCode size={16} color="var(--text-primary)" />
              <h3 style={{ fontSize: "14px", fontWeight: 500 }}>Hello World</h3>
            </div>
            <p className="mono" style={{ color: "var(--text-secondary)" }}>PID: 0x0000...0001</p>
            <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginTop: "12px" }}>
              <span style={{ fontSize: "12px", color: "var(--text-muted)" }}>Platform Primitive</span>
              <button className="btn-secondary" style={{ padding: "4px 8px", fontSize: "12px" }}>Invoke</button>
            </div>
          </div>

          <div className="panel-interactive" style={{ display: "flex", flexDirection: "column", gap: "8px" }}>
            <div style={{ display: "flex", alignItems: "center", gap: "8px", marginBottom: "4px" }}>
              <Database size={16} color="var(--text-primary)" />
              <h3 style={{ fontSize: "14px", fontWeight: 500 }}>Hash Verification</h3>
            </div>
            <p className="mono" style={{ color: "var(--text-secondary)" }}>PID: 0x0000...0002</p>
            <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginTop: "12px" }}>
              <span style={{ fontSize: "12px", color: "var(--text-muted)" }}>Platform Primitive</span>
              <button className="btn-secondary" style={{ padding: "4px 8px", fontSize: "12px" }}>Invoke</button>
            </div>
          </div>

          <button className="btn-primary" style={{ marginTop: "8px", width: "100%" }}>
            Deploy Custom Program
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
              {jobs.map((job) => (
                <div key={job.id} className="log-row" style={{ fontSize: "13px" }}>
                  <div className="mono" style={{ color: "var(--text-primary)" }}>#{job.id}</div>
                  <div>{renderStatus(job.state)}</div>
                  <div className="mono" style={{ color: "var(--text-secondary)" }}>{job.bounty}</div>
                  <div style={{ fontWeight: 500 }}>{job.program}</div>
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
            <span className="mono">Latest Block: 492011</span>
            <span style={{ margin: "0 8px" }}>|</span>
            <Clock size={14} />
            <span className="mono">Network TPS: 42.1</span>
          </div>
        </div>

      </main>
    </div>
  );
}
