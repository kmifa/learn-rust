use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("引数の数が足りません");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    // output.txtを作成する
    let mut file = match File::create("output.txt") {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };

    for line in search(&config.query, &contents) {
        println!("{}", line);
        // 改行コードを付与して書き込む
        let line_with_newline = format!("{}\n", line);
        file.write_all(line_with_newline.as_bytes())?;
    }

    // このように()を使うことは、 runを副作用のためだけに呼び出していると示唆する慣習的な方法です; 必要な値は返しません。
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
}
