use std::env;
use std::fs;
use std::process::exit;

use day_14::*;

enum Direction {
    North,
    East,
    South,
    West,
}

fn slide_vector(input: &Vec<&char>, inverse: bool) -> Vec<char> {
    // println!("input: {:?}", input);
    let vec = match inverse {
        true => input.iter().rev().cloned().collect(),
        false => input.clone(),
    };
    // println!("input: {:?}", vec);
    let mut out = vec.iter().map(|x| **x).collect::<Vec<char>>();

    let mut index = 0;
    while index < vec.len() {
        let n_items;
        if let Some(pos_item) = &vec[index as usize..].iter().position(|x| **x == '#') {
            n_items = index as usize + *pos_item;
        } else {
            n_items = vec.len();
        }

        // println!("index : {index}, n_items {n_items}, len {}, slice: {:?}",vec.len(),&vec[index..n_items]);
        // if index == n_items {
        //     index += 1;
        //     continue;
        // }
        let n_rocks = &vec[index..n_items]
            .iter()
            .filter(|x| ***x == 'O')
            .count();
        let mut n_rocks = *n_rocks;
        // println!("n_rocks: {:?} in {:?}", n_rocks, &vec[index..n_items]);
        // println!("n_rocks: {:?}", n_rocks);
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
    // println!("out:   {:?}\n", out);
    if inverse {
        out.reverse();
    }
    out

}

fn slide_direction(matrix: &mut Matrix<char>, direction: Direction) {
    let reverse;
    match direction {
        Direction::North | Direction::West=> {
            reverse = false;
        }
        Direction::South | Direction::East=> {
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
            },
            Direction::East | Direction::West => {
                let vec = copy_matrix.row(j);
                let vec = slide_vector(&vec, reverse);
                matrix.substitute_row(j, vec);
            },
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




fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("Filename not provided");
        exit(0);
    }

    let filename: &str = args[1].as_str();
    let content = fs::read_to_string(filename).expect("Error reading the file");
    let mut matrix = Matrix::new_from_str(&content);
    let mut part1_matrix = matrix.clone();
    slide_direction(&mut part1_matrix, Direction::North);
    println!("PART 1: {}", compute_load(&part1_matrix));

    println!("Matrix: {}", matrix);
    println!("Matrix: {}", matrix);
    slide_direction(&mut matrix, Direction::West);
    println!("Matrix: {}", matrix);
    // println!("Matrix 2: {}", matrix);

    // println!("PART 2: {part2}");
}
