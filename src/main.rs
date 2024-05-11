#![allow(dead_code, unused_variables)]
use std::env;
use std::process;
pub mod fileops;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
        });
    let fileconts = fileops::read_file(&config.file_path).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}"); 
        process::exit(1);
    });
    let matches = fileops::search_str_rand(fileconts, &config.query);
    for x in &matches {
        println!("Found at line {x}");
    }
    process::exit(0);
}

struct Config {
    query: String,
    file_path: String,
}
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })   
    }
}