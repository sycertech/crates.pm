import {CrateHit, fetchCrateCount} from '../util/meili';
import {ObservableObject} from '@legendapp/state';
import {useObservable} from '@legendapp/state/react';
import {performSearch} from '../util/search';
import {Header} from '../components/Header';
import {Main} from '../components/Main';
import {useEffect} from 'react';

export type InnerAppState = {
	hits: CrateHit[];
	count: number;
	requestTime: string;
};

export type AppState = ObservableObject<InnerAppState>;

export function Home() {
	const state$ = useObservable<InnerAppState>({
		hits: [],
		requestTime: '',
		count: 0,
	});

	useEffect(() => {
		fetchCrateCount(state$);
		performSearch(state$, '');
	});

	return (
		<>
			<Header state$={state$} />
			<Main state$={state$} />
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
