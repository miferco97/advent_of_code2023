use std::env;
use std::fs;
use std::process::exit;

#[derive(Debug)]
struct Point {
    i: i32,
    j: i32,
}

#[derive(Debug)]
struct Number {
    number: u32,
    initial_point: Point,
    final_point: Point,
}

#[derive(Debug)]
struct Symbol {
    symbol: char,
    position: Point,
}

fn parse_content(str: &str) -> (Vec<Number>, Vec<Symbol>) {
    // str.len() -1 eliminates last \n
    let content: Vec<&str> = str.split("\n").collect();
    // println!("{:?}",content);
    let mut numbers_vec: Vec<Number> = Vec::new();
    let mut symbols_vec: Vec<Symbol> = Vec::new();
    let mut i: i32 = -1;
    let mut j: i32;
    let mut j_begin_number: i32;
    let mut j_end_number: i32;

    for line in content {
        i += 1;
        j = -1;
        j_begin_number = -1;
        j_end_number = -1;
        // first search for non_numerical
        for _char in line.chars() {
            j += 1;
            if _char.is_digit(10) {
                if j_begin_number == -1 {
                    j_begin_number = j;
                }
                j_end_number = j;
                // println!("number :{}", _char)
            } else {
                if j_end_number != -1 {
                    let number_str = &line[j_begin_number as usize..j as usize];
                    let number = number_str.parse::<u32>().unwrap();
                    // println!("number_str:{}",number_parsing); // simulates storing the number
                    numbers_vec.push(Number {
                        number,
                        initial_point: Point {
                            i,
                            j: j_begin_number,
                        },
                        final_point: Point { i, j: j_end_number },
                    });
                    j_end_number = -1;
                    j_begin_number = -1;
                }
                if _char == '.' || _char == ' ' {
                    continue;
                }
                // println!("Symbol : {}", _char);
                symbols_vec.push(Symbol {
                    symbol: _char,
                    position: Point { i, j },
                });
            }
        }
        // check if the last character is a number
        if j_end_number != -1 {
            let number_str = &line[j_begin_number as usize..];
            let number = number_str.parse::<u32>().unwrap();
            // println!("number_str:{}",number_parsing); // simulates storing the number
            numbers_vec.push(Number {
                number,
                initial_point: Point {
                    i,
                    j: j_begin_number,
                },
                final_point: Point { i, j: j_end_number },
            });
        }
    }
    // println!("numbers: \n{:?}", numbers_vec);
    // println!("symbols : \n{:?}", symbols_vec);
    (numbers_vec, symbols_vec)
}

fn check_adjacent(symbol: &Symbol, number: &Number) -> bool {
    if (symbol.position.i < number.initial_point.i - 1)
        || (symbol.position.i > number.initial_point.i + 1)
    {
        return false;
    }
    if (symbol.position.j < number.initial_point.j - 1)
        || (symbol.position.j > number.final_point.j + 1)
    {
        return false;
    }
    true
}

fn count_numbers(numbers: &Vec<Number>, symbols: &Vec<Symbol>) {
    let mut sum = 0;
    for number in numbers {
        let mut not_adjactent = true;
        for symbol in symbols {
            if check_adjacent(symbol, number) {
                not_adjactent = false;
            }
        }
        if not_adjactent {
            // println!("{:?} is not adjacent to a symbol", number.number);
        } else {
            // println!("{:?} is adjacent to a symbol", number.number);
            sum += number.number;
        }
    }
    println!("Sum : {}", sum);
}

fn count_gears(numbers: &Vec<Number>, symbols: &Vec<Symbol>) {
    let mut sum = 0;
    for symbol in symbols {
        if symbol.symbol != '*' {
            continue;
        }
        let mut adjacent = 0;
        let mut adjacent_product = 1;
        for number in numbers {
            if check_adjacent(symbol, number) {
                adjacent += 1;
                adjacent_product *= number.number;
            }
        }
        if adjacent == 2 {
            sum += adjacent_product;
        }
    }
    println!("Sum gears ratio : {}", sum);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("Filename not provided");
        exit(0);
    }
    let filename: &str = args[1].as_str();
    let content = fs::read_to_string(filename).expect("Error reading the file");
    let (numbers, symbols) = parse_content(&content);
    count_numbers(&numbers, &symbols);
    count_gears(&numbers, &symbols);
}
