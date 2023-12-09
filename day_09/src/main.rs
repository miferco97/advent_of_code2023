use std::env;
use std::fs;
use std::process::exit;

#[derive(Clone)]
struct Sequence {
    seq: Vec<i64>,
}

fn compute_next_seq(seq: &Vec<i64>) -> Vec<i64> {
    let mut next_seq = vec![0; seq.len() - 1];
    for i in 0..(seq.len() - 1) {
        let next_item = seq[i + 1] - seq[i];
        next_seq[i] = next_item;
    }
    next_seq
}

fn compute_minimun(seq: &Sequence, append_right: bool) -> i64 {
    let mut sequences = vec![seq.seq.clone()];
    let mut next_seq = seq.seq.clone();
    loop {
        next_seq = compute_next_seq(&next_seq);
        sequences.push(next_seq.clone());
        if next_seq.iter().fold(0_i64, |acc, x| x.abs() + acc) == 0 {
            break;
        }
    }

    let mut new_value = 0;
    if append_right{
        for i in 1..(sequences.len() + 1) {
            let index = sequences.len() - i;
            new_value += sequences[index].last().unwrap();
        }
    } else {
        for i in 1..(sequences.len() + 1) {
            let index = sequences.len() - i;
            new_value = sequences[index].first().unwrap()- new_value;
        }

    }
    new_value
}

fn parse_content(str: &str) -> Vec<Sequence> {
    let mut sequences: Vec<Sequence> = Vec::new();
    let content: Vec<&str> = str.split("\n").filter(|x| !x.is_empty()).collect();
    for line in content {
        let seq = line
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        sequences.push(Sequence { seq });
    }
    sequences
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("Filename not provided");
        exit(0);
    }
    let filename: &str = args[1].as_str();
    let content = fs::read_to_string(filename).expect("Error reading the file");
    let sequences = parse_content(&content);
    let predicted = sequences.iter().fold(0, |acc, x| acc + compute_minimun(&x,true));
    println!("PART 1 : {predicted}");
    let predicted = sequences.iter().fold(0, |acc, x| acc + compute_minimun(&x,false));
    println!("PART 2 : {predicted}");
}
