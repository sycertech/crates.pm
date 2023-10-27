import {enableReactTracking} from '@legendapp/state/config/enableReactTracking';
import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App.tsx';
import './index.css';
import { BrowserRouter } from 'react-router-dom';

enableReactTracking({
	auto: true,
});

ReactDOM.hydrateRoot(
  document.getElementById('app')!,
  <BrowserRouter>
    <App />
  </BrowserRouter>,
)
console.log('hydrated')
