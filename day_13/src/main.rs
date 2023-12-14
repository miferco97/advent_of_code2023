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

fn find_mirrors(matrix: &Matrix<char>, axis: u32) -> Option<u64> {
    let n_dims = match axis {
        0 => matrix.rows,
        1 => matrix.cols,
        _ => panic!("axis not defined"),
    };

    let mut candidates: Vec<u32> = Vec::new();

    for i in 0..n_dims - 1 {
        let equal;
        if axis == 0 {
            equal = equal_vecs(&matrix.row(i), &matrix.row(i + 1));
        } else {
            equal = equal_vecs(&matrix.col(i), &matrix.col(i + 1));
        }
        if equal {
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
            let dist = candidate-i;
            let index1 = candidate+1 + dist;
            // println!("i : {} j: {}",index0,index1);
            let equal;
            if axis == 0 {
                equal = equal_vecs(&matrix.row(index0), &matrix.row(index1));
            } else {
                equal = equal_vecs(&matrix.col(index0), &matrix.col(index1));
            }
            if !equal {
                // println!("not equal");
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

fn compute_mirrors(matrix_vec: &Vec<Matrix<char>>) -> u64{
    let mut value = 0;
    for matrix in matrix_vec {
        if let Some(t) = find_mirrors(&matrix, 1) {
            value+=t;
        }
        else if let Some(t) = find_mirrors(&matrix, 0) {
            value+=t*100;
        }
        else {
             println!("\n no mirror \n{}",matrix);
        }
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
    let part1 = compute_mirrors(&matrix_vec);
    println!("PART_1 : {}", part1);

    // println!("PART_2 : {}", dist_2);
}
