export default function TechMarquee() {
  const techStack = [
    { name: "Stellar", logo: "/logos/stellar.png" },
    { name: "Soroban", logo: "/logos/soroban.png" },
    { name: "SP1", logo: "/logos/sp1.png" },
    { name: "Rust", logo: null },
    { name: "WASM", logo: null },
    { name: "ZK Proofs", logo: null },
  ];

  // Triplicate for a seamless infinite scroll
  const items = [...techStack, ...techStack, ...techStack];

  return (
    <div className="marquee-container">
      <div className="marquee-track">
        {items.map((tech, i) => (
          <div key={i} className="marquee-item">
            {tech.logo ? (
              // eslint-disable-next-line @next/next/no-img-element
              <img
                src={tech.logo}
                alt={tech.name}
                className="marquee-logo"
              />
            ) : (
              <span className="marquee-text">{tech.name}</span>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
