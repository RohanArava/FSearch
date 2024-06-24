import {
  createBrowserRouter,
  Route,
  createRoutesFromElements
} from "react-router-dom";

import App from "./App";
import Notifications from "./Pages/Notifications";
import User from "./Pages/User";
import Search from "./Pages/Search";
import Profile from "./Pages/Profile";
import { Navigate } from 'react-router-dom';
import Service from "./Pages/Service";

const router = createBrowserRouter(
  createRoutesFromElements(
    <Route path="/" element={<App />}>
      <Route path="u" element={<User/>}>
        <Route path="search" element={<Search />} />
        <Route path="eval" element={<Profile />} />
        <Route path="result/:id" element={<Service />} />
        <Route path="" element={<Navigate to="search?q=australia wildfires&evalMode=false"/>}/>
      </Route>
      <Route path="" element={<Navigate to="u"/>}/>
    </Route>
  ), {
  location: "/business/dash"
});
export default router; 