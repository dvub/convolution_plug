import type { Metadata } from 'next';
import { Azeret_Mono } from 'next/font/google';
import './globals.css';

const azeret = Azeret_Mono({
	variable: '--font-azeret-mono',
	subsets: ['latin'],
});

// TODO: is metadata needed?
export const metadata: Metadata = {
	title: 'CONVOLUTION UI',
};

export default function RootLayout({
	children,
}: Readonly<{
	children: React.ReactNode;
}>) {
	return (
		<html lang='en'>
			<body className={`${azeret.className} antialiased`}>
				{children}
			</body>
		</html>
	);
}
