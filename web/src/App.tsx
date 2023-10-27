import {CrateHit} from './util/meili';
import {ObservableObject} from '@legendapp/state';
import { Home } from './pages/Home';

export type InnerAppState = {
	hits: CrateHit[];
	count: number;
	requestTime: string;
};

export type AppState = ObservableObject<InnerAppState>;

function App() {
	return (
		<>
			<Home />
		</>
	);
}

export default App;
