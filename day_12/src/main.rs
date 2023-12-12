use std::env;
use std::fs;
use std::process::exit;
use regex::Regex;

#[derive(Debug)]
struct Spring {
    record: String,
    groups: Vec<u64>,
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
    }
    // fn filter(&mut self){
    //     let splits: Vec<&str> = self.record.split(".").collect();
    //     for i in 0..splits.len(){}

       




    // }

    fn unfold(&self) -> Spring{
        let mut record = String::from(self.record.as_str());
        let mut groups = Vec::new();
        for _ in 0..4{
            record.push_str("?");
            record.push_str(self.record.as_str());
        }
        for _ in 0..5{
            for elem in &self.groups{
                groups.push(*elem);
            }
        }
        Spring{record,groups}

    }
}

// fn check_valid(str:&str, pat:&Vec<usize>)->bool{
//     false
// }

// fn count_groups_possibilities(orig:&Spring)->Vec<Spring>{
//     let groups:Vec<&str> = orig.record.split('.').filter(|x| !x.is_empty()).collect();
//     println!("\tgroups {:?}" ,groups);
//     let groups_len : Vec<usize> = groups.iter().map(|x| x.len()).collect();
//     println!("\tgroups len {:?}" ,groups_len);

//     Vec::new()
// }

fn generate_string_from_groups(groups:&Vec<u64>)->String{
    let mut string = String::from(r"^\.*");
    for (i,elem) in groups.iter().enumerate(){
        for _ in 0..*elem {
            string.push('#');
        }
        if i < groups.len()-1{
            string.push_str(r"\.+");
        }
    }
    string.push_str(r"\.*$");
    string
}

fn count_groups_possibilities_brute_force(orig:&Spring)->u64{
    println!("spring : {:?}",orig);
    let string = generate_string_from_groups(&orig.groups);
    // println!("string = {string}");
    let re = Regex::new(&string).unwrap();
    let mut expresion: Vec<char> = orig.record.chars().collect();
    let mut question_positions = Vec::new();
    let mut n_matches = 0;
    for (index, elem) in orig.record.chars().enumerate(){
        if elem == '?'{
            question_positions.push(index);
        }
    }
    let n_combs = 1_u64 << question_positions.len();
    // println!("n_combs {}",n_combs);
    for i in 0..n_combs{
        for (j, index ) in question_positions.iter().enumerate(){
            let new_i = i >> j;
            if new_i % 2== 0{
                expresion[*index] = '#';
            }
            else{
                expresion[*index] = '.';
            }

        }
        let str_converted :String = expresion.iter().collect();
        if re.is_match(&str_converted){
            // println!("expresion = {}",str_converted);
            n_matches+=1;
        }

        
    }
    println!("n_matches : {}",n_matches );
    n_matches
}




fn parse_content(str: &str) -> Vec<Spring> {
    let mut out = Vec::new();
    let content: Vec<&str> = str.split("\n").filter(|x| !x.is_empty()).collect();
    for line in content {
        let (record_str, groups_str) = line.split_once(" ").unwrap();
        let record = record_str.chars().collect();
        let groups = groups_str.split(",").filter(|x| !x.is_empty()).map(|x| x.parse::<u64>().unwrap()).collect();
        let mut spring = Spring{record,groups};
        spring.reduce();
        out.push(spring);
    }
    out
}


fn count_matches(springs:&Vec<Spring>)->u64{
    let mut total_combs = 0;
    for (i,spring) in springs.iter().enumerate(){
        if i % 100 == 0 {
            println!("{}/{}",i,springs.len());
        }
        let n_matches = count_groups_possibilities_brute_force(&spring);
        total_combs+=n_matches;
    }
    total_combs

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
    let part1= count_matches(&springs);
    println!("PART 1: {part1}");

    let test_spring = Spring{record: String::from("????????????????????????????????????????"),groups:vec![2,1,2,1,2,1,2,1,2,1]};
    let number = count_groups_possibilities_brute_force(&test_spring);
    println!("number {number}");
    println!("number2 {}", number.pow(5));

    // let new_springs = springs.iter().map(|x| x.unfold()).collect();
    // let part2= count_matches(&new_springs);
    // println!("PART 2: {part2}");

}
