extern crate file_operation;

use std::env;
use std::process;

use file_operation::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        // 引数解析時に問題
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = file_operation::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
