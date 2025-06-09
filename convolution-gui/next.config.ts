import type { NextConfig } from 'next';

const nextConfig: NextConfig = {
	/* config options here */
	output: 'export',
	// when we do static export, we cant use image optimization (i guess)
	images: {
		unoptimized: true,
	},
	distDir: 'assets',
};

export default nextConfig;
