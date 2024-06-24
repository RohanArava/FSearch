use std::collections::{HashMap, HashSet};
use axum::{extract::Path, routing::get, Json, Router};
use rsnltk::api::natural::tokenize;
use rust_stemmers::{ Algorithm, Stemmer };
use stopwords::{ Language, Spark, Stopwords };
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use tower_http::cors::CorsLayer;

#[derive(serde::Deserialize)]
struct PositionalEntry {
    doc_id: usize,
    position: usize,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct Record{
    doc_id :usize,
    date :String,
    year: u16,
    month: f32,
    day: u16,
    author: String,
    title: String,
    article: String,
    url: String,
    section: String,
    publication: String,
    weight: f64
}

#[derive(Clone, Debug, serde::Serialize)]
struct ApiResponse{
    time: u128,
    query: String,
    results: Vec<(f64, Record)>
}



fn main() {
    let stops_no: std::collections::HashSet<_> = Spark::stopwords(Language::English)
        .unwrap()
        .iter()
        .collect();
    let mut stops:HashSet<String> = HashSet::new();
    for s in stops_no{
        stops.insert(s.to_string());
    }
    let en_stemmer = Stemmer::create(Algorithm::English);
    let re = regex::Regex::new(r"([a-z]+)").unwrap();
    let args :Vec<String> = std::env::args().collect();
    let file = std::fs::File::open(&args[1]).unwrap();
    let file_reader = std::io::BufReader::new(file);
    let mut reader = csv::ReaderBuilder::new().has_headers(false).from_reader(file_reader);
    let mut all_docs :Vec<(Record, u32)> = Vec::new();
    for result in reader.deserialize(){
        all_docs.push((result.unwrap(), 0));
    }
    serve(re, en_stemmer, stops, all_docs);
}

#[tokio::main]
async fn serve(re :regex::Regex, en_stemmer :Stemmer, stops :std::collections::HashSet<String>, all_docs :Vec<(Record, u32)>){
    let shared_state = Arc::new((re, en_stemmer, stops, Mutex::new(all_docs)));
    let app = Router::new().route(
        "/search/:query",
        get({
            let shared_state = Arc::clone(&shared_state);
            move |path| handle_request(path, shared_state)
        }),
    ).route(
        "/rel/:arr",
        get({
            let shared_state = Arc::clone(&shared_state);
            move |path| handle_relfeed(path, shared_state)
        })
    ).layer(CorsLayer::permissive());
    println!("Web Service started at localhost:8085");
    axum::Server::bind(&"0.0.0.0:8085".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    
}

async fn handle_relfeed(Path(arr): Path<String>, state :Arc<(regex::Regex, Stemmer, std::collections::HashSet<String>, Mutex<Vec<(Record, u32)>>)>) -> String {
    let arr_arr :Vec<usize> = arr.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
    let mut all_docs = state.3.lock().unwrap();
    for i in arr_arr{
        all_docs[i-1].1 += 1;
    }
    "{'msg':'success'}".to_string()
}

async fn handle_request(Path(query): Path<String>, state :Arc<(regex::Regex, Stemmer, std::collections::HashSet<String>, Mutex<Vec<(Record, u32)>>)>) -> String{
    // let (re, en_stemmer, stops, all_docs) = *state;
    let time = std::time::Instant::now();
    let docs = match get_docs(&query, &state.0, &state.1, &state.2, &state.3){
        Ok(doc_ids) => doc_ids,
        Err(err) => {return format!("{{\"time\":0, \"query\":\"{query}\", \"results\":[]}}");}
    };
    // println!("{:#?}", docs);
    let elapsed = time.elapsed().as_millis();
    let response = ApiResponse{
        time: elapsed,
        query: query,
        results: docs
    };
    let response_json = serde_json::to_string(&response).unwrap();
    return response_json;
}

fn get_docs(query :&String, re :&regex::Regex, en_stemmer :&Stemmer, stops :&std::collections::HashSet<String>, all_docs :&Mutex<Vec<(Record,u32)>>) -> Result<Vec<(f64, Record)>, Box<dyn std::error::Error>>{
    let tokens = tokenize(&query);
    let mut prev_posting_list: Vec<usize> = Vec::new();
    for &token in &tokens {
        let word = en_stemmer.stem(token).to_lowercase();

        for (_, [word]) in re.captures_iter(&word).map(|c| c.extract()) {
            if stops.contains(word) {
                continue;
            }
            let pl = match get_posting_list(word){
                Ok(pl) => pl,
                Err(err) => {println!("{word}");return Err(err);}
            };
            if pl.len() == 0 {
                continue;
            }
            if prev_posting_list.len() == 0 {
                prev_posting_list = pl;
            } else {
                prev_posting_list = merge_posting_lists(prev_posting_list, pl);
            }
        }
    }
    
    let mut i=0;
    let mut docs = Arc::new(Mutex::new(Vec::<(f64, &Record)>::new()));
    let ad = all_docs.lock().unwrap();
    prev_posting_list.par_iter().for_each(|i| { 
        let record = &ad[i-1].0;
        let mut tfs :HashMap<String, usize> = HashMap::new();
        for &token in &tokens{
            tfs.entry(token.to_string()).and_modify(|mana| *mana += 1).or_insert(1);
        }
        let mut tfs2 :HashMap<String, usize> = HashMap::new();
        for &token in &tokenize(&record.article){
            let word = en_stemmer.stem(token).to_lowercase();
            for (_, [word]) in re.captures_iter(&word).map(|c| c.extract()){
                tfs2.entry(word.to_string()).and_modify(|mana| *mana += 1).or_insert(1);
            }
        }
        let mut weight = 0.0;
        let mut cosine = 0.0;
        for (tkn,w) in tfs.into_iter(){
            weight += w as f64;
            let w2 = match tfs2.get(&tkn){
                Some(w2)=>w2,
                None =>{
                    // println!("{tkn}");
                    &0
                }

            };
            cosine += (w*w2) as f64
        }
        weight = weight.sqrt();
        cosine/=weight;
        cosine/=record.weight;
        // println!("{cosine}, {}",ad[i-1].1);
        cosine += 1.0/(1.0+(-(ad[i-1].1 as f64)).exp());
        let mut docsu = docs.lock().unwrap();
        docsu.push((cosine, &record));
    });
    let mut docs = docs.lock().unwrap().clone();
    docs.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    let num_docs = if 20>docs.len() {docs.len()} else {20};
    let mut rel_docs :Vec<(f64, Record)> = Vec::new();
    for i in 0..num_docs{
        rel_docs.push((docs[i].0, docs[i].1.clone()));
    }
    return Ok(rel_docs);
}

fn get_posting_list(word: &str) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let mut posting_path: String = String::from("../data/i_index/positional");
    for ch in word.chars() {
        posting_path.push_str(&format!("/ch_{ch}"));
    }
    posting_path.push_str("/posting.csv");
    // println!("{posting_path}");
    let file = match std::fs::File::open(&posting_path){
        Ok(file) => file,
        Err(err) => {println!("{err}, {posting_path}");return Err(Box::new(err))}
    };
    let file_reader = std::io::BufReader::new(file);
    let mut reader = csv::ReaderBuilder::new().has_headers(false).from_reader(file_reader);
    let mut pl: Vec<usize> = Vec::new();
    // let mut posl: Vec<usize> = Vec::new();
    let mut prev_doc_id = 0;
    for result in reader.deserialize() {
        let pentry: PositionalEntry = result.unwrap();
        // posl.push(pentry.position);
        if prev_doc_id == 0 {
            prev_doc_id = pentry.doc_id;
        }
        if prev_doc_id != pentry.doc_id {
            pl.push(prev_doc_id);
            // posl = Vec::new();
            prev_doc_id = pentry.doc_id;
        }
        // println!("{:?}", pentry);
    }
    return Ok(pl);
}

fn merge_posting_lists(pl1: Vec<usize>, pl2: Vec<usize>) -> Vec<usize> {
    let mut pl: Vec<usize> = Vec::new();
    let l1 = pl1.len();
    let l2 = pl2.len();
    let mut i = 0;
    let mut j = 0;
    while i < l1 && j < l2 {
        if pl1[i] == pl2[j] {
            pl.push(pl1[i]);
            i += 1;
            j += 1;
        } else if pl1[i] > pl2[j] {
            j += 1;
        } else {
            i += 1;
        }
    }
    return pl;
}
