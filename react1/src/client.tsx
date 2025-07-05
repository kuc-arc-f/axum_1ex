import ReactDOM from 'react-dom/client'
import React from 'react'
import ReactDOM from "react-dom/client";
import { BrowserRouter } from "react-router";

import App from './App';

ReactDOM.createRoot(document.getElementById('app')).render(
  <BrowserRouter>
    <App />
  </BrowserRouter>
)
console.log('createRoot')
