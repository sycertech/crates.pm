import type { ObservableObject } from '@legendapp/state';
import type { CrateHit } from './meili';

export type InnerAppState = {
	count: number;
	hits: CrateHit[];
	requestTime: string;
};

export type AppState = ObservableObject<InnerAppState>;
