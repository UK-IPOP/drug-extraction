/** @type {import('next').NextConfig} */
const nextConfig = {
	reactStrictMode: true,
	async rewrites() {
		return [
			{
				source: '/results.csv',
				destination: '/api/data',
			},
		];
	},
};

module.exports = nextConfig;
