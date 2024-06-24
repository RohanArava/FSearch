import { Outlet } from "react-router";
import {useEffect} from 'react';
import './App.css'
export default function App(){
    useEffect(() => {
        window.history.scrollRestoration = 'manual'
      }, []);
    return (
        <div className="surface app">
        <Outlet/>
        </div>
    )
};