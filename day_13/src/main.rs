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
// row, col expansions
fn find_mirrors(matrix: &Matrix<char>) -> u64 {
    // cols first
    let mut col_mirror = 0;
    for i in 0..matrix.cols-1 {
        if equal_vecs(&matrix.col(i), &matrix.col(i+1)) {
            col_mirror = i;
            break;
        }
    }
    println!("col_mirror: {}", col_mirror);
    0
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
    for matrix in matrix_vec {
        let part1 = find_mirrors(&matrix);
    }
    // let matrix = Matrix::new_from_str(&content);

    // println!("PART_1 : {}", dist_1);
    // println!("PART_2 : {}", dist_2);
}
