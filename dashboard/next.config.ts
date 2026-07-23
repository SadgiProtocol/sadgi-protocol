import type { NextConfig } from "next";

const isProd = process.env.NODE_ENV === "production";

const nextConfig: NextConfig = {
  output: "export",
  // When deployed to GitHub Pages at /sadgi-protocol/
  basePath: isProd ? "/sadgi-protocol" : "",
  assetPrefix: isProd ? "/sadgi-protocol" : "",
  trailingSlash: true,
  // Disable image optimisation (not supported in static export)
  images: {
    unoptimized: true,
  },
};

export default nextConfig;
