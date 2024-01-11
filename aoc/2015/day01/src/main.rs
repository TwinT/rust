use std::env;
use std::error::Error;
use std::fs;

use day01::solve;

struct Config {
    file_path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args)?;

    let contents = fs::read_to_string(config.file_path)?;

    let solution = solve(&format!("{contents}"));
    println!("Floor: {}\nPosition: {}", solution.floor, solution.position);
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
