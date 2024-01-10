use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::
}
