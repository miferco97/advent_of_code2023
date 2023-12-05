use std::env;
use std::fs;
use std::process::exit;

struct Rule {
    destination_start: u64,
    source_start: u64,
    range: u64,
}

impl Rule {
    fn apply(&self, number: u64) -> Option<u64> {
        if number < self.source_start || number > self.source_start + self.range - 1 {
            return None;
        }
        let value: u64 = number - self.source_start + self.destination_start;
        Some(value)
    }
}

struct Category {
    rules: Vec<Rule>,
}

impl Category {
    fn map(&self, number: u64) -> u64 {
        for rule in &self.rules {
            if let Some(value) = rule.apply(number) {
                return value;
            }
        }
        number
    }
}

fn parse_content(str: &str) -> (Vec<u64>, Vec<Category>) {
    let mut categories: Vec<Category> = Vec::new();
    let groups: Vec<&str> = str.split("\n\n").collect();
    let (seeds_str, cat_str) = groups.split_first().unwrap();

    let seeds = seeds_str.split(":").collect::<Vec<&str>>()[1]
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    for cat_content in cat_str {
        let mut rules: Vec<Rule> = Vec::new();
        let lines: Vec<&str> = cat_content.split("\n").filter(|x| !x.is_empty()).collect();
        let (_cat_names, cat_rules) = lines.split_first().unwrap();
        for rule_str in cat_rules {
            let values: Vec<u64> = rule_str
                .split_whitespace()
                .map(|x| x.trim().parse::<u64>().unwrap())
                .collect();
            rules.push(Rule {
                destination_start: values[0],
                source_start: values[1],
                range: values[2],
            })
        }
        categories.push(Category { rules })
    }
    (seeds, categories)
}

fn compute_distance(seed: u64, categories: &Vec<Category>) -> u64 {
    let mut value = seed;
    for cat in categories {
        value = cat.map(value);
    }
    value
}

fn compute_expanded_seeds(seeds: &Vec<u64>) -> Vec<u64> {
    let mut i = 0;
    let mut expanded_seeds: Vec<u64> = Vec::new();

    while i < seeds.len() {
        for j in seeds[i]..seeds[i] + seeds[i + 1] {
            expanded_seeds.push(j);
        }
        i += 2;
    }

    expanded_seeds
}

fn compute_min_dist(seeds: &Vec<u64>, categories: &Vec<Category>) -> u64 {
    let mut min = u64::MAX;
    for seed in seeds {
        let location = compute_distance(*seed, &categories);
        if location < min {
            min = location;
        }
    }
    min
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("Filename not provided");
        exit(0);
    }
    let filename: &str = args[1].as_str();
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let (seeds, categories) = parse_content(&content);

    let min1 = compute_min_dist(&seeds, &categories);
    println!("Min location : {min1}");

    let expanded_seeds = compute_expanded_seeds(&seeds);
    let min2 = compute_min_dist(&expanded_seeds, &categories);
    println!("Min location expanded : {min2}");
}
