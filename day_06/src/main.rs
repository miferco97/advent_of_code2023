use std::env;
use std::fs;
use std::process::exit;

#[derive(Debug)]
struct Race{
    time: f64,
    dist: f64,
}


fn parse_numbers(str:&str)->Vec<f64>{
    let name_numbers:Vec<&str> =str.split(":").collect() ;
    let times : Vec<f64> = name_numbers[1].split_whitespace().filter(|x|!x.is_empty()).map(|x|x.parse::<f64>().unwrap()).collect();
    times
}

fn parse_content(str: &str) -> Vec<Race> {
    let mut out :Vec<Race> = Vec::new();
    let content: Vec<&str> = str.split("\n").collect();
    let time = parse_numbers(content[0]);
    let dist= parse_numbers(content[1]);
    for i in 0..time.len(){
        out.push(Race { time: time[i].into(), dist:dist[i].into() })
    }
    out
}

fn parse_content2(str: &str) -> Vec<Race> {
    let mut out :Vec<Race> = Vec::new();
    let content = str.replace(" ", "");
    let content: Vec<&str> = content.split("\n").collect();
    let time = parse_numbers(content[0]);
    let dist= parse_numbers(content[1]);
    for i in 0..time.len(){
        out.push(Race { time: time[i].into(), dist:dist[i].into() })
    }
    out
}

fn solve_this_eq(t:f64,d:f64)->(u128,u128){
    let s1 = (t - (t.powi(2)-4.0*d).sqrt())/(2.0);
    let s2 = (t + (t.powi(2)-4.0*d).sqrt())/(2.0);
    let s1_i = s1 as u128;
    let mut s2_i = s2 as u128;
    // consider when you read to d but don't win
    if s2 == s2.floor() {
        s2_i-=1;
    }
    
    (s1_i+1,s2_i)
}

impl Race{
    fn compute_stop_times(&self)->(u128,u128){
        solve_this_eq(self.time,self.dist)
    }
    fn compute_n_combs(&self)->u128{
        let (min,max) = self.compute_stop_times();
        max-min+1
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("Filename not provided");
        exit(0);
    }
    let filename: &str = args[1].as_str();
    let content = fs::read_to_string(filename).expect("Error reading the file");
    let races = parse_content(&content);
    println!("Races : {:?}",races);
    let mut result = 1u128;
    for race in races{
        let combinations = race.compute_n_combs();
        println!("combinations : {combinations}");
        result*=combinations;
    }
    println!("RES 1 : {result}");
    let races = parse_content2(&content);
    let mut result = 1u128;
    for race in races{
        let combinations = race.compute_n_combs();
        println!("combinations : {combinations}");
        result*=combinations;
    }
    println!("RES 2 : {result}");
}
