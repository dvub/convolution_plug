import { Azeret_Mono } from 'next/font/google';
import './globals.css';

const azeret = Azeret_Mono({
	variable: '--font-azeret-mono',
	subsets: ['latin'],
});

import type { Viewport } from 'next';

export const viewport: Viewport = {
	width: 'device-width',
	initialScale: 1,
};

export default function RootLayout({
	children,
}: Readonly<{
	children: React.ReactNode;
}>) {
	return (
		<html lang='en'>
			<body
				className={`${azeret.className} antialiased px-1 h-full w-full overflow-hidden`}
			>
				{children}
			</body>
		</html>
	);
}
