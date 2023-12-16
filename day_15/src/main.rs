use std::env;
use std::fs;
use std::process::exit;

fn hash_code(str: &str) -> usize {
    let ascii_vec: Vec<u8> = str.chars().map(|x| x as u8).collect();
    ascii_vec
        .iter()
        .fold(0, |acc, x| ((acc + *x as usize) * 17) % 256)
}

#[derive(Debug, Clone)]
struct Code {
    code: String,
    label: String,
    operation: char,
    focal_length: usize,
}

impl Code {
    fn new(str: &str) -> Code {
        let index = str.find(|c| c == '=' || c == '-').unwrap();
        Code {
            code: str.to_string(),
            label: str[0..index].to_string(),
            operation: str[index..index + 1].chars().next().unwrap(),
            focal_length: match str[index + 1..].parse() {
                Ok(value) => value,
                Err(_) => 0,
            },
        }
    }
}

fn print_boxes(boxes: &Vec<Vec<Code>>) {
    for (i, box_) in boxes.iter().enumerate() {
        if box_.is_empty() {
            continue;
        }
        let mut str_ = String::new();
        for code in box_ {
            str_.push_str(std::format!("[{} {}]", code.label, code.focal_length).as_str());
        }
        println!("Box {i}: {str_}");
    }
}

fn follow_codes(codes: &Vec<Code>) -> usize {
    let mut boxes: Vec<Vec<Code>> = vec![Vec::new(); 256];
    for code in codes {
        let hash = hash_code(&code.label);
        // println!("{:?}, hash {}",code.code, hash);
        if code.operation == '=' {
            let box_index = boxes[hash].iter().position(|x| x.label == code.label);
            if let Some(index) = box_index {
                boxes[hash][index].focal_length = code.focal_length;
                continue;
            }
            boxes[hash].push(code.clone());
        } else if code.operation == '-' {
            for i in 0..boxes[hash].len() {
                if boxes[hash][i].code == code.code {
                    break;
                }

                let box_index = boxes[hash].iter().position(|x| x.label == code.label);
                if let Some(index) = box_index {
                    boxes[hash].remove(index);
                    break;
                }
            }
        }

        // println!("After {:?}", code.code);
        // print_boxes(&boxes);
        // println!();
    }

    let mut value = 0;
    for (i_boxes, box_) in boxes.iter().enumerate() {
        for (i_code, code) in box_.iter().enumerate() {
            let box_value = i_boxes + 1;
            let code_value = i_code + 1;
            let new_value = box_value * code_value * code.focal_length;
            // println!(
            //     "{}: {box_value} * {code_value} * {}",
            //     code.label, code.focal_length
            // );
            value += new_value;
        }
    }
    value
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("Filename not provided");
        exit(0);
    }

    let filename: &str = args[1].as_str();
    let content = fs::read_to_string(filename).expect("Error reading the file");
    let vec_strings: Vec<&str> = content.split("\n").next().unwrap().split(",").collect();
    assert_eq!(hash_code("HASH"), 52);
    let part1 = vec_strings.iter().fold(0, |acc, x| acc + hash_code(x));
    println!("Part 1: {}", part1);

    let vec_codes: Vec<Code> = vec_strings.iter().map(|x| Code::new(x)).collect();
    let value = follow_codes(&vec_codes);
    println!("Part 2: {}", value);
}
