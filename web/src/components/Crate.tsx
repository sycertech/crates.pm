import {CrateHit} from '../util/meili';

interface CrateProps {
	hit: CrateHit;
}

const beautify = (n: number) => {
	const exponent = (Math.log10(n) / 3) | 0;
	const suffixes = ['', 'K', 'M', 'B', 'T'];

	if (exponent == 0) {
		return n;
	}

	const divisor = Math.pow(10, exponent * 3);
	const shortened = n / divisor;
	const rounded = Number(shortened.toFixed(1));
	const suffix = suffixes[exponent];

	return rounded + suffix;
};

export function Crate({hit}: CrateProps) {
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
					{hit.keywords?.map(k => (
						<span className="k">
							<span>#</span>
							{k}
						</span>
					))}
				</div>
			</a>
		</li>
	);
}
