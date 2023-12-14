use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::exit;

fn encode_group(str: &str, groups: &Vec<usize>) -> String {
    let mut string = String::from(str);
    for elem in groups {
        string.push_str(elem.to_string().as_str());
        string.push_str(".");
    }
    string
}

#[derive(Debug)]
struct Spring {
    record: String,
    groups: Vec<usize>,
}

impl Spring {
    fn reduce(&mut self) {
        let mut reduced = String::new();
        let mut last_elem = '.';
        for elem in self.record.chars() {
            if (elem == '.') && (last_elem == '.') {
                continue;
            }
            reduced.push(elem);
            last_elem = elem;
        }
    }

    fn unfold(&self) -> Spring {
        let mut record = String::from(self.record.as_str());
        let mut groups = Vec::new();
        for _ in 0..4 {
            record.push_str("?");
            record.push_str(self.record.as_str());
        }
        for _ in 0..5 {
            for elem in &self.groups {
                groups.push(*elem);
            }
        }
        Spring { record, groups }
    }
    fn encode(&self) -> String {
        encode_group(&self.record, &self.groups)
    }
}

fn count_groups_possibilities_brute_force(
    record: &str,
    groups: &Vec<usize>,
    memo_dict: &mut HashMap<String, u64>,
) -> u64 {
    let key = encode_group(record, groups);
    if memo_dict.contains_key(&key) {
        return *memo_dict.get(&key).unwrap();
    }

    // println!("spring : {:?}", orig);
    let string = generate_string_from_groups(groups);
    // println!("string = {string}");
    let re = Regex::new(&string).unwrap();
    let mut expresion: Vec<char> = record.chars().collect();
    let mut question_positions = Vec::new();
    let mut n_matches = 0;
    for (index, elem) in record.chars().enumerate() {
        if elem == '?' {
            question_positions.push(index);
        }
    }
    let n_combs = 1_u64 << question_positions.len();
    // println!("n_combs {}",n_combs);
    for i in 0..n_combs {
        for (j, index) in question_positions.iter().enumerate() {
            let new_i = i >> j;
            if new_i % 2 == 0 {
                expresion[*index] = '#';
            } else {
                expresion[*index] = '.';
            }
        }
        let str_converted: String = expresion.iter().collect();
        if re.is_match(&str_converted) {
            n_matches += 1;
        }
    }
    memo_dict.insert(key, n_matches);

    n_matches
}

fn find_combs(
    str_: &str,
    groups: &Vec<usize>,
    memo_dict: &mut HashMap<String, u64>,
    level: u64,
) -> u64 {
    // let tabs: String = vec!['\t'; level as usize].iter().collect();
    // println!("{tabs}find combs: {}, groups {:?}", str_, groups);

    if groups.is_empty() {
        if(str_.len() == 0 || str_.find(|x| x == '#').is_none()) {
        return 1;
        }
        else {
            return 0;
        }
    }

    if !groups.is_empty() && str_.len() == 0 {
        return 0;
    }

    let n_items = groups[0];
    if str_.len() < n_items {
        return 0;
    }

    let key = encode_group(str_, groups);
    if memo_dict.contains_key(&key) {
        return *memo_dict.get(&key).unwrap();
    }

    // if first is . or ?
    // go for the next substring
    let mut res = 0;
    let next_char = str_.chars().next().unwrap();
    if next_char == '.' || next_char == '?' {
        res += find_combs(&str_[1..], groups, memo_dict, level + 1);
    }

    // find groups of #....

    if next_char == '#' || next_char == '?' {
        let mut new_str = String::from('#');
        new_str.push_str(&str_[1..]);
        let str_ = new_str;

        // check if all are #
        if str_[0..n_items].find(|x| x == '.').is_none() {
            if str_.len() == n_items {
                // nothing no increment
                let new_groups = &groups[1..].iter().map(|x| *x).collect();
                let new_str = &str_[n_items..];
                res += find_combs(new_str, &new_groups, memo_dict, level + 1)
            } else if &str_[n_items..n_items + 1] != "#" {
                let new_groups = &groups[1..].iter().map(|x| *x).collect();
                let new_str = &str_[n_items + 1..];
                res += find_combs(new_str, &new_groups, memo_dict, level + 1)
            }
        }
    }

    memo_dict.insert(key, res);
    res
}

fn generate_string_from_groups(groups: &Vec<usize>) -> String {
    let mut string = String::from(r"^\.*");
    for (i, elem) in groups.iter().enumerate() {
        for _ in 0..*elem {
            string.push('#');
        }
        if i < groups.len() - 1 {
            string.push_str(r"\.+");
        }
    }
    string.push_str(r"\.*$");
    string
}

fn parse_content(str: &str) -> Vec<Spring> {
    let mut out = Vec::new();
    let content: Vec<&str> = str.split("\n").filter(|x| !x.is_empty()).collect();
    for line in content {
        let (record_str, groups_str) = line.split_once(" ").unwrap();
        let record = record_str.chars().collect();
        let groups = groups_str
            .split(",")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let mut spring = Spring { record, groups };
        spring.reduce();
        out.push(spring);
    }
    out
}

fn count_matches(springs: &Vec<Spring>, memo_dict: &mut HashMap<String, u64>) -> u64 {
    let mut total_combs = 0;
    for (i, spring) in springs.iter().enumerate() {
        let n_matches = find_combs(&spring.record, &spring.groups, memo_dict, 0);
        total_combs += n_matches;
    }
    total_combs
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("Filename not provided");
        exit(0);
    }
    let mut map = HashMap::new();

    let filename: &str = args[1].as_str();
    let content = fs::read_to_string(filename).expect("Error reading the file");
    let springs = parse_content(&content);
    let part1 = count_matches(&springs,&mut map);
    println!("PART 1: {part1}");

    // let test_spring = Spring {
    //     record: String::from("?###????????"),
    //     groups: vec![3, 2, 1],
    // };
    // let number = find_combs(&test_spring.record, &test_spring.groups, &mut map, 0);
    // println!("number {number}");
    // println!("number2 {}", number.pow(5));

    let new_springs = springs.iter().map(|x| x.unfold()).collect();
    let part2 = count_matches(&new_springs,&mut map);
    println!("PART 2: {part2}");
}
