use std::fs;
use std::{env, process::exit};
use inline_colorization::*;

use day_10::*;

fn parse_content(str: &str) -> Matrix<Pipe> {
    Matrix::new_from_pipe(str)
}

fn find_start_end_direction(
    last_end: &Directions,
    pipe_type: &PipeType,
) -> (Directions, Directions) {
    let start = match last_end {
        Directions::East => Directions::West,
        Directions::West => Directions::East,
        Directions::North => Directions::South,
        Directions::South => Directions::North,
        Directions::None => {
            panic!("no start location")
        }
    };

    if *pipe_type == PipeType::Start {
        return (start, Directions::None);
    }

    // println!("last_end : {:?}, pipe_type : {:?}", last_end, pipe_type);

    let end = match start {
        Directions::North => match pipe_type {
            PipeType::Vertical => Directions::South,
            PipeType::NorthEast => Directions::East,
            PipeType::NorthWest => Directions::West,
            _ => {
                panic!("Inconsistent pipe_type")
            }
        },
        Directions::South => match pipe_type {
            PipeType::Vertical => Directions::North,
            PipeType::SouthEast => Directions::East,
            PipeType::SouthWest => Directions::West,
            _ => {
                panic!("Inconsistent pipe_type")
            }
        },
        Directions::West => match pipe_type {
            PipeType::Horizontal => Directions::East,
            PipeType::NorthWest => Directions::North,
            PipeType::SouthWest => Directions::South,
            _ => {
                panic!("Inconsistent pipe_type")
            }
        },
        Directions::East => match pipe_type {
            PipeType::Horizontal => Directions::West,
            PipeType::NorthEast => Directions::North,
            PipeType::SouthEast => Directions::South,
            _ => {
                panic!("Inconsistent pipe_type")
            }
        },
        _ => panic!("error"),
    };
    (start, end)
}

fn search_next(matrix: &mut Matrix<Pipe>, i: u32, j: u32) -> (u32, u32) {
    let curr_elem = matrix.at_mut(i, j);
    let direction = curr_elem.end.clone();
    let (next_i, next_j) = match direction {
        Directions::North => (i - 1, j),
        Directions::South => (i + 1, j),
        Directions::West => (i, j - 1),
        Directions::East => (i, j + 1),
        _ => panic!("Error finding next element"),
    };
    if next_i >= matrix.rows || next_j >= matrix.cols {
        panic!("Out of bounds");
    }
    let next_elem = matrix.at_mut(next_i, next_j);
    next_elem.status=Status::Pipe;
    let (start, end) = find_start_end_direction(&direction, &next_elem.pipe_type);
    next_elem.start = start;
    if end != Directions::None {
        next_elem.end = end;
    }
    (next_i, next_j)
}

fn find_start(matrix: &mut Matrix<Pipe>) -> (u32, u32) {
    let rows = matrix.rows;
    let cols = matrix.cols;
    let mut i = 0;
    let mut j = 0;

    for row in 0..rows {
        for col in 0..cols {
            if matrix.at(row, col).char == 'S' {
                i = row;
                j = col;
                break;
            }
        }
    }
    let mut possible_starts: Vec<(u32, u32)> = Vec::new();
    let mut possible_ends: Vec<Directions> = Vec::new();
    let steps = vec![-1i32, 0i32, 1i32];
    for inc_i in &steps {
        for inc_j in &steps {
            let next_i = i as i32 + *inc_i;
            let next_j = j as i32 + *inc_j;
            if next_i >= matrix.rows as i32
                || next_j >= matrix.cols as i32
                || next_i < 0
                || next_j < 0
            {
                continue;
            }
            if ((inc_i.abs() + inc_j.abs()) == 2) || ((inc_i.abs() + inc_j.abs()) == 0) {
                continue;
            }
            let next_elem = matrix.at(next_i as u32, next_j as u32);
            // println!("next elem {:?} => {:?},", (next_i, next_j), next_elem);
            if *inc_i == 1 {
                match next_elem.pipe_type {
                    PipeType::Vertical | PipeType::NorthEast | PipeType::NorthWest => {
                        possible_starts.push((next_i as u32, next_j as u32));
                        possible_ends.push(Directions::South)
                    }
                    _ => {}
                }
            }
            if *inc_i == -1 {
                match next_elem.pipe_type {
                    PipeType::Vertical | PipeType::SouthEast | PipeType::SouthWest => {
                        possible_starts.push((next_i as u32, next_j as u32));
                        possible_ends.push(Directions::North)
                    }
                    _ => {}
                }
            }
            if *inc_j == 1 {
                match next_elem.pipe_type {
                    PipeType::Horizontal | PipeType::NorthWest | PipeType::SouthWest => {
                        possible_starts.push((next_i as u32, next_j as u32));
                        possible_ends.push(Directions::East)
                    }
                    _ => {}
                }
            }
            if *inc_j == -1 {
                match next_elem.pipe_type {
                    PipeType::Horizontal | PipeType::NorthEast | PipeType::SouthEast => {
                        possible_starts.push((next_i as u32, next_j as u32));
                        possible_ends.push(Directions::West)
                    }
                    _ => {}
                }
            }
        }
    }
    let mut next_node = matrix.at_mut(i, j);
    next_node.end = possible_ends[0];
    next_node.status = Status::Pipe;
    // println!("next_node {:?}", next_node);
    (i, j)
}

