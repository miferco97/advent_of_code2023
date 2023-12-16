use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::exit;

use day_14::*;

#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn slide_vector(input: &Vec<&char>, inverse: bool) -> Vec<char> {
    let vec = match inverse {
        true => input.iter().rev().cloned().collect(),
        false => input.clone(),
    };
    let mut out = vec.iter().map(|x| **x).collect::<Vec<char>>();

    let mut index = 0;
    while index < vec.len() {
        let n_items;
        if let Some(pos_item) = &vec[index as usize..].iter().position(|x| **x == '#') {
            n_items = index as usize + *pos_item;
        } else {
            n_items = vec.len();
        }

        let n_rocks = &vec[index..n_items].iter().filter(|x| ***x == 'O').count();
        let mut n_rocks = *n_rocks;
        for i in index as usize..n_items {
            let value = vec[i];
            if value == &'#' {
                continue;
            }
            let new_value;
            if n_rocks > 0 {
                new_value = 'O';
                n_rocks -= 1;
            } else {
                new_value = '.';
            }
            out[i] = new_value;
        }
        index = n_items + 1;
    }
    if inverse {
        out.reverse();
    }
    out
}

fn slide_direction(matrix: &mut Matrix<char>, direction: Direction) {
    let reverse;
    match direction {
        Direction::North | Direction::West => {
            reverse = false;
        }
        Direction::South | Direction::East => {
            reverse = true;
        }
    }

    let copy_matrix = matrix.clone();
    let n_rows_or_cols = match direction {
        Direction::North | Direction::South => matrix.cols,
        Direction::East | Direction::West => matrix.rows,
    };

    for j in 0..n_rows_or_cols {
        match direction {
            Direction::North | Direction::South => {
                let vec = copy_matrix.col(j);
                let vec = slide_vector(&vec, reverse);
                matrix.substitute_col(j, vec);
            }
            Direction::East | Direction::West => {
                let vec = copy_matrix.row(j);
                let vec = slide_vector(&vec, reverse);
                matrix.substitute_row(j, vec);
            }
        };
    }
}

fn compute_load(matrix: &Matrix<char>) -> u64 {
    let mut load = 0;
    for i in 0..matrix.rows {
        for j in 0..matrix.rows {
            if matrix.at(i, j) == &'O' {
                load += matrix.rows as u64 - i as u64;
            }
        }
    }
    load
}

fn hash_matrix(matrix: &Matrix<char>) -> String {
    matrix.data.iter().collect()
}

fn cycle_matrix(matrix: &mut Matrix<char>, n_times: Option<usize>) -> (usize, usize) {
    let cycle = [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];

    let mut dict: HashMap<String, usize> = HashMap::new();
    let mut n_changes = 0;

    loop {
        for dir in cycle.iter() {
            slide_direction(matrix, *dir);
        }
        n_changes += 1;

        if let None = n_times {
            // infinite case
            let hash = hash_matrix(&matrix);
            if let Some(value) = dict.get(&hash) {
                return (*value, n_changes);
            } else {
                dict.insert(hash.clone(), n_changes);
            }
        } else {
            if n_changes >= n_times.unwrap() {
              return (0, n_changes);
            }
        }
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
    let mut matrix = Matrix::new_from_str(&content);

    // TEST
    let mut part1_matrix = matrix.clone();
    let mut part2_matrix = matrix.clone();

    slide_direction(&mut part1_matrix, Direction::North);
    println!("PART 1: {}", compute_load(&part1_matrix));

    let (begin, n_changes) = cycle_matrix(&mut matrix, None);
    let period = n_changes - begin ;
    let n_iters = begin + (1000000000-begin) % period;
    cycle_matrix(&mut part2_matrix, Some(n_iters));
    println!("PART 2: {}", compute_load(&part2_matrix)); 
}
