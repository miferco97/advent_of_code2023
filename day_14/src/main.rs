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

fn slide_direction(matrix: &mut Matrix<char>, direction: Direction) {
    match direction {
        Direction::North => {}
        Direction::East => {}
        Direction::South => {}
        Direction::West => {}
    }

    if let Direction::North = direction {}

    let copy_matrix = matrix.clone();

    for j in 0..copy_matrix.cols {
        let vec = copy_matrix.col(j);
        // let mut new_vec = vec.clone();
        let mut index = 0;
        while index < copy_matrix.rows {
            let n_items;
            if let Some(pos_item) = &vec[index as usize..].iter().position(|x| **x == '#') {
                n_items = index as usize + *pos_item;
            } else {
                n_items = copy_matrix.rows as usize;
            }
            // println!("index : {index }, n_items{n_items}, len {}",matrix.rows);
            if index == n_items as u32 {
                index+=1;
                continue;
            }
            let n_rocks = &vec[index as usize..n_items]
                .iter()
                .filter(|x| ***x == 'O')
                .count();
            let mut n_rocks = *n_rocks;
            for i in index as usize..n_items {
                let value = copy_matrix.at(i as u32, j);
                if value == &'#' { continue; }
                let new_value;
                if n_rocks > 0 {
                    new_value = 'O';
                    n_rocks-=1;
                }
                else{
                    new_value = '.';
                }
                *matrix.at_mut(i as u32, j) = new_value;
            }
            index=n_items as u32 +1;
        }
    }
}


fn compute_load (matrix: &Matrix<char>)->u64{
    let mut load = 0;
    for i in 0..matrix.rows{
         for j in 0..matrix.rows{
             if matrix.at(i, j) == &'O' {
                 load+= matrix.rows as u64-i as u64;
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
    // println!("Matrix: {}", matrix);
    slide_direction(&mut matrix, Direction::North);
    println!("PART 1: {}",compute_load(&matrix));
    // println!("Matrix 2: {}", matrix);

    // println!("PART 2: {part2}");
}
