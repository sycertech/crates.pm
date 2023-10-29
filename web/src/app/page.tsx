'use client';

import { enableReactTracking } from '@legendapp/state/config/enableReactTracking';
import { useObservable } from '@legendapp/state/react';
import { useEffect } from 'react';
import { Crate } from '../components/Crate';
import { Header } from '../components/Header';
import { fetchCrateCount } from '../util/meili';
import { performSearch } from '../util/search';
import type { InnerAppState } from '../util/state';

enableReactTracking({ auto: true });

export default function Home() {
	const state$ = useObservable<InnerAppState>({
		hits: [],
		requestTime: '',
		count: 0,
	});

	useEffect(() => {
		void fetchCrateCount(state$);
		void performSearch(state$, '');
		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, []);

	return (
		<>
			<Header state$={state$} />
			<main id="results">
				<div className="inner-col">
					<ol id="crate-list">
						{state$.hits.get().map((crate) => (
							<Crate hit={crate} key={crate.name} />
						))}
					</ol>
				</div>
			</main>
			<footer>
				<div className="inner-col">
					<p>
						Forked from <a href="https://github.com/meilisearch/demos">Meilisearch Demos</a>.
					</p>
				</div>
			</footer>
		</>
	);
}
