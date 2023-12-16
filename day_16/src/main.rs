use std::env;
use std::fs;
use std::{time,thread};
use std::process::exit;
use std::collections::HashMap;

use day_16::*;


#[derive (Debug,Copy,Clone,Hash,PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}


#[derive (Debug,Copy,Clone,Hash,PartialEq, Eq)]
struct Laser{
    i: i64,
    j: i64,
    direction : Direction,
}




fn move_laser(laser: &Laser, _map:&Matrix<char>, energized_matrix:&mut Matrix<usize>)->Vec<Laser>{
    let (i,j) = match laser.direction {
        Direction::Up => (laser.i as i64 -1 ,laser.j as i64),
        Direction::Down=> (laser.i as i64 +1,laser.j as i64),
        Direction::Left=> (laser.i as i64 ,laser.j as i64 -1),
        Direction::Right=>(laser.i as i64 ,laser.j as i64 +1),
    };
    if i < 0 || j < 0 || i >= _map.rows as i64 || j >= _map.cols as i64{
        // println!("laser ends");
        return Vec::new();
    }
    *energized_matrix.at_mut(i as usize, j as usize)+=1;

    let elem = _map.at(i as usize,j as usize);
    let vec = match elem {
        '.' => vec![Laser{i,j,direction : laser.direction}] ,
        '\\' => match laser.direction {
                Direction::Up => vec![Laser{i,j,direction : Direction::Left}],
                Direction::Down=>vec![Laser{i,j,direction : Direction::Right}],
                Direction::Left=>vec![Laser{i,j,direction : Direction::Up}],
                Direction::Right=>vec![Laser{i,j,direction : Direction::Down}],
            },
        '/' => match laser.direction {
                Direction::Up => vec![Laser{i,j,direction : Direction::Right}],
                Direction::Down=>vec![Laser{i,j,direction : Direction::Left}],
                Direction::Left=>vec![Laser{i,j,direction : Direction::Down}],
                Direction::Right=>vec![Laser{i,j,direction : Direction::Up}],
            },
        '|' => match laser.direction {
                Direction::Up | Direction::Down=>vec![Laser{i,j,direction : laser.direction}],
                Direction::Left | Direction::Right=>vec![Laser{i,j,direction : Direction::Up},Laser{i,j,direction : Direction::Down}],
            },
        '-' => match laser.direction {
                Direction::Left | Direction::Right=>vec![Laser{i,j,direction : laser.direction}],
                Direction::Up | Direction::Down=>vec![Laser{i,j,direction : Direction::Left},Laser{i,j,direction : Direction::Right}],
            },
        _ => panic!("Character not expected"),
    };

   vec
}

fn energize_map(cave_map:&Matrix<char>,start:Laser)->Matrix<usize>{
    let mut energized_map = Matrix::new(cave_map.rows,cave_map.cols, 0);
    let mut laser_beams = vec![start];
    let mut n_steps = 0;
    let mut laser_hash_map = HashMap::new();
    while !laser_beams.is_empty(){
        n_steps+=1;
        // println!("n_steps : {n_steps}");
        let mut new_lasers_beams = Vec::new();
        for laser in laser_beams{
            let new_lasers = move_laser(&laser, cave_map, &mut energized_map);
            for try_laser  in &new_lasers {
                // println!("try_laser {:?}",try_laser);
                if laser_hash_map.get(try_laser).is_none(){
                    laser_hash_map.insert(try_laser.clone(),true);
                    new_lasers_beams.push(try_laser.clone())
                }
            }
            // new_lasers_beams.extend_from_slice(&new_lasers);
        }
        laser_beams = new_lasers_beams;
        // println!("{}",energized_map);
        // let ten_millis = time::Duration::from_millis(100);
        // thread::sleep(ten_millis);
    }

    energized_map

}

fn count_energized(energized_matrix:&Matrix<usize>)->u64{
    energized_matrix.data.iter().filter(|x| **x>0).fold(0, |acc,_x| acc+1)
}

// fn test_multiple_options(cave_map:&Matrix<char>)->u64{

// }

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("Filename not provided");
        exit(0);
    }

    let filename: &str = args[1].as_str();
    let content = fs::read_to_string(filename).expect("Error reading the file");
    let matrix = Matrix::new_from_str(&content);
    let energize_map = energize_map(&matrix, Laser{i:0,j : -1,direction : Direction::Right});
    // println!("{}",energize_map);
    println!("PART 1: {}",count_energized(&energize_map));


}
