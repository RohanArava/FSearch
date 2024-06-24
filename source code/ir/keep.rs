use std::{
    fs::File,
    io::{BufReader, Write},
    collections::HashMap
};

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

fn main() {
    let args :Vec<String> = std::env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let file_reader = BufReader::new(file);
    let mut reader = csv::Reader::from_reader(file_reader);
    // for result in reader.deserialize(){
    //     let record :Record = result.unwrap();
    //     println!("{:#?}", record);
    //     break
    // }
    let mut sections :HashMap<String, usize> = HashMap::new();
    let mut i = 0;
    for result in reader.deserialize(){
        i+=1;
        let record :Record = result.unwrap();
        if record.article!=""{
            sections.entry(record.section.clone()).and_modify(|mana| *mana += 1).or_insert(1);
        }
        if i%10000==0{
            print!("\r{i} articles");
            std::io::stdout().flush().unwrap();
        }
    }
    for section in sections.keys(){
        if sections.get(section).unwrap() > &100000 {
            let max = *sections.get(section).unwrap();
            let max_sec = section;
            println!("max:{}, section:{}", max, max_sec);
        }
    }
    
}
