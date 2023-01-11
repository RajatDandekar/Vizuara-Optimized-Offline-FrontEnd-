/** @type {import('next').NextConfig} */

const ContentSecurityPolicy = `
  frame-src *;
`

const securityHeaders = {
  key: 'Content-Security-Policy',
  value: ContentSecurityPolicy.replace(/\s{2,}/g, ' ').trim()
}
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  images: {
    unoptimized: true,
  }
};

module.exports = nextConfig;
