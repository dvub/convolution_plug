import { Azeret_Mono } from 'next/font/google';
import './globals.css';

const azeret = Azeret_Mono({
	variable: '--font-azeret-mono',
	subsets: ['latin'],
});

export default function RootLayout({
	children,
}: Readonly<{
	children: React.ReactNode;
}>) {
	return (
		<html lang='en'>
			<body className={`${azeret.className} antialiased px-1`}>
				{children}
			</body>
		</html>
	);
}
