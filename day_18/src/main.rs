// use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::exit;
// use std::{thread, time};

use day_18::*;

#[derive(Debug, PartialEq, PartialOrd)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn from_char(_char: &char) -> Direction {
        match _char {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Direction : {} not known ", _char),
        }
    }
}

#[derive(Debug)]
struct DigPlan {
    direction: Direction,
    steps: usize,
    color: String,
}

impl DigPlan {
    fn from_str(_str: &str) -> DigPlan {
        let splits: Vec<&str> = _str.split_whitespace().collect();
        let direction = Direction::from_char(&splits[0].chars().next().unwrap());
        let steps = splits[1].parse().unwrap();
        let color = String::from(splits[2]);
        DigPlan {
            direction,
            steps,
            color,
        }
    }
}

fn max_min_from_direction(
    plans_vec: &Vec<DigPlan>,
    incr_dir: &Direction,
    dec_dir: &Direction,
) -> (i64, i64) {
    let mut min = i64::MAX;
    let mut max = i64::MIN;
    let mut position = 0;
    for elem in plans_vec {
        if elem.direction == *incr_dir {
            position += elem.steps as i64;
        } else if elem.direction == *dec_dir {
            position -= elem.steps as i64;
        }
        if position > max {
            max = position;
        }
        if position < min {
            min = position;
        }
    }

    (min, max)
}

type Coord = (usize, usize);

fn compute_grid_size(plans_vec: &Vec<DigPlan>) -> (Coord, Coord) {
    let (min_i, max_i) = max_min_from_direction(plans_vec, &Direction::Down, &Direction::Up);
    let (min_j, max_j) = max_min_from_direction(plans_vec, &Direction::Right, &Direction::Left);

    let horizontal_increase = -min_j + max_j;
    let vertical_increase = -min_i + max_i;
    (
        (
            vertical_increase as usize + 1,
            horizontal_increase as usize + 1,
        ),
        (-min_i as usize, -min_j as usize),
    )
}

fn fill_map(map: &mut Matrix<char>, plans_vec: &Vec<DigPlan>, origin: &Coord) {
    let mut initial_i = origin.0;
    let mut initial_j = origin.1;
    for plan in plans_vec {
        // println!("initial_i : {initial_i}, initial_j: {initial_j}, {:?}",plan);
        match plan.direction {
            Direction::Up => {
                for i in initial_i - plan.steps..initial_i {
                    *map.at_mut(i, initial_j) = '#';
                }
                initial_i = initial_i - plan.steps;
            }
            Direction::Down => {
                for i in initial_i..initial_i + plan.steps + 1 {
                    *map.at_mut(i, initial_j) = '#';
                }
                initial_i = initial_i + plan.steps;
            }
            Direction::Left => {
                for j in initial_j - plan.steps..initial_j {
                    // println!("i:{initial_i},j:{j}");
                    *map.at_mut(initial_i, j) = '#';
                }
                initial_j = initial_j - plan.steps;
            }
            Direction::Right => {
                for j in initial_j..initial_j + plan.steps + 1 {
                    *map.at_mut(initial_i, j) = '#';
                }
                initial_j = initial_j + plan.steps;
            }
        }

        // println!("{}",map);
    }
}

fn count_fill(map: &Matrix<char>) -> usize {
    map.data
        .iter()
        .filter(|x| x != &&'.')
        .fold(0, |acc, _| acc + 1)
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Cover {
    Up,
    Down,
    Both,
    None,
}

fn fill_inside(map: & Matrix<char>)-> Matrix<char>{
    let mut filled_map = map.clone();
    for i in 0..map.rows {
        let mut inside = false;
        let mut cover: Vec<Cover> = Vec::new();
        for j in 0..map.cols {
            let _char: char = *map.at(i, j);

            if _char == '.' {
                if !cover.is_empty() {
                    let first = cover.first().unwrap();
                    let last = cover.last().unwrap();
                    if cover.len() == 1 || (first != last)  {
                        if inside {
                            inside = false;
                        } else {
                            inside = true;
                        }
                    }
                    cover.clear();
                }
                if inside {
                    *filled_map.at_mut(i, j) = '#';
                }
            }

            if _char == '#' {
                // check up
                let mut index = None;
                if (i > 0) && (map.at(i - 1, j) == &'#') {
                    cover.push(Cover::Up);
                    *filled_map.at_mut(i, j) = '^';
                    index = Some(cover.len() - 1);
                }
                if (i < map.rows - 1) && (map.at(i + 1, j) == &'#'){
                    if index.is_some() {
                        *cover.last_mut().unwrap() = Cover::Both;
                        *filled_map.at_mut(i, j) = '|';
                    } else {
                        cover.push(Cover::Down);
                        *filled_map.at_mut(i, j) = 'v';
                    }
                    index = Some(cover.len() - 1);
                }
                if index.is_none() {
                    cover.push(Cover::None);
                    *filled_map.at_mut(i, j) = '-';
                }
            }
        }
    }
    filled_map
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("Filename not provided");
        exit(0);
    }

    let filename: &str = args[1].as_str();
    let content = fs::read_to_string(filename).expect("Error reading the file");
    let plans: Vec<DigPlan> = content
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| DigPlan::from_str(&x))
        .collect();

    let (dims, centre) = compute_grid_size(&plans);
    let mut map = Matrix::new(dims.0, dims.1, '.');

    // println!("{:?}", plans);
    // println!("{:?}", compute_grid_size(&plans));
    fill_map(&mut map, &plans, &centre);
    // println!("{}", map);
    let filled_map = fill_inside(&mut map);
    // println!("{}", filled_map);
    println!("PART 1: {}", count_fill(&filled_map));
}
