import {AppState} from '../App';
import {fetchCrates} from './meili';

let abortController: AbortController | undefined = undefined;

export const performSearch = async (state$: AppState, input: string) => {
	try {
		abortController = new AbortController();
		const res = await fetchCrates(abortController!, input ?? '');
		abortController = undefined;

		if (res) {
			state$.set({
				count: state$.count.get(),
				hits: res.hits,
				requestTime: `${res.processingTimeMs}ms`,
			});
		}
	} catch (err: unknown) {
		if (err instanceof DOMException && err.name === 'AbortError') {
			return;
		}
		throw err;
	}
};
