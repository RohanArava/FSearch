import {Outlet} from "react-router-dom";
import { useNavigate } from "react-router-dom";
import { useLocation } from "react-router-dom";
import "./User.css";
import { useState, useEffect} from "react";

export default function User(){
    const [searchTerm, setSearchTerm] = useState("");
    const [evalMode, setEvalMode] = useState(false);
    const [show, setShow] = useState(false);
    const navigate = useNavigate();
    return <div className="user">
        <div className="userHeader">
            <span className="headline-medium primary-text">International News</span>
            <div className="searchbar">
            <input value={searchTerm} onKeyDown={(e) => {
                        if (e.key === "Enter") {
                            console.log(searchTerm)
                            navigate(searchTerm&&searchTerm!==""?`/u/search?q=${searchTerm}&evalMode=${evalMode}&show=${show}`:`/u/search?q=australian wildfires&evalMode=${evalMode}&show=${show}`)
                        }}} onChange={(e)=>{
                setSearchTerm(e.target.value)
            }} className="searchInput" placeholder="Search" type="text"/>
            <button onClick={()=>{
                navigate(searchTerm&&searchTerm!==""?`/u/search?q=${searchTerm}&evalMode=${evalMode}&show=${show}`:`/u/search?q=australian wildfires&evalMode=${evalMode}&show=${show}`)
            }} className="searchSubmit on-secondary-container-text"><span className="material-symbols-rounded">search</span></button>
            </div>
            <div  onClick={()=>{
                if(evalMode){
                    navigate(searchTerm&&searchTerm!==""?`/u/search?q=${searchTerm}&evalMode=false&show=${show}`:`/u/search?q=australian wildfires&evalMode=false&show=${show}`)
                }else{
                    navigate(searchTerm&&searchTerm!==""?`/u/search?q=${searchTerm}&evalMode=true&show=${show}`:`/u/search?q=australian wildfires&evalMode=true&show=${show}`)
                }
                setEvalMode(!evalMode);
                
            }}>
                <span className="material-symbols-rounded header-medium primary-text">
                    check_circle
                </span>
            </div>
            {/* <div>
                <span className="material-symbols-rounded header-medium primary-text">
                    info
                </span>
            </div> */}
            <div onClick={()=>{
                if(show){
                    navigate(searchTerm&&searchTerm!==""?`/u/search?q=${searchTerm}&evalMode=${evalMode}&show=false`:`/u/search?q=australian wildfires&evalMode=${evalMode}&show=false`)
                }else{
                    navigate(searchTerm&&searchTerm!==""?`/u/search?q=${searchTerm}&evalMode=${evalMode}&show=true`:`/u/search?q=australian wildfires&evalMode=${evalMode}&show=true`)
                }
                setShow(!show);
            }}>
                <span className="material-symbols-rounded header-medium primary-text">
                    menu
                </span>
            </div>
        </div>
        <Outlet/>
    </div>
}