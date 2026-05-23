use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;    
use clap::{App, Arg};   

fn main() {
  let args = App::new("grep-lite")
    .version("0.1.0")
    .about("searches for patterns")
    .arg(Arg::with_name("pattern")
      .help("the pattern to search for")
      .required(true)
      .takes_value(true))
    .arg(Arg::with_name("input")
      .help("File to search")
      .takes_value(true)
      .required(true))
    .get_matches();

  let pattern = args.value_of("pattern").unwrap();    
  let re = Regex::new(pattern).unwrap();    

  let input_file = args.value_of("input").unwrap();
  let file = File::open(input_file).unwrap();
  let reader = BufReader::new(file);

  for line_ in reader.lines() {
    let line = line_.unwrap();
    match re.find(&line){
        Some(_) => println!("{}", line),    // <4>
        None => (),    // <5>
    }
  }
}