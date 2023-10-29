import './globals.css';
import { URL } from 'node:url';
import type { Metadata, Viewport } from 'next';
import { JetBrains_Mono } from 'next/font/google';
import PlausibleProvider from 'next-plausible';

const jetbrainsMono = JetBrains_Mono({ subsets: ['latin'] });

export const viewport: Viewport = {
	themeColor: '#dfa83e',
};

export const metadata: Metadata = {
	metadataBase: new URL(process.env.NEXT_PUBLIC_METADATA_BASE!),
	title: 'crates.pm',
	description: 'A search engine for Crates',
	icons: {
		icon: '/cargo.png',
	},
	openGraph: {
		type: 'website',
		title: 'crates.pm',
		locale: 'en_US',
		alternateLocale: 'en_GB',
		description: 'A search engine for Crates',
		images: [
			{
				url: '/cargo.png',
				width: 225,
				height: 225,
				alt: 'Crates Logo',
			},
		],
	},
	twitter: {
		site: '@fykowo',
		creator: '@fykowo',
		title: 'crates.pm',
		description: 'A search engine for Crates',
		images: [
			{
				url: '/cargo.png',
				width: 225,
				height: 225,
				alt: 'Crates Logo',
			},
		],
		card: 'summary',
	},
};

export default function RootLayout({ children }: { readonly children: React.ReactNode }) {
	return (
		<html lang="en">
			<head>
				<PlausibleProvider
					domain={process.env.NEXT_PUBLIC_PLAUSIBLE_DOMAIN!}
					customDomain={process.env.NEXT_PUBLIC_PLAUSIBLE_LOCATION!}
					taggedEvents
					trackOutboundLinks
				/>
			</head>
			<body className={jetbrainsMono.className}>{children}</body>
		</html>
	);
}
