use std::env;
use std::error::Error;
use std::fs;

struct Solution {
    floor: i32,
    position: usize,
}

struct Config {
    file_path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args)?;

    let contents = fs::read_to_string(config.file_path)?;

    let solution: Solution = solve(&format!("{contents}"));
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

// Logic

fn solve(input: &str) -> Solution {
    let mut floor: i32 = 0;
    let mut position: usize = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        if position == 0 && floor == -1 {
            position = i + 1;
        }
    }
    Solution { floor, position }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(solve("(())").floor, 0);
        assert_eq!(solve("()()").floor, 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve("(((").floor, 3);
        assert_eq!(solve("(()(()(").floor, 3);
        assert_eq!(solve("))(((((").floor, 3);
    }

    #[test]
    fn test_minus_1() {
        assert_eq!(solve("())").floor, -1);
        assert_eq!(solve("))(").floor, -1);
    }

    #[test]
    fn test_minus_3() {
        assert_eq!(solve(")))").floor, -3);
        assert_eq!(solve(")())())").floor, -3);
    }
}
