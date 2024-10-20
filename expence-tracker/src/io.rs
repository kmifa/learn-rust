use std::{fs::File, path::Path};

use crate::expence::Expence;

const EXPENCE_FILE: &str = "expence.json";

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
