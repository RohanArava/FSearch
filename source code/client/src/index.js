import React from 'react';
import ReactDOM from 'react-dom/client';
import {
  RouterProvider,
} from "react-router-dom";
import router from './Routing';
import {Provider} from "react-redux";
import 'bootstrap/dist/css/bootstrap.min.css';
import './css/theme.css';
import './index.css';
import {store} from "./app/store";
import reportWebVitals from './reportWebVitals';



const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
  <React.StrictMode>
    <Provider store={store}>
    <RouterProvider router={router} />
    </Provider>
  </React.StrictMode>
);


reportWebVitals();
