use std::{
    fs::File,
    io::{BufReader, Write},
    collections::HashMap
};
use rsnltk::api::natural::tokenize;

#[derive(Debug, serde::Deserialize)]
struct Record{
    date :String,
    year: u16,
    month: f32,
    day: u16,
    author: String,
    title: String,
    article: String,
    url: String,
    section: String,
    publication: String
}

#[derive(Debug, serde::Serialize)]
struct RecordNew{
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

fn main() {
    let args :Vec<String> = std::env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let file_reader = BufReader::new(file);
    let mut reader = csv::Reader::from_reader(file_reader);
    let mut i = 0;
    let nf = std::fs::OpenOptions::new().append(true).create(true).open("../data/world_new.csv").unwrap();
    let mut writer = csv::WriterBuilder::new().has_headers(false).from_writer(nf);
    for result in reader.deserialize(){
        i+=1;
        let record :Record = result.unwrap();
        let mut tfs :HashMap<String, usize> = HashMap::new();
        for token in tokenize(&record.article){
            tfs.entry(token.to_string()).and_modify(|mana| *mana += 1).or_insert(1);
        }
        let mut weight = 0.0;
        for (_,w) in tfs.into_iter(){
            weight += w as f64;
        }
        weight = weight.sqrt();
        let record_new :RecordNew = RecordNew{
            doc_id: i,
            date: record.date,
            year: record.year,
            month: record.month,
            day: record.day,
            author: record.author,
            title: record.title,
            article: record.article,
            url: record.url,
            section: record.section,
            publication: record.publication,
            weight:weight
        };
        writer.serialize(record_new);
        if i%10000==0{
            print!("\r{i} articles");
            std::io::stdout().flush().unwrap();
        }

    }
    
}
