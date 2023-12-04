use std::env;
use std::fs;
use std::process::exit;

#[derive(Debug)]
struct Card{
    // id : u32,
    card_numbers: Vec<u32>,
    winning_numbers: Vec<u32>,
}

fn parse_content(str: &str) -> Vec<Card> {
    // str.len() -1 eliminates last \n
    let mut cards = Vec::new();
    let lines: Vec<&str> = str.split("\n").collect();
    for line in lines {
        if line.is_empty(){continue;}
        let card_numbers : Vec<&str> = line.split(":").collect();
        // let id  = card_numbers[0][5..].trim().parse::<u32>().unwrap();
        let numbers: Vec<&str>=  card_numbers[1].split("|").map(|x| x.trim()).collect();
        let card_numbers: Vec<u32> = numbers[0].split(" ").filter(|x| !x.is_empty()).map(|x| x.parse::<u32>().unwrap()).collect();
        let winning_numbers: Vec<u32> = numbers[1].split(" ").filter(|x| !x.is_empty()).map(|x| x.parse::<u32>().unwrap()).collect();
        // let card = Card{id,card_numbers,winning_numbers};
        let card = Card{card_numbers,winning_numbers};
        // println!("{:?}",card);
        cards.push(card);
    }
    cards
}

impl Card{
    fn count_matches(&self)->u32{
        let mut count = 0;
        for number in &self.card_numbers{
            if let Some(_) = self.winning_numbers.iter().find(|x| *x == number){
                count +=1;
            }
        }
        count
    }
    fn count_points(&self)->u32{
        let count = self.count_matches();
        if count == 0 {
            return  0;
        }
        2_u32.pow(count-1)

    }
}

// use std::process::Command;

fn count_cards(cards:&Vec<Card>, begin_index:usize, end_index:usize)->u32{
    // println!("{:?}",&cards[begin_index..end_index]);
    // println!("begin_index : {}, end_index {}",begin_index,end_index);
    let mut i = begin_index;
    let mut count = 0;
    while i < end_index{
        let matches= cards[i].count_matches();
        // println!("index = {} with {} matches ",i+1, matches);
        if matches> 1 {
            // check what happens here
            let new_begin_index = i+1;
            let new_end_index = new_begin_index + matches as usize;
            // let mut child = Command::new("sleep").arg("1").spawn().unwrap();
            // let _result = child.wait().unwrap();
            // println!("count {}, begin_index : {}, end_index{}",counts,new_begin_index,new_end_index);
            // panic!("here");
            let result = count_cards(cards, new_begin_index, new_end_index);
            count+=result;
        }
        count+=1;
        i+= 1;
    }
    count+1
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("Filename not provided");
        exit(0);
    }
    let filename: &str = args[1].as_str();
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let cards = parse_content(&content);
    let mut sum = 0;
    for card in &cards{
        let count = card.count_points();
        sum+=count;
    }

    println!("Sol 1: {}",sum);
    let n_cards = count_cards(&cards, 0, cards.len());
    println!("Sol 2 : {}",n_cards);

}
