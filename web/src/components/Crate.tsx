import type { CrateHit } from '../util/meili';

type CrateProps = {
	readonly hit: CrateHit;
};

const beautify = (number: number) => {
	const exponent = Math.trunc(Math.log10(number) / 3);
	const suffixes = ['', 'K', 'M', 'B', 'T'];

	// eslint-disable-next-line eqeqeq
	if (exponent == 0) {
		return number;
	}

	const divisor = 10 ** (exponent * 3);
	const shortened = number / divisor;
	const rounded = Number(shortened.toFixed(1));
	const suffix = suffixes[exponent] ?? '';

	return rounded + suffix;
};

export function Crate({ hit }: CrateProps) {
	return (
		<li key={hit.name}>
			<a href={`https://crates.pm/${hit.name}`}>
				<div className="h">
					<h4>{hit.name}</h4>
					<p className="desc">{hit.description}</p>
				</div>
				<div className="meta">
					<span className="version stable">
						<span>v</span>
						{hit.version}
					</span>
					{hit.downloads ? (
						<span className="downloads" title={`${hit.downloads} recent downloads`}>
							{beautify(hit.downloads)}
						</span>
					) : (
						<></>
					)}
					{hit.keywords?.map((keyword) => (
						<span className="k">
							<span>#</span>
							{keyword}
						</span>
					))}
				</div>
			</a>
		</li>
	);
}