fn traverse_pipes(matrix: &mut Matrix<Pipe>) -> u32 {
    let start_node = find_start(matrix);
    let mut next_node = search_next(matrix, start_node.0, start_node.1);
    let mut n_steps = 1u32;
    loop {
        next_node = search_next(matrix, next_node.0, next_node.1);
        let pipe = matrix.at(next_node.0, next_node.1);
        if pipe.pipe_type == PipeType::Start {
            break;
        }
        n_steps += 1;
        // println!("n_steps: {n_steps}");
    }
    if n_steps % 2 == 0 {
        return n_steps / 2;
    }
    n_steps / 2 + 1
}

fn count_cells_known (matrix:&mut Matrix<Pipe>)->u32{
    let mut n_cells_known = 0;
    for i in 0..matrix.rows {
        for j in 0..matrix.cols {
            let elem = matrix.at_mut(i, j);
            if elem.status != Status::Unknown{
                n_cells_known+=1;
            }
        }
    }
    n_cells_known
}

fn count_cells_unknown (matrix:&mut Matrix<Pipe>)->u32{
    let mut n_cells = 0;
    for i in 0..matrix.rows {
        for j in 0..matrix.cols {
            let elem = matrix.at_mut(i, j);
            if elem.status == Status::Unknown{
                n_cells+=1;
            }
        }
    }
    n_cells
}

fn fill_outside_cells(matrix: &mut Matrix<Pipe>) {
    let rows = matrix.rows;
    let cols = matrix.cols;
    let mut n_cells_known =100;
    let mut prev_n_cells_known = 0;
    while prev_n_cells_known < n_cells_known {
        prev_n_cells_known = count_cells_known(matrix);
        for i in 0..matrix.rows {
            for j in 0..matrix.cols {
                let elem = matrix.at_mut(i, j);
                let mut new_status = elem.status;
                match elem.status {
                    Status::Unknown => {
                        let steps = vec![-1i32, 0i32, 1i32];
                        for inc_i in &steps {
                            for inc_j in &steps {
                                let next_i = i as i32 + *inc_i;
                                let next_j = j as i32 + *inc_j;
                                if next_i >= rows as i32
                                    || next_j >=cols as i32
                                    || next_i < 0
                                    || next_j < 0
                                {
                                    new_status = Status::Outside;
                                    continue;
                                }
                                if (inc_i.abs() + inc_j.abs()) == 0
                                {
                                    // if its itself
                                    continue;
                                }

                                let new_elem = matrix.at_mut(next_i as u32, next_j as u32);
                                if new_elem.status == Status::Outside{
                                    new_status = Status::Outside;

                                } 

                            }
                        }
                    }
                    _ => {
                        n_cells_known += 1;
                        continue;
                    }
                };
                let elem = matrix.at_mut(i, j);
                elem.status = new_status;
            }
        }

        n_cells_known = count_cells_known(matrix);

    }


}

fn fill_inside_cells(matrix: &mut Matrix<Pipe>) {
    let rows = matrix.rows;
    let cols = matrix.cols;
    let mut n_cells_known =100;
    let mut prev_n_cells_known = 0;
    while prev_n_cells_known < n_cells_known {
        prev_n_cells_known = count_cells_known(matrix);
        for i in 0..matrix.rows {
            for j in 0..matrix.cols {
                let elem = matrix.at_mut(i, j);
                let mut new_status = elem.status;
                match elem.status {
                    Status::Pipe => {
                        let steps = vec![-1i32, 0i32, 1i32];
                        for inc_i in &steps {
                            for inc_j in &steps {
                                let next_i = i as i32 + *inc_i;
                                let next_j = j as i32 + *inc_j;
                                if next_i >= rows as i32
                                    || next_j >=cols as i32
                                    || next_i < 0
                                    || next_j < 0
                                {
                                    new_status = Status::Outside;
                                    continue;
                                }
                                if (inc_i.abs() + inc_j.abs()) == 0
                                {
                                    // if its itself
                                    continue;
                                }

                                let new_elem = matrix.at_mut(next_i as u32, next_j as u32);
                                if new_elem.status == Status::Outside{
                                    new_status = Status::Outside;

                                } 

                            }
                        }
                    }
                    _ => {
                        n_cells_known += 1;
                        continue;
                    }
                };
                let elem = matrix.at_mut(i, j);
                elem.status = new_status;
            }
        }

        n_cells_known = count_cells_known(matrix);

    }


}


