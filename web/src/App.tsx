import {CrateHit, fetchCrateCount} from './util/meili';
import {ObservableObject} from '@legendapp/state';
import {useObservable} from '@legendapp/state/react';
import {performSearch} from './util/search';
import {Header} from './components/Header';
import {Main} from './components/Main';

export type InnerAppState = {
	hits: CrateHit[];
	count: number;
	requestTime: string;
};

export type AppState = ObservableObject<InnerAppState>;

function App() {
	const state$ = useObservable<InnerAppState>({
		hits: [],
		requestTime: '',
		count: 0,
	});
	performSearch(state$, '');
	fetchCrateCount(state$);

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

export default App;
