
/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  /*async redirects() {
    return [
      {
        source: '/login',
        destination: 'http://localhost:8000/gnap/login/',
        permanent: false
      }
    ]
  }*/
  experimental: {
    outputStandalone: true,
  },
}
module.exports = nextConfig
