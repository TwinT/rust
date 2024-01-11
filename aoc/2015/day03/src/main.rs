use std::env;
use std::error::Error;
use std::fs;

use day01::solve;

struct Config {
    file_path: String,
    santas: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args)?;

    let contents = fs::read_to_string(config.file_path)?;

    let houses = solve(&format!("{contents}"));
    println!("Houses: {}", houses);
    Ok(())
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let file_path = args[1].clone();
        let santas: u32 = args[2].parse().unwrap_or_else(|err| {
            return Err("Parsing error: {err}");
        });

        Ok(Config { file_path, santas })
    }
}
