use std::fmt;
use std::ops::{Index, IndexMut};

pub struct Matrix<T> {
    pub data: Vec<T>,
    pub rows: u32,
    pub cols: u32,
}

impl<T: Clone + fmt::Display> Matrix<T> {
    pub fn new(rows: u32, cols: u32, value: T) -> Matrix<T> {
        let data = vec![value; (rows * cols) as usize];
        Matrix { data, rows, cols }
    }
    pub fn at(&self, i: u32, j: u32) -> &T {
        let index = i * self.cols + j;
        &self.data[index as usize]
    }
    pub fn at_mut(&mut self, i: u32, j: u32) -> &mut T {
        let index = i * self.cols + j;
        &mut self.data[index as usize]
    }
}

impl Matrix<char> {
    pub fn new_from_str(str: &str) -> Matrix<char> {
        let mut n_rows: u32 = 0;
        let mut n_cols: u32 = 0;
        let mut data: Vec<char> = Vec::new();

        for elems in str.lines() {
            n_rows += 1;
            // let elems: Vec<char> = line
            //     .iter()
            //     .filter(|x| !x.is_empty())
            //     .map(|x| x.parse::<u32>().unwrap())
            //     .collect();
            n_cols = elems.len() as u32;
            for elem in elems.chars() {
                data.push(elem);
            }
        }

        Matrix {
            data,
            rows: n_rows,
            cols: n_cols,
        }
    }
}

impl<T: Clone + fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n")?;
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{}", *self.at(i, j))?;
            }
            write!(f, "\n")?;
        }
        std::fmt::Result::Ok(())
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Directions {
    North,
    South,
    East,
    West,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PipeType {
    Horizontal,
    Vertical,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Start,
    None,
}

impl PipeType {
    pub fn from_char(c: char) -> PipeType {
        match c {
            '-' => PipeType::Horizontal,
            '|' => PipeType::Vertical,
            'L' => PipeType::NorthEast,
            'J' => PipeType::NorthWest,
            '7' => PipeType::SouthWest,
            'F' => PipeType::SouthEast,
            'S' => PipeType::Start,
            _ => PipeType::None,
        }
    }
}

#[derive(Debug, Clone, Copy,PartialEq)]
pub enum Status {
    Unknown,
    Outside,
    Inside,
    Pipe,
}

#[derive(Debug, Clone, Copy)]
pub struct Pipe {
    pub pipe_type: PipeType,
    pub char: char,
    pub start: Directions,
    pub end: Directions,
    pub status: Status,
}

impl fmt::Display for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?}\n", self.char, self)?;
        std::fmt::Result::Ok(())
    }
}

impl Matrix<Pipe> {
    pub fn new_from_pipe(str: &str) -> Matrix<Pipe> {
        let mut n_rows: u32 = 0;
        let mut n_cols: u32 = 0;
        let mut data: Vec<Pipe> = Vec::new();

        for elems in str.lines() {
            n_rows += 1;
            n_cols = elems.len() as u32;
            for elem in elems.chars() {
                data.push(Pipe {
                    char: elem,
                    pipe_type: PipeType::from_char(elem),
                    start: Directions::None,
                    end: Directions::None,
                    status: Status::Unknown,
                });
            }
        }

        Matrix {
            data,
            rows: n_rows,
            cols: n_cols,
        }
    }
}
