'use client';
import { fetchCrates } from './meili';
import type { AppState } from './state';

// todo: reimplement debounce
// let abortController: AbortController | undefined;

export const performSearch = async (state$: AppState, input: string) => {
	try {
		const res = await fetchCrates(new AbortController()!, input ?? '');

		if (res) {
			state$.set({
				count: state$.count.get(),
				hits: res.hits,
				requestTime: `${res.processingTimeMs}ms`,
			});
		}
	} catch (error: unknown) {
		if (error instanceof DOMException && error.name === 'AbortError') {
			return;
		}

		throw error;
	}
};
