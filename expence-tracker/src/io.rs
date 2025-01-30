use std::{fs::File, path::Path};

use crate::expence::Expence;
use crate::limit::Limit;

const EXPENCE_FILE: &str = "expence.json";
const LIMIT_FILE: &str = "limit.json";

pub fn load_expence_list() -> Vec<Expence> {
    if Path::new(EXPENCE_FILE).exists() {
        let file = File::open(EXPENCE_FILE).unwrap();
        let reader = std::io::BufReader::new(file);
        let des: Result<Vec<Expence>, serde_json::Error> = serde_json::from_reader(reader);

        des.unwrap_or_default()
    } else {
        Vec::new()
    }
}

pub fn save_expence(tasks: &Vec<Expence>) {
    let file = File::create(EXPENCE_FILE).unwrap();
    serde_json::to_writer_pretty(file, tasks).unwrap();
}

pub fn save_limit(limits: &Vec<Limit>) {
    let file = File::create(LIMIT_FILE).unwrap();
    serde_json::to_writer_pretty(file, limits).unwrap();
}

pub fn load_limit_list() -> Vec<Limit> {
    if Path::new(LIMIT_FILE).exists() {
        let file = File::open(LIMIT_FILE).unwrap();
        let reader = std::io::BufReader::new(file);
        let des: Result<Vec<Limit>, serde_json::Error> = serde_json::from_reader(reader);

        des.unwrap_or_default()
    } else {
        Vec::new()
    }
}

pub fn download_csv() {
    let expences = load_expence_list();
    let mut wtr = csv::Writer::from_path("expences.csv").unwrap();
    wtr.write_record(["Date", "Amount", "Category", "Description"])
        .unwrap();
    for expence in expences {
        wtr.write_record(&[
            expence.date.to_string(),
            expence.amount.to_string(),
            expence.category.to_string(),
            expence.description.to_string(),
        ])
        .unwrap();
    }
    wtr.flush().unwrap();
}
