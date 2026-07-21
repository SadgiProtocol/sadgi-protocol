import type { NextConfig } from "next";

const isProd = process.env.NODE_ENV === "production";

const nextConfig: NextConfig = {
  output: "export",
  // When deployed to GitHub Pages at /sadgi-protocol/app/
  basePath: isProd ? "/sadgi-protocol/app" : "",
  assetPrefix: isProd ? "/sadgi-protocol/app" : "",
  trailingSlash: true,
  // Disable image optimisation (not supported in static export)
  images: {
    unoptimized: true,
  },
};

export default nextConfig;
