use rsnltk::api::natural::tokenize;
use rust_stemmers::{Algorithm, Stemmer};
use std::collections::HashMap;
use std::fs::{create_dir_all, OpenOptions};
use std::{
    fs::File,
    io::{BufReader, Write},
};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Record {
    date: String,
    year: u16,
    month: f32,
    day: u16,
    author: String,
    title: String,
    article: String,
    url: String,
    section: String,
    publication: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct PositionalEntry {
    doc_id: usize,
    position: usize,
}

fn main() {
    // for i in 97..123 {
    //     let _ = std::fs::create_dir_all(format!("./index/posting/part_{}", i as u8 as char));
    // }

    let mut word_buffer: HashMap<String, Vec<PositionalEntry>> = HashMap::new();
    let en_stemmer = Stemmer::create(Algorithm::English);

    let args: Vec<String> = std::env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let file_reader = BufReader::new(file);
    let mut reader = csv::Reader::from_reader(file_reader);

    let mut i = 0;
    let re = regex::Regex::new(r"([a-z]+)").unwrap();
    let now = std::time::Instant::now();
    for result in reader.deserialize() {
        if i % 1000 == 0 { 
            std::io::stdout().flush().unwrap();
            write_pl_mul(&word_buffer);
            word_buffer = HashMap::new();
            let elapsed = now.elapsed().as_secs();
            print!("\r{:10?}  {} minutes remaining", i, if i!=0 {(elapsed as f64)*(110000.0 - i as f64)/(i as f64*60.0)} else {0.0});
        }
        i += 1;
        let record: Record = result.unwrap();
        let tokens = tokenize(&record.article);
        let mut j = 0;
        for token in tokens {
            j += 1;
            let word = en_stemmer.stem(token).to_lowercase();
            for (_, [word]) in re.captures_iter(&word).map(|c| c.extract()) {
                word_buffer
                    .entry(word.to_string())
                    .and_modify(|v| {
                        v.push(PositionalEntry {
                            doc_id: i,
                            position: j,
                        })
                    })
                    .or_insert(vec![PositionalEntry {
                        doc_id: i,
                        position: j,
                    }]);
            }
        }

    }
}

fn write_pl(word: &String, pls: &Vec<PositionalEntry>) {
    let mut posting_path = String::from("./i_index/positional");
    for ch in word.chars() {
        posting_path.push_str(&format!("/ch_{ch}"));
    }
    let _ = create_dir_all(&posting_path);

    match OpenOptions::new()
        .append(true)
        .create(true)
        .open(format!("{posting_path}/posting.csv"))
    {
        Ok(file) => {
            let mut csv_writer = csv::WriterBuilder::new()
                .has_headers(false)
                .from_writer(file);
            for pl in pls {
                let _ = csv_writer.serialize(pl);
            }
        }
        Err(_) => {
            println!("{:?}", word);
        }
    }
}

fn write_pl_mul(word_buffer: &HashMap<String, Vec<PositionalEntry>>){
    for (word, pls) in word_buffer.into_iter(){
        write_pl(word, pls)
    }
}