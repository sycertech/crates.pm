'use client';
import type { AppState } from './state';

export type CrateHit = {
	categories?: string[];
	description?: string;
	downloads?: number;
	keywords?: string[];
	name: string;
	readme?: string;
	version?: string;
};

export type SearchResponse = {
	estimatedTotalHits: number;
	hits: CrateHit[];
	limit: number;
	offset: number;
	processingTimeMs: number;
	query: string;
};

export async function fetchCrates(abort: AbortController, query: string): Promise<SearchResponse | undefined> {
	try {
		const response = await fetch(
			`${process.env.NEXT_PUBLIC_MEILISEARCH_ADDRESS}/indexes/${process.env.NEXT_PUBLIC_MEILISEARCH_CRATES_INDEX_UID}/search`,
			{
				cache: 'no-cache',
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					Authorization: `Bearer ${process.env.NEXT_PUBLIC_MEILISEARCH_API_KEY}`,
				},
				body: JSON.stringify({
					// eslint-disable-next-line id-length
					q: query,
					limit: 25,
					offset: 0,
					sort: ['downloads:desc'],
				}),
				signal: abort.signal,
			},
		);

		return await response.json();
	} catch (error: unknown) {
		if (error instanceof DOMException && error.name === 'AbortError') {
			// ignore
		} else throw new Error('Failed to fetch crates');
	}

	return undefined;
}

type CountResponse = {
	total: number;
};

export async function fetchCrateCount(state$: AppState): Promise<void> {
	const response = await fetch(
		`${process.env.NEXT_PUBLIC_MEILISEARCH_ADDRESS}/indexes/${process.env.NEXT_PUBLIC_MEILISEARCH_CRATES_INDEX_UID}/documents/fetch`,
		{
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${process.env.NEXT_PUBLIC_MEILISEARCH_API_KEY}`,
			},
			body: JSON.stringify({
				limit: 0,
				offset: 0,
			}),
		},
	);

	const json = (await response.json()) as CountResponse;
	state$.count.set(json.total);
}
