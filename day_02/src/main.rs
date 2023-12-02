use std::env;
use std::fs;
use std::process::exit;

#[derive(Debug)]
struct Play {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    plays: Vec<Play>,
}

impl Play {
    fn new() -> Play {
        Play {
            red: 0,
            blue: 0,
            green: 0,
        }
    }
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }

    fn parse_str(&mut self, str: &str) {
        if str.is_empty() {
            return;
        }
        let dices: Vec<&str> = str.trim().split(" ").map(|x| x.trim()).collect();
        let color = dices[1].trim_end_matches(",");
        let number = dices[0].parse::<u32>().unwrap();
        // println!("color {} with {} dices", color, number);
        match color.trim() {
            "red" => {
                self.red = number;
            }
            "green" => {
                self.green = number;
            }
            "blue" => {
                self.blue = number;
            }
            _ => {
                panic!("Cant parser {}", color.trim());
            }
        }
    }
    fn is_valid(&self, red_max: u32, green_max: u32, blue_max: u32) -> bool {
        if self.red > red_max || self.green > green_max || self.blue > blue_max {
            return false;
        }
        true
    }
}

impl Game {
    fn compute_min_dices(&self) -> Play {
        let mut min_red: u32 = 0;
        let mut min_green: u32 = 0;
        let mut min_blue: u32 = 0;
        for play in &self.plays {
            if play.red > min_red {
                min_red = play.red;
            }
            if play.blue > min_blue {
                min_blue = play.blue;
            }
            if play.green > min_green {
                min_green = play.green;
            }
        }
        Play {
            red: min_red,
            blue: min_blue,
            green: min_green,
        }
    }
}

fn parse_games(lines: &str) -> Vec<Game> {
    let mut out: Vec<Game> = Vec::new();
    for line in lines.split("\n") {
        if line.is_empty() {
            continue;
        }
        let content: Vec<&str> = line.split(':').collect();
        let (game, plays) = content.split_first().unwrap();
        let game_number = game[5..].parse::<u32>().unwrap();
        let mut play_vec: Vec<Play> = Vec::new();
        // println!("Game: {}", game_number);
        // println!("plays : {:?}", plays);
        let games_str: Vec<&str> = plays[0].split(';').map(|x| x.trim()).collect();
        for dices_plays in games_str {
            let mut play = Play::new();
            let play_str_vec: Vec<&str> = dices_plays.split(",").collect();
            for play_str in play_str_vec {
                // println!("play_str : {}",play_str);
                play.parse_str(play_str);
            }
            // println!("{:?}", play);
            play_vec.push(play);
        }
        out.push(Game {
            id: game_number,
            plays: play_vec,
        });
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
    let games = parse_games(&content);
    // println!("{:?}", games);
    let mut result: u32 = 0;
    for game in &games {
        let mut valid = true;
        for play in &game.plays {
            if !play.is_valid(12, 13, 14) {
                valid = false;
            }
        }
        if valid == false {
            continue;
        }
        result += game.id;
    }
    println!("Result 1 = {}", result);

    let mut power_result : u32 = 0;
    for game in &games {
        let min_dices = &game.compute_min_dices();
        let power = min_dices.power();
        // println!("Plays power is : {}",power);
        power_result+=power;
    }
    println!("Result 2 = {}", power_result);

}
