use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::exit;

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone, Copy, Hash)]
enum CardNumber {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl CardNumber {
    fn new_from_char(str: &char, use_jokers: bool) -> CardNumber {
        let card = match str {
            'A' => CardNumber::Ace,
            '2' => CardNumber::Two,
            '3' => CardNumber::Three,
            '4' => CardNumber::Four,
            '5' => CardNumber::Five,
            '6' => CardNumber::Six,
            '7' => CardNumber::Seven,
            '8' => CardNumber::Eight,
            '9' => CardNumber::Nine,
            'T' => CardNumber::Ten,
            'J' => {
                if use_jokers {
                    return CardNumber::Joker;
                }
                CardNumber::Jack
            }
            'Q' => CardNumber::Queen,
            'K' => CardNumber::King,
            _ => {
                panic!("Card doesnt exist");
            }
        };
        card
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone, Copy)]
enum Play {
    Repoker,
    Poker,
    Full,
    Three,
    TwoPair,
    Pair,
    High,
}

impl Play {
    fn compute_play(cards: &[CardNumber; 5]) -> Play {
        let mut set = HashMap::new();
        for card in cards {
            let mut count = 1u32;
            if let Some(value) = set.get(card) {
                count += value;
            }
            set.insert(card, count);
        }
        let play = match set.len() {
            1 => Play::Repoker,
            2 => {
                if set.get(&CardNumber::Joker).is_some() {
                    return Play::Repoker;
                } else if set.iter().any(|(_key, val)| *val == 4) {
                    return Play::Poker;
                }
                Play::Full
            }
            3 => {
                if set.iter().any(|(_key, val)| *val == 3) {
                    if set.get(&CardNumber::Joker).is_some() {
                        return Play::Poker;
                    }
                    return Play::Three;
                }
                if set.get(&CardNumber::Joker).is_some_and(|x| *x == 2) {
                    return Play::Poker;
                } else if set.get(&CardNumber::Joker).is_some_and(|x| *x == 1) {
                    return Play::Full;
                }

                Play::TwoPair
            }
            4 => {
                if set.get(&CardNumber::Joker).is_some() {
                    return Play::Three;
                }
                Play::Pair
            }
            5 => {
                if set.get(&CardNumber::Joker).is_some() {
                    return Play::Pair;
                }
                Play::High
            }
            _ => panic!("Error matching play"),
        };
        play
    }
}

#[derive(Debug)]
struct Hand {
    cards: [CardNumber; 5],
    bet: u32,
    play: Play,
}

impl Hand {
    fn new_from_str(hand_str: &str, bet_str: &str, use_jokers: bool) -> Hand {
        let mut cards_array: [CardNumber; 5] = [CardNumber::Two; 5];
        for (index, char) in hand_str.chars().enumerate() {
            let card = CardNumber::new_from_char(&char, use_jokers);
            // println!("card : {:?}", card);
            cards_array[index] = card;
        }
        let bet = bet_str.replace(" ", "").parse::<u32>().unwrap();
        Hand {
            cards: cards_array,
            bet,
            play: Play::compute_play(&cards_array),
        }
    }
    fn cmp(&self, other: &Self) -> Ordering {
        let comp_result = self.play.partial_cmp(&other.play).unwrap();
        if let Ordering::Equal = comp_result {
            for i in 0..5 {
                let card_result = self.cards[i].partial_cmp(&other.cards[i]).unwrap();
                if let Ordering::Equal = card_result {
                    continue;
                }
                return card_result;
            }
        }
        comp_result
    }
}

fn parse_content(str: &str, use_jokers: bool) -> Vec<Hand> {
    let mut hands: Vec<Hand> = Vec::new();
    let content: Vec<&str> = str.split("\n").filter(|x| !x.is_empty()).collect();
    for line in content {
        let (hand_str, bet_str) = line.split_once(" ").unwrap();
        hands.push(Hand::new_from_str(hand_str, bet_str, use_jokers));
    }
    hands
}

fn compute_winnings(hands: &mut Vec<Hand>) -> usize {
    hands.sort_by(|a, b| a.cmp(b));
    // println!("sorted hands {:?}",hands);
    let mut winnings: usize = 0;
    for (i, hand) in hands.iter().enumerate() {
        let rank = hands.len() - i;
        winnings += rank * (hand.bet as usize);
        // println!(
        //     "rank {:3} : bet {:3} : play{:?} : card {:?}",
        //     rank, hand.bet, hand.play, hand.cards
        // )
    }
    winnings
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Filename not provided");
        exit(0);
    }
    let filename: &str = args[1].as_str();
    let content = fs::read_to_string(filename).expect("Error reading the file");
    let mut hands = parse_content(&content, false);
    println!("Winnings part 1  are {}", compute_winnings(&mut hands));
    let mut hands_with_jokers = parse_content(&content, true);
    println!(
        "Winnings part 2  are {}",
        compute_winnings(&mut hands_with_jokers)
    );
}
