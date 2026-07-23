"use client";

import Link from "next/link";
import { Terminal, Server, Shield, ArrowRight } from "lucide-react";
import { useState, useEffect } from "react";

export default function LandingPage() {
  const [tps, setTps] = useState(42.1);
  const [activeProvers, setActiveProvers] = useState(142);
  const [proofsGen, setProofsGen] = useState(14029);

  // Mock live stats ticking
  useEffect(() => {
    const interval = setInterval(() => {
      setTps(t => +(t + (Math.random() * 4 - 2)).toFixed(1));
      if (Math.random() > 0.7) setProofsGen(p => p + 1);
      if (Math.random() > 0.9) setActiveProvers(p => p + (Math.random() > 0.5 ? 1 : -1));
    }, 2000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="layout-container" style={{ paddingBottom: "64px" }}>
      {/* Header */}
      <header className="header-nav">
        <div style={{ display: "flex", alignItems: "center", gap: "12px" }}>
          <div style={{ fontWeight: 700, fontSize: "18px", letterSpacing: "0.5px" }}>SADGI PROTOCOL</div>
          <div className="status-indicator" style={{ marginLeft: "16px" }}>
            <div className="status-dot green" />
            <span style={{ color: "var(--text-muted)" }}>TESTNET ONLINE</span>
          </div>
        </div>
        <div style={{ display: "flex", gap: "16px" }}>
          <a href="https://github.com/SadgiProtocol/sadgi-protocol" target="_blank" rel="noreferrer" className="btn-secondary" style={{ textDecoration: "none" }}>
            GitHub
          </a>
          <Link href="/marketplace" className="btn-primary" style={{ textDecoration: "none" }}>
            Launch App
          </Link>
        </div>
      </header>

      <main style={{ display: "flex", flexDirection: "column", gap: "64px", marginTop: "48px" }}>
        
        {/* Hero Section */}
        <section style={{ textAlign: "center", maxWidth: "800px", margin: "0 auto", padding: "32px 0" }}>
          <div className="mono" style={{ color: "var(--accent-blue)", marginBottom: "16px", textTransform: "uppercase", letterSpacing: "1px" }}>
            {/* Version 1.0.0-beta */}
            v1.0.0-beta
          </div>
          <h1 style={{ fontSize: "56px", fontWeight: 800, lineHeight: 1.1, marginBottom: "24px", letterSpacing: "-1px" }}>
            The Decentralized <br/> ZK Prover Marketplace
          </h1>
          <p style={{ fontSize: "18px", color: "var(--text-secondary)", lineHeight: 1.6, marginBottom: "40px" }}>
            Outsource complex computations to a decentralized network of provers. <br/>
            Verify them instantly on-chain. Powered by SP1 and Stellar Soroban.
          </p>
          
          <div style={{ display: "flex", gap: "16px", justifyContent: "center" }}>
            <Link href="/marketplace" className="btn-primary" style={{ textDecoration: "none", display: "inline-flex", alignItems: "center", gap: "8px", padding: "12px 24px", fontSize: "15px" }}>
              Launch Marketplace <ArrowRight size={16} />
            </Link>
            <a href="https://sadgiprotocol.github.io/sadgi-protocol/" className="btn-secondary" style={{ textDecoration: "none", display: "inline-flex", alignItems: "center", gap: "8px", padding: "12px 24px", fontSize: "15px" }}>
              Read the Docs
            </a>
          </div>
        </section>

        {/* Live Network Stats Banner */}
        <section className="panel" style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(200px, 1fr))", gap: "24px", textAlign: "center" }}>
          <div>
            <div className="log-header" style={{ borderBottom: "none" }}>Active Provers</div>
            <div className="mono" style={{ fontSize: "32px", color: "var(--text-primary)" }}>{activeProvers}</div>
          </div>
          <div>
            <div className="log-header" style={{ borderBottom: "none" }}>Total Proofs Generated</div>
            <div className="mono" style={{ fontSize: "32px", color: "var(--text-primary)" }}>{proofsGen.toLocaleString()}</div>
          </div>
          <div>
            <div className="log-header" style={{ borderBottom: "none" }}>Avg. Verification Cost</div>
            <div className="mono" style={{ fontSize: "32px", color: "var(--text-primary)" }}>0.001 XLM</div>
          </div>
          <div>
            <div className="log-header" style={{ borderBottom: "none" }}>Network TPS</div>
            <div className="mono" style={{ fontSize: "32px", color: "var(--accent-green)" }}>{tps.toFixed(1)}</div>
          </div>
        </section>

        {/* How It Works */}
        <section>
          <h2 style={{ fontSize: "24px", fontWeight: 600, marginBottom: "24px", borderBottom: "1px solid var(--border-subtle)", paddingBottom: "16px" }}>
            System Architecture
          </h2>
          <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(300px, 1fr))", gap: "24px" }}>
            
            <div className="panel-interactive" style={{ display: "flex", flexDirection: "column", gap: "16px" }}>
              <Terminal size={32} color="var(--accent-blue)" />
              <h3 style={{ fontSize: "18px", fontWeight: 600 }}>1. Submit Job</h3>
              <p style={{ color: "var(--text-secondary)", fontSize: "14px", lineHeight: 1.5 }}>
                Developers submit arbitrary Rust/SP1 computations to the decentralized queue via the Stellar network, along with a bounty.
              </p>
            </div>

            <div className="panel-interactive" style={{ display: "flex", flexDirection: "column", gap: "16px" }}>
              <Server size={32} color="var(--accent-green)" />
              <h3 style={{ fontSize: "18px", fontWeight: 600 }}>2. Generate Proof</h3>
              <p style={{ color: "var(--text-secondary)", fontSize: "14px", lineHeight: 1.5 }}>
                A decentralized network of Prover Nodes race to execute the SP1 program and generate a Zero-Knowledge Proof.
              </p>
            </div>

            <div className="panel-interactive" style={{ display: "flex", flexDirection: "column", gap: "16px" }}>
              <Shield size={32} color="var(--text-primary)" />
              <h3 style={{ fontSize: "18px", fontWeight: 600 }}>3. Verify On-Chain</h3>
              <p style={{ color: "var(--text-secondary)", fontSize: "14px", lineHeight: 1.5 }}>
                The proof is cryptographically verified on-chain via the Soroban Smart Contract, ensuring trustless execution.
              </p>
            </div>

          </div>
        </section>

      </main>

      {/* Footer */}
      <footer style={{ marginTop: "64px", borderTop: "1px solid var(--border-subtle)", paddingTop: "32px", display: "flex", justifyContent: "space-between", color: "var(--text-muted)", fontSize: "13px" }}>
        <div>© 2026 Sadgi Protocol. All rights reserved.</div>
        <div style={{ display: "flex", gap: "24px" }}>
          <a href="https://github.com/SadgiProtocol" target="_blank" rel="noreferrer" style={{ color: "inherit", textDecoration: "none" }}>GitHub</a>
          <a href="https://sadgiprotocol.github.io/sadgi-protocol/" style={{ color: "inherit", textDecoration: "none" }}>Documentation</a>
        </div>
      </footer>

    </div>
  );
}
