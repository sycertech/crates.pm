import {AppState} from '../App';

const url = 'https://search.crates.pm/api/meili';
const indexUID = 'crates';
const apiKey = 'dfe530c6fd7ff313236262c15ce2b9f23d680fc2aabe7ee61d2492f4a0a9d17e';

export interface CrateHit {
	name: string;
	description?: string;
	keywords?: string[];
	categories?: string[];
	readme?: string;
	version?: string;
	downloads?: number;
}

export interface SearchResponse {
	hits: CrateHit[];
	query: string;
	processingTimeMs: number;
	limit: number;
	offset: number;
	estimatedTotalHits: number;
}

export async function fetchCrates(
	abort: AbortController,
	query: string,
): Promise<SearchResponse | undefined> {
	try {
		const response = await fetch(`${url}/indexes/${indexUID}/search`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Authorization: `Bearer ${apiKey}`,
			},
			body: JSON.stringify({
				q: query,
				limit: 25,
				offset: 0,
				sort: ['downloads:desc'],
			}),
			signal: abort.signal,
		});

		return response.json();
	} catch (err: unknown) {
		if (err instanceof DOMException && err.name === 'AbortError') {
			// ignore
		} else throw new Error('Failed to fetch crates');
	}
}

interface CountResponse {
	total: number;
}

export async function fetchCrateCount(state$: AppState): Promise<void> {
	const response = await fetch(`${url}/indexes/${indexUID}/documents/fetch`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${apiKey}`,
		},
		body: JSON.stringify({
			limit: 0,
			offset: 0,
		}),
	});

	const json = (await response.json()) as CountResponse;
	state$.count.set(json.total);
}
