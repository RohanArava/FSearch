import { useLocation } from "react-router-dom";
import {useNavigate} from "react-router-dom";
import "./Service.css";
import ReactStars from "react-rating-stars-component";
import {useSelector} from "react-redux";
import { useEffect } from "react";
export default function Service(){
    const location = useLocation();
    const navigate = useNavigate();
    const id = parseInt(location.pathname.split("/")[3]);
    const data = useSelector(state => state.objReducer.object);
    console.log("data",data)
    console.log(id)
    useEffect(()=>{
        console.log("here");
        if (!data.results||data.results.length<id+1){
            console.log("hrre")
        navigate(`u/search?q=australia wildfires`);
    }
    }, [data, id]);
    if (!data.results || data.results.length<id+1 || !data.results[id][1]){
        return <div></div>    
    }
    let paras = data.results[id][1].article.split(".");
    let paras1 = paras.slice(0, Math.floor(paras.length/2));
    let paras2 = paras.slice(Math.floor(paras.length/2), paras.length);
    return <div className="serviceWrap">
        <div className="serviceUp">
        <div className="details padx-8">
            <p className=" headline-small primary-text">{data.results[id][1].title}</p><br/>
            <p className="on-surface-text body-large primary-text">{
                paras1.join(".")
            }.</p>
            <p className="on-surface-text body-large primary-text">{paras2.join(".")}.</p>
        </div>
        </div>
    </div>
}

function ReviewItem({review}){
    return <div className="secondary-container reviewItem">
        <p className="on-secondary-container-text title-medium">{review.user}</p>
        <div className="df"><ReactStars
                count={5}
                isHalf={true}
                value={review.rating}
                size={16}
                activeColor="rgb(212, 232, 208)"
                edit={false}
            /> <span className="on-secondary-container-text">{review.rating}</span></div>
            <p className="on-secondary-container-text body-medium">{review.review}</p>
    </div>
}