use std::env;
use std::fs;
use std::process::exit;

use day_13::*;

type Discrepancy = (u32,u32);
type Smack = [Discrepancy;2];

fn find_discrepancies(
    matrix: &Matrix<char>,
    index0: u32,
    index1: u32,
    axis: u32,
) -> Option<Vec<Smack>> {
    let (v1, v2) = match axis {
        0 => (matrix.row(index0), matrix.row(index1)),
        1 => (matrix.col(index0), matrix.col(index1)),
        _ => panic!("axis not supported"),
    };
    if v1.len() != v2.len() {
        panic!("vec lenghts not compatible");
    }

    let mut discrepancies = Vec::new();
    for i in 0..v1.len() {
        if v1[i] != v2[i] {
            let smack = match axis {
                0 => [(index0, i as u32), (index1, i as u32)],
                1 => [(i as u32, index0), (i as u32, index1)],
                _ => panic!("axis not supported"),
            };

            discrepancies.push(smack);
        }
    }

    if discrepancies.is_empty() {
        return None;
    }
    Some(discrepancies)
}

fn find_symmetry(matrix: &Matrix<char>, axis: u32) -> Option<u64> {
    let n_dims = match axis {
        0 => matrix.rows,
        1 => matrix.cols,
        _ => panic!("axis not defined"),
    };

    let mut candidates: Vec<u32> = Vec::new();

    for i in 0..n_dims - 1 {
        if find_discrepancies(&matrix, i, i + 1, axis).is_none() {
            candidates.push(i);
        }
    }

    for candidate in candidates {
        let mut valid = true;
        let dist_right = n_dims - 1 - (candidate + 1);
        let max_dist = (candidate).min(dist_right);
        // println!(
        //     "i {candidate}, dist_left: {candidate}, dist_right:{dist_right}, max_dist: {max_dist}"
        // );
        for i in (candidate - max_dist)..(candidate) {
            let index0 = i;
            let dist = candidate - i;
            let index1 = candidate + 1 + dist;
            if find_discrepancies(&matrix, index0, index1, axis).is_some() {
                valid = false;
                break;
            }
        }

        if valid {
            return Some(candidate as u64 + 1);
        }
    }
    None
}

fn find_mirrors(matrix: &Matrix<char>, axis: u32) -> Option<u64> {
    // if none then we have to call it again

    let n_dims = match axis {
        0 => matrix.rows,
        1 => matrix.cols,
        _ => panic!("axis not defined"),
    };

    let mut discrepancies = Vec::new();
    let mut candidates: Vec<u32> = Vec::new();

    for i in 0..n_dims - 1 {
        if find_discrepancies(&matrix, i, i + 1, axis).is_none() {
            candidates.push(i);
        }
        if let Some(discrepancy) = find_discrepancies(&matrix, i, i + 1, axis) {
            if discrepancy.len() > 1 {
                continue;
            }
            discrepancies.push(discrepancy[0]);
        }
    }
    println!("candidates: {:?}", candidates);
    if discrepancies.len() == 0 {
        // if no have been found search after a reflection axis and try again
        for candidate in candidates {
            let dist_right = n_dims - 1 - (candidate + 1);
            let max_dist = (candidate).min(dist_right);
            for i in (candidate - max_dist)..(candidate) {
                let index0 = i;
                let dist = candidate - i;
                let index1 = candidate + 1 + dist;
                if let Some(discrepancy) = find_discrepancies(&matrix, index0, index1, axis) {
                    if discrepancy.len() > 1 {
                        continue;
                    }
                    discrepancies.push(discrepancy[0]);
                }
            }
        }
    }

    println!("discrepancies: {:?}", discrepancies);
    for smacks in discrepancies {
        let mut copy_matrix = matrix.clone();
        for discrepancy in smacks{
            println!("changed: [{},{}]", discrepancy.0, discrepancy.1);

        let curr_value = matrix.at(discrepancy.0, discrepancy.1);
        // println!("[find reflection axis] discrepancy in axis: {axis} at [{i},{j}]");
        *copy_matrix.at_mut(discrepancy.0, discrepancy.1) = match curr_value {
            '.' => '#',
            '#' => '.',
            _ => panic!("not expected character"),
        };

        println!("copy_matrix: {}", copy_matrix);
        if let Some(t) = find_symmetry(&copy_matrix, axis) {
            return Some(t);
        }
    }
    }
    // find_symmetry(matrix, axis)
    None
}

fn compute_mirrors(
    matrix_vec: &mut Vec<Matrix<char>>,
    perfect_reflection: bool,
    _last_values: &mut Vec<(u32, u32)>,
) -> u64 {
    if perfect_reflection {
        let mut value = 0;
        for matrix in matrix_vec {
            if let Some(t) = find_symmetry(matrix, 1) {
                value += t;
            } else if let Some(t) = find_symmetry(matrix, 0) {
                value += 100 * t;
            }
        }
        return value;
    }

    let mut value = 0;
    let mut i = 0;
    for matrix in matrix_vec {
        if let Some(t) = find_mirrors(matrix, 1) {
            value += t;
        } else if let Some(t) = find_mirrors(matrix, 0) {
            value += 100 * t;
        } else {
            println!(" {i} no reflection {}", matrix);
        }
        i += 1;
    }
    value
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("Filename not provided");
        exit(0);
    }
    let filename: &str = args[1].as_str();
    let content = fs::read_to_string(filename).expect("Error reading the file");
    let mut matrix_vec: Vec<Matrix<char>> = Vec::new();
    for mat_str in content.split("\n\n") {
        matrix_vec.push(Matrix::new_from_str(mat_str));
    }
    let mut v = vec![(2, 0); matrix_vec.len()];
    let part1 = compute_mirrors(&mut matrix_vec, true, &mut v);
    println!("PART_1 : {}", part1); // 27664

    let part2 = compute_mirrors(&mut matrix_vec, false, &mut v);
    println!("PART_2 : {}", part2); // 33991
}
