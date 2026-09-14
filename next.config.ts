import type { NextConfig } from "next"
import createNextIntlPlugin from "next-intl/plugin"

const isProd = process.env.NODE_ENV === "production"
const internalHost = process.env.TAURI_DEV_HOST || "localhost"
const withNextIntl = createNextIntlPlugin({
  requestConfig: "./src/i18n/request.ts",
})

const nextConfig: NextConfig = {
  output: "export",
  images: {
    unoptimized: true,
  },
  assetPrefix: isProd ? undefined : `http://${internalHost}:3000`,
  // 调试构建：跳过 TypeScript 和 ESLint 检查，加快构建速度
  typescript: {
    ignoreBuildErrors: true,
  },
  eslint: {
    ignoreDuringBuilds: true,
  },
}

export default withNextIntl(nextConfig)
