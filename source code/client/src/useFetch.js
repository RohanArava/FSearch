import {useState, useEffect} from "react";
export function useFetch(uri){
    const [data, setData] = useState();
    const [error, setError] = useState();
    const [loading, setLoading] = useState(true);
    useEffect(() => {
        if (!uri) {
            setLoading(false)
            return;
        }
        fetch(uri)
        .then(data => data.json())
        .then(data => { return data;})
        .then(setData)
        .then(()=>setLoading(false))
        .catch(e=>{console.log(e);setError(e);setLoading(false)});
    }, [uri]);

    return {
        loading,
        data,
        error
    };
}