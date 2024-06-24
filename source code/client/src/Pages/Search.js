import { useFetch } from "../useFetch"
import Error from "./Error";
import Loading from "./Loading"
import "./Search.css"
import ReactStars from "react-rating-stars-component";
import { useLocation, useNavigate} from "react-router-dom";
import {useDispatch} from "react-redux";
import {useState} from "react";
import { replaceObject } from "../app/store";
import Chart from 'chart.js/auto';
import {Line} from 'react-chartjs-2';

export default function Search() {
    const location = useLocation();
    const dispatch = useDispatch();
    const urlsp = new URLSearchParams(location.search);
    const query_term = urlsp.get("q");
    const evalMode = urlsp.get("evalMode");
    const show = urlsp.get("show");
    const { loading, error, data } = useFetch(`http://localhost:8085/search/${query_term}`);
    const [relArr, setRelArray] = useState(Array(20).fill(false));
    if (loading) return <Loading />;

    if (error) return <Error />

    dispatch(replaceObject(data))
    // console.log(relArr);
    return <>
    <div style={{display:"grid",background:"rgb(26, 28, 25)",padding:"1em", boxShadow:"5px 0px 5px black",visibility:show==="true"?"visible":"hidden", width: show==="true"?"50vw":"0em",position:"absolute", top:0, left:0, zIndex:2, height: show==="true"?"100vh":"0em",}} className="sidebar">
        <p className="primary-text headline-small">P-R Curve</p>
        <Line data={{labels:getRecall(relArr, data.results.length, 21),datasets:[{
        label:"Precision",
        data:getPrecision(relArr, data.results.length),
        backgroundColor:"#75d0dd",
      borderColor:"#75d0dd",
      pointRadius:0,
      // fill:{value: 0}
      }, {
        label:"Interpolated Precision",
        data:getIPrecision(relArr, data.results.length),
        backgroundColor:"##ff0000",
      borderColor:"#ff0000",
      pointRadius:0,
      // fill:{value: 0}
      },]}}
      />
      <button onClick={()=>{submitRelevanceFeedback(data.results, relArr)}} style={{border:"none", padding:"0.5em", margin:"0.5em", alignSelf:"end"}} className="tertiary-container on-tertiary-container-text">Submit Relevance Feedback</button>
    </div>
    
    <div className="SearchWrap">
    <div style={{width:"100%", display:"grid", padding:"0.2em 2em", gridTemplateColumns:"1fr 1fr 1fr"}} className="top on-surface-text">
        <span>Time: {data.time/1000}secs</span>
        <span>Top {data.results.length} results</span>
        <span>Query: {data.query}</span>
    </div>
        <div className="results">
            {data.results.map((item, index) => {
                return evalMode==="true"?<div style={{display:"grid", gridTemplateColumns: "10fr 1fr"}} key={index}><SearchItem index={index} item={item} /><input onChange={(e)=>setRelArray({...relArr, [index]:e.target.checked})} style={{width:"1em"}} type="checkbox"></input></div>:<SearchItem key={index} index={index} item={item} />
            })}
        </div>
        <div></div>
    </div></>
}

function SearchItem({ item, index }) {
    const navigate = useNavigate();
    return <div onClick={
        ()=>{
            navigate(`/u/result/${index}`)
        }
    } className="searchItem secondary-container">
        {/* <img className="searchImage" src={item.image} alt="img" /> */}
        <div>
            <span className="on-secondary-container-text headline-small">{item[1].title}</span>
            {/* <div className="df"><ReactStars
                count={5}
                isHalf={true}
                value={item.rating}
                size={16}
                activeColor="rgb(212, 232, 208)"
                edit={false}
            /> <span className="on-secondary-container-text">   {item.num_ratings}</span></div> */}
            <br/><br/>
            <span className="on-secondary-container-text">{item[1].article.slice(0,Math.min(300, item[1].article.length))}..............</span>
        </div>

    </div>
}

function getRecall(relArr, len, total){
    let recall = [];
    let upto = 0;
    for(let i=0; i<len; i++){
        if(relArr[i]===true){
            upto+=1;
        }
        recall.push(upto/total);
    }
    return recall;
}

function getPrecision(relArr, len){
    let precision = [];
    let upto = 0;
    for(let i=0; i<len; i++){
        if(relArr[i]===true){
            upto+=1;
        }
        precision.push(upto/(i+1));
    }
    return precision;
}

function getIPrecision(relArr, len){
    let precision = getPrecision(relArr, len);
    let i_precision = [];
    for(let i=0; i<len; i++){
        i_precision.push(Math.max(...precision.slice(i)));
    }
    // console.log(precision, i_precision)
    return i_precision;
}

function submitRelevanceFeedback(items, relArr) {
    let relevantDocs = [];
    for (let i =0; i<items.length; i++) {
        if(relArr[i]){
            relevantDocs.push(items[i][1].doc_id);
        }
    }
    relevantDocs = relevantDocs.join(",");
    fetch(`http://localhost:8085/rel/${relevantDocs}`).then((result)=>{console.log(result);return result;}).then((msg)=>alert("Feedback Sent"));
}