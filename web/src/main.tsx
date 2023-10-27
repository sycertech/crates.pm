import {enableReactTracking} from '@legendapp/state/config/enableReactTracking';
import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App.tsx';
import './index.css';

enableReactTracking({
	auto: true,
});

ReactDOM.createRoot(document.getElementById('root')!).render(
	<React.StrictMode>
		<App />
	</React.StrictMode>,
);
