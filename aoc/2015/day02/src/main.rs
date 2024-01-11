use std::env;
use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;

use day02::solve;
use day02::Solution;

struct Config {
    file_path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args)?;

    let f = fs::File::open(config.file_path)?;
    let f = BufReader::new(f);

    let mut total = Solution {
        paper: 0,
        ribbon: 0,
    };

    for line in f.lines() {
        let solution = solve(&line.unwrap());
        total.paper += solution.paper;
        total.ribbon += solution.ribbon;
    }

    println!("Paper: {}\nRibbon: {}", total.paper, total.ribbon);
    Ok(())
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }
        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}
