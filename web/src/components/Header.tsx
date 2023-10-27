import {observer} from '@legendapp/state/react';
import {AppState} from '../App';
import {performSearch} from '../util/search';

export const Header = observer(function Header({state$}: {state$: AppState}) {
	return (
		<header id="serp">
			<div className="inner-col">
				<h2>search.crates.pm</h2>
				<p>
					<h4>Meilisearch-backed Crate searching</h4>
					<br />
					This project is <a href="https://github.com/sycertech/search.crates.pm">open source</a>!
					<br />
					This design was taken from <a href="https://lib.rs">lib.rs</a>.
					<br />
					<br />
					Meilisearch endpoint:{' '}
					<a href="https://search.crates.pm/api/meili/health">https://search.crates.pm/api/meili</a>
					<br />
					API Key: <code>42c23e43c9529fd1f1e47f4eb4d428b571f18e099d32091cc85b8546bb9d1be7</code>
					<br />
					Please set an identifiable User-Agent header.
					<br />
					<br />
					We currently have <code>{state$.count.get().toLocaleString()}</code> crates indexed!
				</p>

				<form role="search" id="search">
					<input
						placeholder="name, keywords, description"
						autoFocus
						autoCapitalize="off"
						autoCorrect="off"
						autoComplete="off"
						tabIndex={1}
						type="search"
						id="textSearch"
						onInput={event => performSearch(state$, event.currentTarget.value)}
					/>
					<span id="request-time">{state$.requestTime.get()}</span>
				</form>
				<nav>
					<ul>
						<li className="active">Sorted by relevance</li>
					</ul>
				</nav>
			</div>
		</header>
	);
});
