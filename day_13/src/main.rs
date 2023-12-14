use std::env;
use std::fs;
use std::process::exit;

use day_13::*;

fn equal_vecs<T: PartialEq>(v1: &Vec<T>, v2: &Vec<T>) -> bool {
    if v1.len() != v2.len() {
        return false;
    }
    for i in 0..v1.len() {
        if v1[i] != v2[i] {
            return false;
        }
    }
    true
}

fn find_discrepancies(
    matrix: &Matrix<char>,
    index0: u32,
    index1: u32,
    axis: u32,
) -> Option<Vec<(u32, u32)>> {
    // if axis = 0 -> compare rows
    // if axis = 1 -> compare columns
    // let (index_0, index_1) = match axis{
    //     0 =>
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
            let (i, j) = match axis {
                0 => (index0, i as u32),
                1 => (i as u32, index0),
                _ => panic!("axis not supported"),
            };

            discrepancies.push((i, j));
        }
    }

    if discrepancies.is_empty() {
        return None;
    }
    Some(discrepancies)
}

fn find_mirrors(
    matrix: &mut Matrix<char>,
    axis: u32,
    perfect_reflection: bool,
    allow_change: bool,
    remove_candidate: Option<u32>,
) -> Option<u64> {
    // if none then we have to call it again

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
        if !allow_change || perfect_reflection {
            continue;
        }
        if let Some(discrepancy) = find_discrepancies(&matrix, i, i + 1, axis) {
            // println!("{}", matrix);
            if discrepancy.len() > 1 {
                continue;
            }
            let (i, j) = discrepancy[0];
            let curr_value = matrix.at(i, j);
            println!("[find reflection axis] discrepancy in axis: {axis} at [{i},{j}]");
            *matrix.at_mut(discrepancy[0].0, discrepancy[0].1) = match curr_value {
                '.' => '#',
                '#' => '.',
                _ => panic!("not expected character"),
            };
            return None;
        }
    }
    // if candidates.is_empty() {
    // if true {
    //     println!("entering");
    //     for i in 0..n_dims - 1 {
    //     }
    // }

    for candidate in candidates {
        if remove_candidate.is_some_and(|x| x==candidate){
            println!(" axis {axis} ,removing candidate {}", candidate);
            continue;
        }
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
            if let Some(discrepancy) = find_discrepancies(&matrix, index0, index1, axis) {
                if perfect_reflection || !allow_change {
                    valid = false;
                    break;
                }

                let (i, j) = discrepancy[0];
                println!("[check reflection] discrepancy in axis: {axis} at [{i},{j}]");
                let curr_value = matrix.at(i, j);
                *matrix.at_mut(discrepancy[0].0, discrepancy[0].1) = match curr_value {
                    '.' => '#',
                    '#' => '.',
                    _ => panic!("not expected character"),
                };
                return None;
            }
        }
        if valid {
            return Some(candidate as u64 + 1);
        }
    }
    Some(0)
}

fn compute_mirrors(matrix_vec: &mut Vec<Matrix<char>>, perfect_reflection: bool, last_values : &mut Vec<(u32,u32)>) -> u64 {
    let mut value = 0;
    let mut i = 0;
    for matrix in matrix_vec {
        // println!("Before {}", matrix);
        if !perfect_reflection {
            if find_mirrors(matrix, 0, perfect_reflection, true,None).is_some() {
                find_mirrors(matrix, 1, perfect_reflection, true,None);
            }
        }
        let mut remove_v_candidate= None;
        let mut remove_h_candidate= None;
        if ! perfect_reflection{
            match last_values[i].0{
                0 => remove_h_candidate = Some(last_values[i].1),
                1 => remove_v_candidate = Some(last_values[i].1),
                _ => panic!("not expected value"),
            }
        }
            
        if let Some(t) = find_mirrors(matrix, 1, perfect_reflection, false, remove_v_candidate) {
            if t > 0 {
                if perfect_reflection{
                    last_values.push((1,t as u32 -1));
                }
                value += t;
                println!(" {i} vertical reflection {t}");

            }
        }
        if let Some(t) = find_mirrors(matrix, 0, perfect_reflection, false, remove_h_candidate) {
            if t > 0 {
                if perfect_reflection{
                    last_values.push((0,t as u32-1));
                }
                value += 100 * t;
                println!(" {i} horizontal reflection {t}");
            }
        }
        // println!("After {}", matrix);
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
    let mut v = Vec::new();
    let part1 = compute_mirrors(&mut matrix_vec, true,&mut v);
    println!("PART_1 : {}", part1); // 27664

    let part2 = compute_mirrors(&mut matrix_vec, false, &mut v);
    println!("PART_2 : {}", part2); // 33991
}