fn search_inside_and_outside_cells(matrix: &mut Matrix<Pipe>) -> u32 {
    fill_outside_cells(matrix);
    // fill_inside_cells(matrix);
    count_cells_unknown(matrix)
}

fn extend_matrix(matrix:&mut Matrix<Pipe>)->Matrix<Pipe>{
    let _ = find_start(matrix);
    let n_rows_orig= matrix.rows;
    let n_cols_orig= matrix.cols;
    let n_rows= matrix.rows*2;
    let n_cols= matrix.cols*2;
    let mut new_matrix:Matrix<Pipe> = Matrix::new(n_rows, n_cols, Pipe{pipe_type:PipeType::None,char:'.',start:Directions::None,end:Directions::None,status: Status::Unknown});
    for i in 0..n_rows_orig{
    for j in 0..n_cols_orig{
        let new_index_i = i*2;
        let new_index_j = j*2;

        let elem = matrix.at(i, j).clone();
        let e00 = new_matrix.at_mut(new_index_i, new_index_j);
        *e00 = elem ;
        let e01_content = match elem.pipe_type {
            PipeType::Horizontal|PipeType::NorthEast|PipeType::SouthEast => {
                Pipe{pipe_type:PipeType::Horizontal,char:'-',start:Directions::None,end:Directions::None,status: Status::Unknown}
            },
            PipeType::Start => {
                if elem.end == Directions::East{
                Pipe{pipe_type:PipeType::Horizontal,char:'-',start:Directions::None,end:Directions::None,status: Status::Unknown}
                }else{
                Pipe{pipe_type:PipeType::None,char:'.',start:Directions::None,end:Directions::None,status: Status::Unknown}
                }

            },
            _ => { Pipe{pipe_type:PipeType::None,char:'.',start:Directions::None,end:Directions::None,status: Status::Unknown}}
        };
        let e01 = new_matrix.at_mut(new_index_i, new_index_j+1);
        *e01 = e01_content;

        let e10_content = match elem.pipe_type {
            PipeType::Vertical|PipeType::SouthWest|PipeType::SouthEast => {
                Pipe{pipe_type:PipeType::Vertical,char:'|',start:Directions::None,end:Directions::None,status: Status::Unknown}
            },
            PipeType::Start => {
                if elem.end == Directions::South{
                Pipe{pipe_type:PipeType::Vertical,char:'|',start:Directions::None,end:Directions::None,status: Status::Unknown}
                }else{
                Pipe{pipe_type:PipeType::None,char:'.',start:Directions::None,end:Directions::None,status: Status::Unknown}
                }

            },
            _ => { Pipe{pipe_type:PipeType::None,char:'.',start:Directions::None,end:Directions::None,status: Status::Unknown}}
        };
        let e10 = new_matrix.at_mut(new_index_i+1, new_index_j);
        *e10 = e10_content;

        let e11_content = match e10_content.pipe_type{
            _ => { Pipe{pipe_type:PipeType::None,char:'.',start:Directions::None,end:Directions::None,status: Status::Unknown}}
        };
        let e11 = new_matrix.at_mut(new_index_i+1, new_index_j+1);
        *e11 = e11_content;


    }
    }
    new_matrix

}
fn print_matrix(matrix:&Matrix<Pipe>){
    for i in 0..matrix.rows{
    for j in 0..matrix.cols{
        let elem  = matrix.at(i,j);
            match elem.status {
                Status::Pipe =>{
                    print!("{color_red}{}{color_reset}",elem.char);
                },
                Status::Outside => {
                    print!("O");
                },
                Status::Inside=>{()},
                Status::Unknown=>{
                    print!("{color_blue}I{color_reset}")
                },
            };
        }
    println!();
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
    let mut matrix = parse_content(&content);
    let mut matrix = extend_matrix(&mut matrix);

    let result = traverse_pipes(&mut matrix);
    print_matrix(&matrix);
    println!("Part 1: N_length {}", result);
    let result = search_inside_and_outside_cells(&mut matrix);
    print_matrix(&matrix);
    println!("Part 2: N_inside_cells {}", result);

}
