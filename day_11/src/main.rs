use std::env;
use std::fs;
use std::process::exit;

use day_11::*;

// row, col expansions
fn count_expansions(matrix: &Matrix<char>) -> (Vec<u64>, Vec<u64>) {
    let mut row_indices: Vec<u64> = Vec::new();
    let mut col_indices: Vec<u64> = Vec::new();
    for i in 0..matrix.rows {
        if matrix.row(i).iter().find(|&&x| *x == '#').is_none() {
            row_indices.push(1);
        } else {
            row_indices.push(0);
        }
    }
    for j in 0..matrix.cols {
        if matrix.col(j).iter().find(|&&x| *x == '#').is_none() {
            col_indices.push(1);
        } else {
            col_indices.push(0);
        }
    }
    (row_indices, col_indices)
}

fn dist_between_stars(
    star1: (i64, i64),
    star2: (i64, i64),
    row_expansions: &Vec<u64>,
    col_expansions: &Vec<u64>,
    expansion: u64,
) -> u64 {
    let (x1, y1) = star1;
    let (x2, y2) = star2;
    if (x1 == x2) && (y1 == y2) {
        return 0;
    }

    let min_i = std::cmp::min(x1, x2) as usize;
    let max_i = std::cmp::max(x1, x2) as usize;
    let min_j = std::cmp::min(y1, y2) as usize;
    let max_j = std::cmp::max(y1, y2) as usize;

    let horizontal_expansion = &row_expansions[min_i..max_i].iter().sum::<u64>();
    let vertical_expansion = &col_expansions[min_j..max_j].iter().sum::<u64>();
    let dist = (x1 - x2).abs() + (y1 - y2).abs();

    dist as u64 + (horizontal_expansion + vertical_expansion) * expansion
}

fn find_stars(matrix: &Matrix<char>) -> Vec<(i64, i64)> {
    let mut stars: Vec<(i64, i64)> = Vec::new();
    for i in 0..matrix.rows {
        for j in 0..matrix.cols {
            if matrix.at(i, j) == &'#' {
                stars.push((i as i64, j as i64));
            }
        }
    }
    stars
}

fn compute_distances_to_all_stars(
    stars: &Vec<(i64, i64)>,
    row_expansions: &Vec<u64>,
    col_expansions: &Vec<u64>,
    expansion: u64,
) -> u64 {
    let mut dist = 0;
    for i in 0..stars.len() {
        for j in i..stars.len() {
            dist += dist_between_stars(
                stars[i],
                stars[j],
                row_expansions,
                col_expansions,
                expansion,
            ) as u64;
        }
    }
    dist
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("Filename not provided");
        exit(0);
    }
    let filename: &str = args[1].as_str();
    let content = fs::read_to_string(filename).expect("Error reading the file");
    let matrix = Matrix::new_from_str(&content);
    let (row_expansions, col_expansions) = count_expansions(&matrix);
    let stars = find_stars(&matrix);
    let dist_1 = compute_distances_to_all_stars(&stars, &row_expansions, &col_expansions, 1);
    println!("PART_1 : {}", dist_1);
    let dist_2 = compute_distances_to_all_stars(&stars, &row_expansions, &col_expansions, 999_999);
    println!("PART_2 : {}", dist_2);
}
