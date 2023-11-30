use std::env;
use std::fs;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("Filename not provided");
        exit(0);
    }
    let filename: &str = args[1].as_str();
    let content = fs::read_to_string(filename).expect("Error reading the file");
    println!("content:{}", content);
}
