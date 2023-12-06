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

fn parse_content(str: &str, collapse_whitespaces:bool) -> Vec<Race> {
    let mut out :Vec<Race> = Vec::new();
    let mut content = String::from(str);
    if collapse_whitespaces {
        content = str.replace(" ", "");
    }
    let content: Vec<&str> = content.split("\n").collect();
    let time = parse_numbers(content[0]);
    let dist= parse_numbers(content[1]);
    for i in 0..time.len(){
        out.push(Race { time: time[i].into(), dist:dist[i].into() })
    }
    out
}


impl Race{
    fn compute_stop_times(&self)->(f64,f64){
        let t = &self.time;
        let d = &self.dist;
        let s1 = (t - (t.powi(2)-4.0*d).sqrt())/(2.0);
        let mut s2 = (t + (t.powi(2)-4.0*d).sqrt())/(2.0);
        // consider when you read to d but don't win
        if s2 == s2.floor() {
            s2-=1.0;
        }
        (s1.floor()+1.0,s2.floor())
    }
    fn compute_n_combs(&self)->f64{
        let (min,max) = self.compute_stop_times();
        max-min+1.0
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
    let races = parse_content(&content,false);
    println!("Races : {:?}",races);

    let result = races.iter().fold(1.0,|acc,x| acc*x.compute_n_combs());
    println!("RES 1 : {result}");

    let races = parse_content(&content,true);
    let result  = races[0].compute_n_combs();
    println!("RES 2 : {result}");
}
