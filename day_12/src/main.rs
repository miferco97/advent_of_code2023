use std::env;
use std::fs;
use std::process::exit;

#[derive(Debug)]
struct Spring {
    record: String,
    groups: Vec<u32>,
}

impl Spring{
    fn reduce(&mut self){
        let mut reduced =String::new();
        let mut last_elem = '.';
        for elem in self.record.chars(){
            if (elem == '.') && (last_elem == '.'){
                continue;
            }
            reduced.push(elem);
            last_elem=elem;
        }
        self.record = reduced;
    }
}

fn check_valid(str:&str, pat:&Vec<usize>)->bool{
    false
}

fn count_groups_possibilities(orig:&Spring)->Vec<Spring>{
    let groups:Vec<&str> = orig.record.split('.').filter(|x| !x.is_empty()).collect();
    println!("\tgroups {:?}" ,groups);
    let groups_len : Vec<usize> = groups.iter().map(|x| x.len()).collect();
    println!("\tgroups len {:?}" ,groups_len);

    Vec::new()
}


fn parse_content(str: &str) -> Vec<Spring> {
    let mut out = Vec::new();
    let content: Vec<&str> = str.split("\n").filter(|x| !x.is_empty()).collect();
    for line in content {
        let (record_str, groups_str) = line.split_once(" ").unwrap();
        let record = record_str.chars().collect();
        let groups = groups_str.split(",").filter(|x| !x.is_empty()).map(|x| x.parse::<u32>().unwrap()).collect();
        let mut spring = Spring{record,groups};
        spring.reduce();
        out.push(spring);
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("Filename not provided");
        exit(0);
    }
    let filename: &str = args[1].as_str();
    let content = fs::read_to_string(filename).expect("Error reading the file");
    let springs= parse_content(&content);
    for spring in &springs{
        println!("{:?}",spring);
        count_groups_possibilities(&spring);
    }
}
