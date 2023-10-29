import { observer } from '@legendapp/state/react';
import { useEffect, useState } from 'react';
import { performSearch } from '../util/search';
import type { AppState } from '../util/state';

export const Header = observer(({ state$ }: { state$: AppState }) => {
	const [buttonContent, setButtonContent] = useState('Copy');
	const changeButtonState = () => {
		setButtonContent('Copied!');
		setTimeout(() => {
			setButtonContent('Copy');
		}, 3_000);
	};

	const [healthColor, setHealthColor] = useState('#6B7280');
	useEffect(() => {
		const health = async () => {
			const response = await fetch(`${process.env.NEXT_PUBLIC_MEILISEARCH_ADDRESS}/health`);
			const json = (await response.json()) as { status: 'available' | 'unavailable' };
			if (json.status === 'available') {
				setHealthColor('#48BB78');
			} else {
				setHealthColor('#EF4444');
			}
		};

		void health();
	}, []);

	return (
		<header id="serp">
			<div className="inner-col">
				<h2>search.crates.pm</h2>
				<h4>Meilisearch-backed Crate searching</h4>
				<p>
					<br />
					This project is <a href="https://github.com/sycertech/search.crates.pm">open source</a>!
					<br />
					This design was taken from <a href="https://lib.rs">lib.rs</a>.
					<br />
					<br />
					Meilisearch endpoint:{' '}
					<span>
						<span
							style={{
								backgroundColor: healthColor,
								height: '0.5rem',
								width: '0.5rem',
								display: 'inline-block',
								borderRadius: '9999px',
								marginRight: '0.5rem',
							}}
						/>
						<a href="https://crates.pm/api/meili/health">https://crates.pm/api/meili</a>
					</span>
					<br />
					API Key:{' '}
					<button
						type="submit"
						style={{ cursor: 'grab' }}
						onClick={async () => {
							await navigator.clipboard.writeText(process.env.NEXT_PUBLIC_MEILISEARCH_API_KEY!);
							changeButtonState();
						}}
					>
						{buttonContent}
					</button>
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
						// tabIndex={1}
						type="search"
						id="textSearch"
						onInput={async (event) => performSearch(state$, event.currentTarget.value)}
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
