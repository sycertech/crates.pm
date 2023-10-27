import {AppState} from '../App';
import {Crate} from './Crate';

export function Main({state$}: {state$: AppState}) {
	return (
		<main id="results">
			<div className="inner-col">
				<ol id="crate-list">
					{state$.hits.get().map(crate => (
						<Crate hit={crate} key={crate.name} />
					))}
				</ol>
			</div>
		</main>
	);
}
