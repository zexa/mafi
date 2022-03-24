/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
}

module.exports = {
  serverRuntimeConfig: {
    apiUrl: "http://localhost:3001"
  },
  async redirects() {
    return [
      {
        source: '/',
        destination: '/login',
        permanent: false,
      }
    ];
  }
}
